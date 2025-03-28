use lambda_runtime::{Error, LambdaEvent};
use aws_lambda_events::event::apigw::{
    ApiGatewayV2httpRequest, ApiGatewayV2httpResponse
};
use http::header::{HeaderMap, HeaderValue};
use aws_lambda_events::encodings::Body;
use crate::http_page::{
    get_error_page, redirect_url,get_server_error, session_out,
};
use crate::libs::MainPageParams;
use crate::intro_console::main_console_page;
use crate::oidc_request::{
    TokenResponse, get_token_accesss, discover_endpoints,
};
use crate::oidc_database::{
    SessionData, get_session_data, save_to_dynamo,
};
use lambda_runtime::tracing::{error, info};
use rand::Rng;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use url::Url;
use std::env;

// Constant for the session length
const SESSION_LENGTH: i64 = 1 * 60 * 60 * 1000; // 1 hour

pub(crate) async fn function_handler(
    event: LambdaEvent<ApiGatewayV2httpRequest>,
) -> Result<ApiGatewayV2httpResponse, Error> {
    info!("Event: {:?}", event);
    let request = event.payload;
    
    // Access request_context
    let request_context = &request.request_context;

    // Access query_string_parameters - this is a QueryMap which is a wrapper around a HashMap
    let query_params = &request.query_string_parameters;
    info!("Query string parameters: {:?}", query_params);
    
    // Get table name
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");

    // Create headers to response
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));
    headers.insert("X-Custom-Header", HeaderValue::from_static("custom-value"));

    // Create cookies
    let mut cookies = vec![
        "session=abc123; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=3600".to_string(),
        "lang=en; Path=/; Max-Age=604800".to_string(),
    ];

    // Get Smart App callback
    let scope = "meldrx-api cds profile openid launch patient/*.*";
    
    // Extract domain name
    let domain_name = request_context.domain_name
        .as_deref()
        .unwrap_or("No domain name");
    
    let redirect_uri = format!("https://{}/callback", domain_name);

    // Extract the time epoch (timestamp)
    let actual_time_epoch = request_context.time_epoch;

    // Extract route_key from the request context    
    let route_key = request_context.route_key
        .as_deref()
        .unwrap_or("No route key");

    match route_key {
        // ~~~~~~~~~~~~~~~~~~~~ LAUNCH ~~~~~~~~~~~~~~~~~~~~~~~~~~
        "GET /launch" => {
            info!("Route key: {}", route_key);
            // Get the IssuerUrl and Launch
            let launch = query_params.first("launch").unwrap_or_default();
            let client_id = query_params.first("client").unwrap_or_default();
            let issuer = query_params.first("iss").unwrap_or_default();

            let state: String = generate_random_state(16);

            // Discover auth endpoints
            let (auth_endpoint, token_endpoint) = match discover_endpoints(issuer).await {
                Ok(endpoints) => endpoints,
                Err(e) => {
                    error!("Error getting auth endpoints: {} [E463]", e);
                    let message = get_error_page("E463");
                    let body = Body::Text(message);
                    return Ok(ApiGatewayV2httpResponse {
                        status_code: 463,
                        headers: headers,
                        multi_value_headers: HeaderMap::new(),
                        body: Some(body),
                        cookies: cookies,
                        is_base64_encoded: false}
                    );
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
                client_id: client_id.to_string().into(),
                code_verifier: code_verifier.into(),
                code_challenge: code_challenge.clone().into(),
                code_challenge_method: Some("S256".to_string()),
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
            let message = redirect_url(&link);
            let body = Body::Text(message);
            return Ok(ApiGatewayV2httpResponse {
                status_code: 200,
                headers: headers,
                multi_value_headers: HeaderMap::new(),
                body: Some(body),
                cookies: cookies,
                is_base64_encoded: false}
            );
        }
        // ~~~~~~~~~~~~~~~~~~~~ CALLBACK ~~~~~~~~~~~~~~~~~~~~~~~~~~
        "ANY /callback" => {
            info!("Route key: {}", route_key);
            // Extract parameters
            let code = query_params.first("code").unwrap_or_default();
            let session_state = query_params.first("session_state").unwrap_or_default();
            let state = query_params.first("state").unwrap_or_default();         

            cookies.push(format!("session={state}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=3600"));
            let mut issuer = String::new();
            let mut client_id = String::new();
            let mut code_verifier = String::new();
            let mut code_challenge = String::new();
            let mut code_challenge_method = String::new();
            let mut auth_endpoint = String::new();
            let mut token_endpoint = String::new();
            let mut session_timeout: i64 = 0;

            match get_session_data(
                state, 
                &table_name
            ).await {
                Ok(Some(sd)) => {
                    if let Some(av) = sd.iss { issuer = av.clone(); }
                    if let Some(av) = sd.client_id { client_id = av.clone(); }
                    if let Some(av) = sd.code_challenge { code_challenge = av.clone(); }
                    if let Some(av) = sd.code_challenge_method { code_challenge_method = av.clone(); }
                    if let Some(av) = sd.code_verifier { code_verifier = av.clone(); }
                    if let Some(av) = sd.auth_endpoint { auth_endpoint = av.clone(); }
                    if let Some(av) = sd.token_endpoint { token_endpoint = av.clone(); }
                    if let Some(av) = sd.session_timeout { session_timeout = av; }
                },
                Ok(None) => error!("No session data found [E311]"),
                Err(e) => error!("Error retrieving session data: {:?} [E312]", e),
            }

            // ~~~~~~~~~~~~~~~~~~~~ TOKEN ~~~~~~~~~~~~~~~~~~~~~~~~~~     
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
                    error!("Error getting token: {} [E471]", e);
                    let message = get_server_error("E471");
                    let body = Body::Text(message);
                    return Ok(ApiGatewayV2httpResponse {
                        status_code: 471,
                        headers: headers,
                        multi_value_headers: HeaderMap::new(),
                        body: Some(body),
                        cookies: cookies,
                        is_base64_encoded: false}
                    );
                }
            };

            let token = token_resp.access_token.clone();
            let patient_id = token_resp.patient.clone().unwrap_or_default();

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
                code_challenge_method: Some(code_challenge_method.clone()),
                auth_endpoint: Some(auth_endpoint.clone()),
                token_endpoint: Some(token_endpoint.clone()),
                iss: Some(issuer.clone()),
                session_timeout: Some(session_timeout),
                patient: Some(patient_id.clone()),
            };
        
            match save_to_dynamo(
                &session_data,                  
                &table_name
            ).await {
                Ok(_) => info!("Session data saved to Dynamo successfully"),
                Err(e) => error!("Error saving session data to Dynamo: {:?} [E332]", e),
            }

            let app_url = format!("https://{}/scrabapp", domain_name);

            // Parse the base endpoint URL
            let base_url = Url::parse(&app_url)?;

            // Create a mutable URL for building the query
            let mut url = base_url.clone();

            // Add all query parameters
            url.query_pairs_mut()
                .append_pair("state", &state)
                .append_pair("code_challenge", &code_challenge)
                .append_pair("code_challenge_method", &code_challenge_method);

            // Convert Url to string
            let link = url.to_string();
            let message = redirect_url(&link);
            let body = Body::Text(message);
            return Ok(ApiGatewayV2httpResponse {
                status_code: 200,
                headers: headers,
                multi_value_headers: HeaderMap::new(),
                body: Some(body),
                cookies: cookies,
                is_base64_encoded: false}
            );
        }
        // ~~~~~~~~~~~~~~~~~~~~ SCRAB APP ~~~~~~~~~~~~~~~~~~~~~~~~~~
        "ANY /scrabapp" => {
            info!("Route key: {}", route_key);

            // Extract parameters
            let state = query_params.first("state").unwrap_or_default();        

            // Set mutable variables
            let mut issuer = String::new();
            let mut token = String::new();
            let mut patient_id = String::new();
            let mut session_timeout: i64 = 0;

            match get_session_data(
                state, 
                &table_name
            ).await {
                Ok(Some(sd)) => {
                    if let Some(av) = sd.iss { issuer = av.clone(); }
                    if let Some(av) = sd.access_token { token = av.clone(); }
                    if let Some(av) = sd.patient { patient_id = av.clone(); }
                    if let Some(av) = sd.session_timeout { session_timeout = av; }
                },
                Ok(None) => error!("No session data found [E711]"),
                Err(e) => error!("Error retrieving session data: {:?} [E712]", e),
            }

            if session_timeout != 0 && session_timeout < actual_time_epoch {
                error!("Session timeout [E713]");
                let message = session_out("E713");
                let body = Body::Text(message);
                return Ok(ApiGatewayV2httpResponse {
                    status_code: 713,
                    headers: headers,
                    multi_value_headers: HeaderMap::new(),
                    body: Some(body),
                    cookies: cookies,
                    is_base64_encoded: false}
                );
            }
    
            let mpage_params = MainPageParams {
                iss: issuer.clone(),
                access_token: token.clone(),
                patient_id: patient_id.clone(),
            };

            let message = match main_console_page(&mpage_params).await {
                Ok(message) => message,
                Err(e) => {
                    error!("Error getting main page: {} [E495]", e);
                    get_server_error("E495")
                }
            };
            let body = Body::Text(message);
            return Ok(ApiGatewayV2httpResponse {
                status_code: 200,
                headers: headers,
                multi_value_headers: HeaderMap::new(),
                body: Some(body),
                cookies: cookies,
                is_base64_encoded: false}
            );
        }
        _ => {
            error!("Route key not found: {} [E341]", route_key);
        }
    }

    let message = get_server_error("E495");
    let body = Body::Text(message);
    let resp = ApiGatewayV2httpResponse {
        status_code: 495,
        headers: headers,
        multi_value_headers: HeaderMap::new(),
        body: Some(body),
        cookies: cookies,
        is_base64_encoded: false,
    };
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