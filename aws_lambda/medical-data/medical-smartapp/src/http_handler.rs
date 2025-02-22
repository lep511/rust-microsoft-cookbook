use reqwest::Client;
use lambda_http::{Body, Error, Request, RequestExt, Response};
use crate::http_page::get_http_page;
use crate::oidc_request::{
    TokenResponse, exchange_code_for_tokens, 
    make_authenticated_request,
};
use lambda_http::tracing::{error, info};
use url::Url;
use std::env;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("Event: {:?}", event);
    // let who = event
    //     .query_string_parameters_ref()
    //     .and_then(|params| params.first("name"))
    //     .unwrap_or("world");

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

    if version == "v1" {
        match resource.as_str() {
            "launch" => {
                info!("Resource: {}", resource);
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
            }
            "callback" => {
                info!("Callback resource: {}", resource);
            }
            "patient" => {
                info!("Patient resource: {}", resource);
            }
            _ => {
                error!("Resource not found: {}", resource);
            }
        }
    }

    // let client = Client::builder()
    //     .use_rustls_tls()
    //     .build()?;

    let message = get_http_page();

    // let token_endpoint = "https://provider.example.com/oauth2/token";
    // let client_id = "your_client_id";
    // let client_secret = "your_client_secret";
    // let redirect_uri = "http://localhost:8080/callback";
    // let code = "authorization_code_from_redirect";

    // let tokens = exchange_code_for_tokens(
    //     &client,
    //     token_endpoint,
    //     client_id,
    //     client_secret,
    //     redirect_uri,
    //     code,
    // ).await?;

    // println!("Access Token: {}", tokens.access_token);

    // // Use the Access Token for Authenticated Requests
    // let protected_url = "https://api.example.com/protected-resource";
    // let access_token = "your_access_token";

    // make_authenticated_request(&client, protected_url, access_token).await?;

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
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