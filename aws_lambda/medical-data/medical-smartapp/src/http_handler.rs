use lambda_http::{Body, Error, Request, RequestExt, Response};
use rand::Rng;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use crate::http_page::{get_http_page, get_connect_page};
use crate::oidc_request::{
    get_auth_endpoint,
};
use lambda_http::tracing::{error, info};
use url::Url;
use std::env;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("Event: {:?}", event);
    
    // Get Smart App callback
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let url_str = event.uri().to_string();
    
    let (resource, version) = match extract_resource_ver(&url_str) {
        Ok(resource) => resource,
        Err(e) => {
            error!("Error extracting resource and version: {}", e);
            return Ok(Response::builder()
                .status(404)
                .header("content-type", "text/html")
                .body("<!DOCTYPE html><html><head><title>Not Found</title></head><body><h1>Not Found.</h1></body></html>".into())
                .map_err(Box::new)?);
        }
    };

    let mut message = String::new();

    if version == "v1" {
        match resource.as_str() {
            "launch" => {
                info!("Resource: {}", resource);
                // Get the IssuerUrl and Launch
                let (iss, launch) = match extract_query_params(&url_str) {
                    Ok((iss, launch)) => (iss, launch),
                    Err(e) => {
                        error!("Error extracting query parameters: {}", e);
                        return Ok(Response::builder()
                            .status(404)
                            .header("content-type", "text/html")
                            .body("<!DOCTYPE html><html><head><title>Not Found</title></head><body><h1>Not Found.</h1></body></html>".into())
                            .map_err(Box::new)?);
                    }
                };
            
                info!("iss: {}", iss);
                info!("launch: {}", launch);
                
                let auth_endpoint = match get_auth_endpoint(&iss).await {
                    Ok(auth_endpoint) => auth_endpoint,
                    Err(e) => {
                        error!("Error getting auth endpoint: {}", e);
                        return Ok(Response::builder()
                            .status(404)
                            .header("content-type", "text/html")
                            .body("<!DOCTYPE html><html><head><title>Not Found</title></head><body><h1>Not Found.</h1></body></html>".into())
                            .map_err(Box::new)?);
                    }
                };

                // Generate the code_verifier
                let code_verifier = generate_code_verifier();
                let code_challenge = generate_code_challenge(&code_verifier);

                // Parse the base endpoint URL
                let base_url = Url::parse(&auth_endpoint)?;

                // Create a mutable URL for building the query
                let mut url = base_url.clone();

                // Add all query parameters
                url.query_pairs_mut()
                    .append_pair("response_type", "code")
                    .append_pair("client_id", &client_id)
                    .append_pair("scope", "launch openid patient/*.*")
                    .append_pair("redirect_uri", &redirect_uri)
                    .append_pair("code_challenge", &code_challenge)
                    .append_pair("code_challenge_method", "S256");

                // Convert to string
                let link = url.to_string();
                message = get_connect_page(&link);
            }
            "callback" => {
                info!("Resource: {}", resource);
                message = get_http_page();
            }
            "patient" => {
                info!("Resource: {}", resource);
                message = get_http_page();
            }
            _ => {
                error!("Resource not found: {}", resource);
                message = get_http_page();
            }
        }
    }

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

fn extract_query_params(
    url_str: &str
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Parse the URL
    let url = Url::parse(url_str)?;

    // Get the query pairs as a HashMap-like structure
    let query_pairs: Vec<_> = url.query_pairs().collect();

    // Extract 'iss' and 'launch' values
    let iss = query_pairs
        .iter()
        .find(|(key, _)| key == "iss")
        .map(|(_, value)| value.to_string())
        .ok_or("Missing 'iss' parameter")?;

    let launch = query_pairs
        .iter()
        .find(|(key, _)| key == "launch")
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

fn generate_code_verifier() -> String {
    let mut rng = rand::rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.random()).collect(); // 32 bytes = 43 chars after encoding
    URL_SAFE_NO_PAD.encode(&random_bytes)
}

fn generate_code_challenge(code_verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(&hash)
}