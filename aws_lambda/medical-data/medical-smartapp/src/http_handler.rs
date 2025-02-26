use lambda_http::{Body, Error, Request, RequestExt, Response};
use crate::http_page::{get_http_page, get_connect_page, get_error_page};
use crate::oidc_request::{
    TokenResponse, get_token_accesss, get_param_endpoint, 
};
use crate::oidc_database::{SessionData, get_session_token, save_session_token};
use lambda_http::tracing::{error, info};
use url::Url;
use std::env;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("Event: {:?}", event);

    let request_context = RequestExt::request_context(&event);
    info!("Request context: {:?}", request_context);

    let params = event.query_string_parameters();
    info!("Query string parameters: {:?}", params);
    
    // Get environment variables
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");

    // Get Smart App callback
    let url_str = event.uri().to_string();
    let scope = "meldrx-api cds profile openid launch patient/*.*";
    let code_verifier = "Q4XqM0pPdsNHwhdEpt6eVAil7djAzhf6zMRAmbb8d-4".to_string();
    let code_challenge = "scOFvF4mB7t-R5egnefSgn0W_hL4HAzYKG-zDs_mWgM".to_string();
    
    let (resource, version) = match extract_resource_ver(&url_str) {
        Ok(resource) => resource,
        Err(e) => {
            error!("Error extracting resource and version: {}", e);
            let message = get_error_page("E101");
            return Ok(Response::builder()
                .status(404)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?);
        }
    };

    if version == "v1" {
        match resource.as_str() {
            "launch" => {
                info!("Resource: {}", resource);
                // Get the IssuerUrl and Launch
                let iss = params.first("iss").unwrap_or("nan").to_string();
                let launch = params.first("launch").unwrap_or("nan").to_string();
                // let client_id = params.first("client_id").unwrap_or("nan").to_string();
            
                info!("iss: {}", iss);
                info!("launch: {}", launch);
                
                let auth_endpoint = match get_param_endpoint(
                    &iss, 
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
                // let code_verifier = generate_code_verifier();
                // let code_challenge = generate_code_challenge(&code_verifier);

                // Parse the base endpoint URL
                let base_url = Url::parse(&auth_endpoint)?;

                // Create a mutable URL for building the query
                let mut url = base_url.clone();

                // Add all query parameters
                url.query_pairs_mut()
                    .append_pair("response_type", "code")
                    .append_pair("client_id", &client_id)
                    .append_pair("scope", scope)
                    .append_pair("redirect_uri", &redirect_uri)
                    .append_pair("code_challenge", &code_challenge)
                    .append_pair("launch", &launch)
                    .append_pair("aud", &iss)
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
            "callback" => {
                info!("Resource: {}", resource);

                // Extract parameters
                let code = params.first("code").unwrap_or("nan").to_string();
                let session_state = params.first("session_state").unwrap_or("nan").to_string();
                let iss = params.first("iss").unwrap_or("nan").to_string();

                info!("code: {}", code);
                info!("session_state: {}", session_state);
                info!("iss: {}", iss);

                // Set token mutable
                let mut token = String::new();

                match get_session_token(
                    &session_state, 
                    &table_name
                ).await {
                    Ok(g_token) => token = g_token,
                    Err(e) => info!("Error checking session state: {}", e),
                }

                if token == "nan".to_string() {
                    // ToDo - Imrpove this line
                    let fmt_iss = format!("{}/api/fhir/1", iss);
                    
                    let token_endpoint = match get_param_endpoint(
                        &fmt_iss, 
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
                        &code, 
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

                    let expires_in = token_resp.expires_in
                        .unwrap_or_else(|| 3600);

                    let session_data = SessionData {
                        session_state: session_state,
                        access_token: token.clone(),
                        expires_in: expires_in,
                        scope: Some(scope.to_string()),
                        token_type: token_resp.token_type,
                        id_token: token_resp.id_token,
                    };
               
                    match save_session_token(
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
            "tasks" => {
                info!("Resource: {}", resource);
            }
            _ => {
                error!("Resource not found: {}", resource);
            }
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

pub fn extract_resource_ver(
    url_str: &str
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Parse the URL
    let url = Url::parse(url_str)?;

    // Get resource
    let resource = url
        .path_segments()
        .and_then(|mut segments| segments.nth(1))
        .ok_or("Invalid URL format")?
        .to_string();

    // Get version
    let version = url
        .path_segments()
        .and_then(|mut segments| segments.next())
        .ok_or("Invalid URL format")?
        .to_string();

    Ok((resource, version))
}

// use rand::Rng;
// use sha2::{Sha256, Digest};
// use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

// fn generate_code_verifier() -> String {
//     let mut rng = rand::rng();
//     let random_bytes: Vec<u8> = (0..32).map(|_| rng.random()).collect(); // 32 bytes = 43 chars after encoding
//     URL_SAFE_NO_PAD.encode(&random_bytes)
// }

// fn generate_code_challenge(code_verifier: &str) -> String {
//     let mut hasher = Sha256::new();
//     hasher.update(code_verifier.as_bytes());
//     let hash = hasher.finalize();
//     URL_SAFE_NO_PAD.encode(&hash)
// }