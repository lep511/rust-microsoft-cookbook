use reqwest::blocking::Client;
use reqwest::{self, header::{HeaderMap, HeaderValue}};
use reqwest::Method;
use serde_json::Value;
use lambda_runtime::tracing::error;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::scrab_errors::ScrabError;

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
) -> Result<(String, String), ScrabError> {
    let client = Client::new();
    
    // First try SMART configuration
    let smart_config_url = format!("{}/.well-known/smart-configuration", iss.trim_end_matches('/'));
    let response = client.get(&smart_config_url).send()?;
    
    if response.status().is_success() {
        let config: Value = response.json()?;
        let auth_endpoint = config["authorization_endpoint"].as_str()
            .ok_or(ScrabError::MissingAuthEndpoint)?;
        let token_endpoint = config["token_endpoint"].as_str()
            .ok_or(ScrabError::MissingTokenEndpoint)?;
        
        return Ok((auth_endpoint.to_string(), token_endpoint.to_string()));
    }
    
    // Fallback to OAuth configuration
    let oauth_config_url = format!("{}/.well-known/oauth-authorization-server", iss.trim_end_matches('/'));
    let response = client.get(&oauth_config_url).send()?;
    
    if response.status().is_success() {
        let config: Value = response.json()?;
        let auth_endpoint = config["authorization_endpoint"].as_str()
            .ok_or(ScrabError::MissingAuthEndpoint)?;
        let token_endpoint = config["token_endpoint"].as_str()
            .ok_or(ScrabError::MissingTokenEndpoint)?;
        
        return Ok((auth_endpoint.to_string(), token_endpoint.to_string()));
    }
    
    Err(ScrabError::GenericError("Could not discover authorization endpoints".into()))
}

pub async fn get_token_accesss(
    client_id: &str,
    token_endpoint: &str,
    code: &str,
    code_verifier: &str,
    redirect_uri: &str,
    scope: &str,
) -> Result<TokenResponse, ScrabError> {
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

    let response = match request.send().await {
        Ok(response) => response,
        Err(e) => {
            error!("Error sending token request: {}", e);
            return Err(ScrabError::RequestError(e.to_string()));
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            error!("Error reading token response: {}", e);
            return Err(ScrabError::RequestError(e.to_string()));
        }
    };
    
    let token_response: TokenResponse = match serde_json::from_str(&body) {
        Ok(token) => token,
        Err(e) => {
            error!("Error parsing token response: {}", e);
            return Err(ScrabError::InvalidResponseFormat(e));
        }
    };

    Ok(token_response)
}

pub async fn get_mdata(
    iss: &str,
    query: &str,
    access_token: &str,
) -> Result<String, ScrabError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .build()?;

    let url = format!(
        "{}/{}", 
        iss.trim_end_matches('/'),
        query,
    );
    let mut headers = HeaderMap::new();
    let auth_str = format!("Bearer {}", access_token);

    headers.insert("accept", HeaderValue::from_static("application/json"));
    headers.insert("authorization", HeaderValue::from_str(&auth_str).unwrap());

    let request = client.request(Method::GET, url)
        .headers(headers);

    let response = match request.send().await {
        Ok(response) => response,
        Err(e) => {
            error!("Error sending token request: {}", e);
            return Err(ScrabError::RequestError(e.to_string()));
        }
    };

    // Check if status code is in the 200-299 range
    let status = response.status();
    if !status.is_success() {
        return Err(ScrabError::RequestError(format!(
            "Request failed with status: {}", status
        )));
    }
    
    // Parse the response body as a string
    let body = response.text().await?;
    Ok(body)
}
