use reqwest::blocking::Client;
use reqwest::{self, header::{HeaderMap, HeaderValue}};
use reqwest::Method;
use serde_json::Value;
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

    #[error("Missing or invalid token_endpoint in response")]
    MissingTokenEndpoint,
    
    #[error("Invalid authorization endpoint URL: {0}")]
    InvalidAuthEndpoint(String),

    #[error("{0}")]
    GenericError(String),
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: Option<i32>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub patient: Option<String>,
}

pub async fn discover_endpoints(
    iss: &str
) -> Result<(String, String), AuthEndpointError> {
    let client = Client::new();
    
    // First try SMART configuration
    let smart_config_url = format!("{}/.well-known/smart-configuration", iss.trim_end_matches('/'));
    let response = client.get(&smart_config_url).send()?;
    
    if response.status().is_success() {
        let config: Value = response.json()?;
        let auth_endpoint = config["authorization_endpoint"].as_str()
            .ok_or(AuthEndpointError::MissingAuthEndpoint)?;
        let token_endpoint = config["token_endpoint"].as_str()
            .ok_or(AuthEndpointError::MissingTokenEndpoint)?;
        
        return Ok((auth_endpoint.to_string(), token_endpoint.to_string()));
    }
    
    // Fallback to OAuth configuration
    let oauth_config_url = format!("{}/.well-known/oauth-authorization-server", iss.trim_end_matches('/'));
    let response = client.get(&oauth_config_url).send()?;
    
    if response.status().is_success() {
        let config: Value = response.json()?;
        let auth_endpoint = config["authorization_endpoint"].as_str()
            .ok_or(AuthEndpointError::MissingAuthEndpoint)?;
        let token_endpoint = config["token_endpoint"].as_str()
            .ok_or(AuthEndpointError::MissingTokenEndpoint)?;
        
        return Ok((auth_endpoint.to_string(), token_endpoint.to_string()));
    }
    
    Err(AuthEndpointError::GenericError("Could not discover authorization endpoints".into()))
}

pub async fn get_token_accesss(
    client_id: &str,
    token_endpoint: &str,
    code: &str,
    code_verifier: &str,
    redirect_uri: &str,
    scope: &str,
) -> Result<TokenResponse, AuthEndpointError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .build()?;

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