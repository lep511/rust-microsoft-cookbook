use reqwest::Client;
use reqwest::{self, header::{HeaderMap, HeaderValue}};
use reqwest::Method;
use serde_json::Value;
use url::Url;
use thiserror::Error;
use lambda_http::tracing::{info, error};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AuthEndpointError {   
    #[error("Network request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Invalid response format")]
    InvalidResponseFormat(#[from] serde_json::Error),
    
    #[error("Missing or invalid authorization_endpoint in response")]
    MissingAuthEndpoint,
    
    #[error("Invalid authorization endpoint URL: {0}")]
    InvalidAuthEndpoint(String),
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: Option<i32>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
}

/// Fetches the authorization endpoint from a SMART on FHIR issuer's configuration
///
/// # Arguments
/// * `iss` - The base URL of the SMART on FHIR issuer
///
/// # Returns
/// * `Result<String, AuthEndpointError>` - The authorization endpoint URL if successful
///
/// # Example
/// ```
/// let auth_url = get_auth_endpoint("https://example.com/fhir").await?;
/// ```
pub async fn get_auth_endpoint(iss: &str) -> Result<String, AuthEndpointError> {
    // Construct the well-known URL
    let config_url = format!("{}/.well-known/smart-configuration", iss);

    // Create client with reasonable defaults
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // Fetch and parse configuration
    let response: Value = client
        .get(config_url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    // Extract authorization endpoint
    let auth_endpoint = response["authorization_endpoint"]
        .as_str()
        .ok_or(AuthEndpointError::MissingAuthEndpoint)?;

    // Validate the authorization endpoint URL
    Url::parse(auth_endpoint)
        .map_err(|_| AuthEndpointError::InvalidAuthEndpoint(auth_endpoint.to_string()))?;

    Ok(auth_endpoint.to_string())
}

pub async fn get_token_accesss(
    client_id: &str,
    code: &str,
    code_verifier: &str,
    redirect_uri: &str,
    scope: &str,
) -> Result<TokenResponse, AuthEndpointError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let token_endpoint = "https://app.meldrx.com/connect/token";

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert("Accept", HeaderValue::from_static("application/json"));

    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("code", code);
    params.insert("grant_type", "authorization_code");
    params.insert("code_verifier", code_verifier);
    params.insert("redirect_uri", redirect_uri);
    params.insert("scope", scope);

    let request = client.request(Method::POST, token_endpoint)
        .headers(headers)
        .form(&params);

    let response = request.send().await?;
    let body = response.text().await?;
    info!("Token response: {}", body);

    let token_response: TokenResponse = match serde_json::from_str(&body) {
        Ok(token) => token,
        Err(e) => {
            error!("Error parsing token response: {}", e);
            return Err(AuthEndpointError::InvalidResponseFormat(e));
        }
    };

    Ok(token_response)
}