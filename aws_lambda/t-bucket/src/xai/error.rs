use std::env;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum CompatibleChatError {
    #[error("XAI_API_KEY not found in environment variables")]
    ApiKeyNotFound,
    
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),

    #[error("Failed to open file: {0}")]
    FileError(String),
    
    #[error("Failed to get response content")]
    ResponseContentError,
    
    #[error("Error in converting to json {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Response content error: {message}")]
    GenericError {
        message: String,
        detail: String,
    },
}