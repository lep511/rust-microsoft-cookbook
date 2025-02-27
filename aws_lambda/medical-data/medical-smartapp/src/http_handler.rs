use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_http::request::RequestContext;
use crate::http_page::{get_http_page, get_connect_page, get_error_page};
use crate::oidc_request::{
    TokenResponse, get_token_accesss, get_param_endpoint, 
};
use crate::oidc_database::{SessionData, get_session_data, save_to_dynamo};
use lambda_http::tracing::{error, info};
use rand::Rng;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use url::Url;
use std::env;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("Event: {:?}", event);

    let req_ext = RequestExt::request_context(&event);
    info!("Request context: {:?}", req_ext);

    let params = event.query_string_parameters();
    info!("Query string parameters: {:?}", params);
    
    // Get table name from environment variables
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");

    // Get Smart App callback
    let scope = "meldrx-api cds profile openid launch patient/*.*";
    
    let request = event.request_context();

    // Extract domain name
    let domain_name = match &request {
        RequestContext::ApiGatewayV2(ctx) => {
            ctx.domain_name.clone().unwrap_or_else(|| "unknown".to_string())
        },
        _ => "unknown".to_string(),
    };
    let redirect_uri = format!("https://{}/callback", domain_name);

    // Extract route_key from the request context    
    let route_key = match &request {
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
            let iss = params.first("iss").unwrap_or_default();
            let launch = params.first("launch").unwrap_or_default();
            let client_id = params.first("client").unwrap_or_default();
                        
            let auth_endpoint = match get_param_endpoint(
                iss, 
                "authorization_endpoint",
            ).await {
                Ok(auth_endpoint) => auth_endpoint,
                Err(e) => {
                    let message = get_error_page("E103");
                    error!("Error getting auth endpoint: {}", e);
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

            // Generate a random alphanumeric string
            let state = generate_random_state(16);

            // Add all query parameters
            url.query_pairs_mut()
                .append_pair("response_type", "code")
                .append_pair("client_id", client_id)
                .append_pair("scope", scope)
                .append_pair("redirect_uri", &redirect_uri)
                .append_pair("code_challenge", &code_challenge)
                .append_pair("launch", launch)
                .append_pair("aud", iss)
                .append_pair("state", &state)
                .append_pair("code_challenge_method", "S256");

            // Save state to DynamoDB
            let state_data = SessionData {
                pk: state.clone(),
                access_token: None,
                expires_in: None,
                scope: None,
                token_type: None,
                id_token: None,
                session_state: None,
                client_id: Some(client_id.to_string()),
                code_verifier: Some(code_verifier.clone()),
                code_challenge: Some(code_challenge.clone()),
                iss: Some(iss.to_string()),
            };
        
            match save_to_dynamo(
                &state_data,                  
                &table_name
            ).await {
                Ok(_) => info!("Session data saved to Dynamo successfully"),
                Err(e) => error!("Error saving session data to Dynamo: {:?}", e),
            }

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
            let state = params.first("state").unwrap_or_default();

            if state.is_empty() {
                error!("State not found in parameters");
                let message = get_error_page("E102");
                return Ok(Response::builder()
                    .status(404)
                    .header("content-type", "text/html")
                    .body(message.into())
                    .map_err(Box::new)?);
            }

            let mut iss = String::new();
            let mut token = String::new();
            let mut client_id = String::new();
            let mut code_verifier = String::new();
            let mut code_challenge = String::new();

            match get_session_data(
                state, 
                &table_name
            ).await {
                Ok(Some(session_data)) => {
                    if let Some(acc_token) = session_data.access_token {
                        token = acc_token.clone();
                    }
                    if let Some(acc_iss) = session_data.iss {
                        iss = acc_iss.clone();
                    }
                    if let Some(acc_client_id) = session_data.client_id {
                        client_id = acc_client_id.clone();
                    }
                    if let Some(acc_code_challenge) = session_data.code_challenge {
                        code_challenge = acc_code_challenge.clone();
                    }
                    if let Some(acc_code_verifier) = session_data.code_verifier {
                        code_verifier = acc_code_verifier.clone();
                    }
                },
                Ok(None) => error!("No session data found"),
                Err(e) => error!("Error retrieving session data: {:?}", e),
            }

            // Verify the code verifier matches the challenge
            if verify_code_challenge(&code_verifier, &code_challenge) {
                info!("Server: Verification successful!");
            } else {
                error!("Server: Verification failed!");
            }

            if token.is_empty() {
                info!("Token not found, getting new token");
                let token_endpoint = match get_param_endpoint(
                    &iss, 
                    "token_endpoint",
                ).await {
                    Ok(t_endpoint) => t_endpoint,
                    Err(e) => {
                        let message = get_error_page("E109");
                        error!("Error getting token endpoint: {}", e);
                        return Ok(Response::builder()
                            .status(404)
                            .header("content-type", "text/html")
                            .body(message.into())
                            .map_err(Box::new)?);
                    }
                };
                
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
                        error!("Error getting token: {}", e);
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

                info!("Patient: {}", patient);

                let session_data = SessionData {
                    pk: state.to_string(),
                    access_token: Some(token.clone()),
                    expires_in: token_resp.expires_in,
                    scope: Some(scope.to_string()),
                    token_type: token_resp.token_type,
                    id_token: token_resp.id_token,
                    session_state: Some(session_state.to_string()),
                    client_id: Some(client_id.clone()),
                    code_verifier: Some(code_verifier.clone()),
                    code_challenge: Some(code_challenge.clone()),
                    iss: Some(iss.clone()),
                };
            
                match save_to_dynamo(
                    &session_data,                  
                    &table_name
                ).await {
                    Ok(_) => info!("Session data saved to Dynamo successfully"),
                    Err(e) => error!("Error saving session data to Dynamo: {:?}", e),
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
            error!("Route key not found: {}", route_key);
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

/// Verify that a code verifier matches a previously created challenge
fn verify_code_challenge(
    code_verifier: &str, 
    expected_challenge: &str
) -> bool {
    let calculated_challenge = generate_code_challenge(code_verifier);
    calculated_challenge == expected_challenge
}