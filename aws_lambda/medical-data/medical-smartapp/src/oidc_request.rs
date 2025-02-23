use reqwest::Client;
use serde_json::Value;
use url::Url;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthEndpointError {   
    #[error("Network request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    
    #[error("Missing or invalid authorization_endpoint in response")]
    MissingAuthEndpoint,
    
    #[error("Invalid authorization endpoint URL: {0}")]
    InvalidAuthEndpoint(String),
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