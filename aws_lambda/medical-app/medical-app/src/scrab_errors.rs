use thiserror::Error;
use url::ParseError;
use std::io;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ScrabError {   
    #[error("Network request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Invalid response format: {0}")]
    InvalidResponseFormat(#[from] serde_json::Error),
    
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),

    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] ParseError),
    
    #[error("Missing or invalid authorization_endpoint in response")]
    MissingAuthEndpoint,

    #[error("Missing or invalid token_endpoint in response")]
    MissingTokenEndpoint,
    
    #[error("Invalid authorization endpoint URL: {0}")]
    InvalidAuthEndpoint(String),

    #[error("Request error: {0}")]
    RequestError(String),
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("{0}")]
    GenericError(String),
}