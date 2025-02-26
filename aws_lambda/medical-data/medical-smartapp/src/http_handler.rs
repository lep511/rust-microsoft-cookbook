use lambda_http::{Body, Error, Request, RequestExt, Response};
use crate::http_page::{get_http_page, get_connect_page, get_error_page};
use crate::oidc_request::{TokenResponse, get_token_accesss};
use crate::oidc_database::{SessionData, get_session_token, save_session_token};
use lambda_http::tracing::{error, info};
use url::Url;
use std::env;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("Event: {:?}", event);
    
    // Get environment variables
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");

    // Get Smart App callback
    let url_str = event.uri().to_string();
    let scope = "meldrx-api cds profile openid launch patient/*.*";
    let code_verifier = "Q4XqM0pPdsNHwhdEpt6eVAil7djAzhf6zMRAmbb8d-4".to_string();
    let code_challenge = "scOFvF4mB7t-R5egnefSgn0W_hL4HAzYKG-zDs_mWgM".to_string();
    let auth_endpoint = "https://app.meldrx.com/connect/authorize";
    
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
                let (iss, launch) = match extract_query_params(&url_str, "iss", "launch") {
                    Ok((iss, launch)) => (iss, launch),
                    Err(e) => {
                        error!("Error extracting query parameters: {}", e);
                        let message = get_error_page("E102");
                        return Ok(Response::builder()
                            .status(404)
                            .header("content-type", "text/html")
                            .body(message.into())
                            .map_err(Box::new)?);
                    }
                };
            
                info!("iss: {}", iss);
                info!("launch: {}", launch);
                
                // let auth_endpoint = match get_auth_endpoint(&iss).await {
                //     Ok(auth_endpoint) => auth_endpoint,
                //     let message = get_error_page("E103");
                //     Err(e) => {
                //         error!("Error getting auth endpoint: {}", e);
                //         return Ok(Response::builder()
                //             .status(404)
                //             .header("content-type", "text/html")
                //             .body(message.into())
                //             .map_err(Box::new)?);
                //     }
                // };

                // Generate the CodeVerifier and CodeChallenge
                // let code_verifier = generate_code_verifier();
                // let code_challenge = generate_code_challenge(&code_verifier);

                // Parse the base endpoint URL
                let base_url = Url::parse(auth_endpoint)?;

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

                // Convert to string
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
                let parsed_url = Url::parse(&url_str)?;

                // Extract the code parameter
                let code = parsed_url.query_pairs()
                    .find(|(key, _)| key == "code")
                    .map(|(_, value)| value.into_owned())
                    .unwrap_or_else(|| "nan".to_string());

                // Extract the session state parameter
                let session_state = parsed_url.query_pairs()
                    .find(|(key, _)| key == "session_state")
                    .map(|(_, value)| value.into_owned())
                    .unwrap_or_else(|| "nan".to_string());

                if code == "nan" || session_state == "nan" {
                    error!("Code or session state not found");
                    let message = get_error_page("E104");
                    return Ok(Response::builder()
                        .status(404)
                        .header("content-type", "text/html")
                        .body(message.into())
                        .map_err(Box::new)?);
                }

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
                    let token_resp: TokenResponse = match get_token_accesss(
                        &client_id,
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
        
                // Get the IssuerUrl and Code
                let (iss, code) = match extract_query_params(&url_str, "iss", "code") {
                    Ok((iss, code)) => (iss, code),
                    Err(e) => {
                        println!("Error extracting query parameters: {}", e);
                        ("none".to_string(), "none".to_string())
                    }
                };
                info!("iss: {}", iss);
                info!("code: {}", code);

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

fn extract_query_params(
    url_str: &str,
    param1: &str,
    param2: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Parse the URL
    let url = Url::parse(url_str)?;

    // Get the query pairs as a HashMap-like structure
    let query_pairs: Vec<_> = url.query_pairs().collect();

    // Extract 'iss' and 'launch' values
    let iss = query_pairs
        .iter()
        .find(|(key, _)| key == param1)
        .map(|(_, value)| value.to_string())
        .ok_or("Missing 'iss' parameter")?;

    let launch = query_pairs
        .iter()
        .find(|(key, _)| key == param2)
        .map(|(_, value)| value.to_string())
        .ok_or("Missing 'launch' parameter")?;

    Ok((iss, launch))
}

fn extract_resource_ver(
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