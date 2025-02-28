use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_http::request::RequestContext;
use crate::http_page::{get_http_page, get_connect_page, get_error_page};
use crate::oidc_request::{
    TokenResponse, get_token_accesss, discover_endpoints, 
};
use crate::oidc_database::{
    SessionData, get_session_data, save_to_dynamo, get_client_data,
    remove_session_data,
};
use lambda_http::tracing::{error, info};
use rand::Rng;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use url::Url;
use std::env;

// Constant for the session length
const SESSION_LENGTH: i64 = 6 * 60 * 60 * 1000; // 6 hours

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("Event: {:?}", event);
    let request_cont = event.request_context();
    let params = event.query_string_parameters();
    info!("Query string parameters: {:?}", params);
    
    // Get table name
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");

    // Get index name
    let index_name = env::var("INDEX_NAME").expect("INDEX_NAME must be set");

    // Get Smart App callback
    let scope = "meldrx-api cds profile openid launch patient/*.*";
    
    // Extract domain name
    let domain_name = match &request_cont {
        RequestContext::ApiGatewayV2(ctx) => {
            ctx.domain_name.clone().unwrap_or_else(|| "unknown".to_string())
        },
        _ => "unknown".to_string(),
    };
    let redirect_uri = format!("https://{}/callback", domain_name);

    // Extract actual time epoch
    let actual_time_epoch = match &request_cont {
        RequestContext::ApiGatewayV2(ctx) => { ctx.time_epoch },
        _ => 0,
    };

    // Extract route_key from the request context    
    let route_key = match &request_cont {
        RequestContext::ApiGatewayV2(ctx) => {
            ctx.route_key.clone().unwrap_or_else(|| "unknown".to_string())
        },
        _ => "unknown".to_string(),
    };

    match route_key.as_str() {
        // ~~~~~~~~~~~~~~~~~~~~ LAUNCH ~~~~~~~~~~~~~~~~~~~~~~~~~~
        "GET /launch" => {
            info!("Route key: {}", route_key);
            // Get the IssuerUrl and Launch
            let issuer = params.first("iss").unwrap_or_default();
            let launch = params.first("launch").unwrap_or_default();
            let client_id = params.first("client").unwrap_or_default();

            let mut state: String = generate_random_state(16);
            let mut session_timeout: i64 = 0;
            let mut authorized: bool = false;

            // Checking if client_id exists in the database
            match get_client_data(
                &client_id, 
                &table_name,
                &index_name
            ).await {
                Ok(Some(sd)) => {
                    state = sd.state.clone();
                    session_timeout = sd.session_timeout.clone();
                    authorized = sd.authorized;
                },
                Ok(None) => info!("No client data found."),
                Err(e) => error!("Error retrieving client data: {:?}[E301]", e),
            }

            info!("Auth: {}", authorized);

            if session_timeout != 0 && authorized {
                if actual_time_epoch < session_timeout {
                    info!("Session is still valid");
                    let message = get_http_page();
                    return Ok(Response::builder()
                        .status(200)
                        .header("content-type", "text/html")
                        .body(message.into())
                        .map_err(Box::new)?);
                } else {
                    // Remove actual session data
                    info!("State: {}", state);
                    match remove_session_data(
                        &table_name,
                        "pk",
                        &state,
                    ).await {
                        Ok(_) => info!("Session has expired. Session data removed successfully"),
                        Err(e) => error!("Session has expired. Error removing session data: {:?} [E302]", e),
                    }
                }
            }

            // Discover auth endpoints
            let (auth_endpoint, token_endpoint) = match discover_endpoints(issuer).await {
                Ok(endpoints) => endpoints,
                Err(e) => {
                    let message = get_error_page("E103");
                    error!("Error getting auth endpoints: {} [E103]", e);
                    return Ok(Response::builder()
                        .status(404)
                        .header("content-type", "text/html")
                        .body(message.into())
                        .map_err(Box::new)?);
                }
            };

            // Generate the CodeVerifier and CodeChallenge
            let code_verifier = generate_code_verifier();
            let code_challenge = generate_code_challenge(&code_verifier);

            // Parse the base endpoint URL
            let base_url = Url::parse(&auth_endpoint)?;

            // Create a mutable URL for building the query
            let mut url = base_url.clone();

            // Save state to DynamoDB
            let state_data = SessionData {
                pk: state.clone(),
                authorized: false,
                client_id: client_id.to_string().into(),
                code_verifier: code_verifier.into(),
                code_challenge: code_challenge.clone().into(),
                auth_endpoint: auth_endpoint.into(),
                token_endpoint: token_endpoint.into(),
                iss: issuer.to_string().into(),
                session_timeout: (actual_time_epoch + SESSION_LENGTH).into(),
                ..Default::default()
            };

            match save_to_dynamo(
                &state_data,                  
                &table_name
            ).await {
                Ok(_) => info!("Session data saved to Dynamo successfully"),
                Err(e) => error!("Error saving session data to Dynamo: {:?} [E303]", e),
            }

            // Add all query parameters
            url.query_pairs_mut()
                .append_pair("response_type", "code")
                .append_pair("client_id", client_id)
                .append_pair("scope", scope)
                .append_pair("redirect_uri", &redirect_uri)
                .append_pair("code_challenge", &code_challenge)
                .append_pair("launch", launch)
                .append_pair("aud", issuer)
                .append_pair("state", &state)
                .append_pair("code_challenge_method", "S256");
        
            // Convert Url to string
            let link = url.to_string();
            let message = get_connect_page(&link);

            return Ok(Response::builder()
                .status(200)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?);
        }
        // ~~~~~~~~~~~~~~~~~~~~ CALLBACK ~~~~~~~~~~~~~~~~~~~~~~~~~~
        "ANY /callback" => {
            info!("Route key: {}", route_key);
            // Extract parameters
            let code = params.first("code").unwrap_or_default();
            let session_state = params.first("session_state").unwrap_or_default();
            let state = match params.first("state") {
                Some(state) if !state.is_empty() => state,
                _ => {
                    error!("State not found in parameters [E102]");
                    let message = get_error_page("E102");
                    return Ok(Response::builder()
                        .status(404)
                        .header("content-type", "text/html")
                        .body(message.into())
                        .map_err(Box::new)?);
                }
            };           

            let mut issuer = String::new();
            let mut token = String::new();
            let mut client_id = String::new();
            let mut code_verifier = String::new();
            let mut code_challenge = String::new();
            let mut auth_endpoint = String::new();
            let mut token_endpoint = String::new();
            let mut session_timeout: i64 = 0;

            match get_session_data(
                state, 
                &table_name
            ).await {
                Ok(Some(sd)) => {
                    if let Some(av) = sd.access_token { token = av.clone(); }
                    if let Some(av) = sd.iss { issuer = av.clone(); }
                    if let Some(av) = sd.client_id { client_id = av.clone(); }
                    if let Some(av) = sd.code_challenge { code_challenge = av.clone(); }
                    if let Some(av) = sd.code_verifier { code_verifier = av.clone(); }
                    if let Some(av) = sd.auth_endpoint { auth_endpoint = av.clone(); }
                    if let Some(av) = sd.token_endpoint { token_endpoint = av.clone(); }
                    if let Some(av) = sd.session_timeout { session_timeout = av; }
                },
                Ok(None) => error!("No session data found [E311]"),
                Err(e) => error!("Error retrieving session data: {:?} [E312]", e),
            }

            if token.is_empty() {
                info!("Token not found, getting new token");               
                let token_resp: TokenResponse = match get_token_accesss(
                    &client_id,
                    &token_endpoint,
                    code, 
                    &code_verifier,
                    &redirect_uri,
                    &scope,
                ).await {
                    Ok(token) => token,
                    Err(e) => {
                        error!("Error getting token: {} [E331]", e);
                        let message = get_error_page("E105");
                        return Ok(Response::builder()
                            .status(404)
                            .header("content-type", "text/html")
                            .body(message.into())
                            .map_err(Box::new)?);
                    }
                };

                token = token_resp.access_token.clone();
                let patient = token_resp.patient.clone().unwrap_or_default();

                let session_data = SessionData {
                    pk: state.to_string(),
                    authorized: true,
                    access_token: Some(token.clone()),
                    expires_in: token_resp.expires_in,
                    scope: Some(scope.to_string()),
                    token_type: token_resp.token_type,
                    id_token: token_resp.id_token,
                    session_state: Some(session_state.to_string()),
                    client_id: Some(client_id.clone()),
                    code_verifier: Some(code_verifier.clone()),
                    code_challenge: Some(code_challenge.clone()),
                    auth_endpoint: Some(auth_endpoint.clone()),
                    token_endpoint: Some(token_endpoint.clone()),
                    iss: Some(issuer.clone()),
                    session_timeout: Some(session_timeout),
                    patient: Some(patient.clone()),
                };
            
                match save_to_dynamo(
                    &session_data,                  
                    &table_name
                ).await {
                    Ok(_) => info!("Session data saved to Dynamo successfully"),
                    Err(e) => error!("Error saving session data to Dynamo: {:?} [E332]", e),
                }
            }
    
            let message = get_http_page();
            return Ok(Response::builder()
                .status(200)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?);
        }
        // ~~~~~~~~~~~~~~~~~~~~ TASKS ~~~~~~~~~~~~~~~~~~~~~~~~~~
        "GET /tasks" => {
            info!("Route key: {}", route_key);
        }
        _ => {
            error!("Route key not found: {} [E341]", route_key);
        }
    }

    let message = get_error_page("E909");
    let resp = Response::builder()
        .status(404)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

fn generate_random_state(length: usize) -> String {
    const CHARSET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

// Generate a random code verifier
fn generate_code_verifier() -> String {
    let mut rng = rand::rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.random()).collect(); // 32 bytes = 43 chars after encoding
    URL_SAFE_NO_PAD.encode(&random_bytes)
}

/// Create the code challenge from the verifier using S256 method
fn generate_code_challenge(code_verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(&hash)
}

// Verify that a code verifier matches a previously created challenge
// fn verify_code_challenge(
//     code_verifier: &str, 
//     expected_challenge: &str
// ) -> bool {
//     let calculated_challenge = generate_code_challenge(code_verifier);
//     calculated_challenge == expected_challenge
// }