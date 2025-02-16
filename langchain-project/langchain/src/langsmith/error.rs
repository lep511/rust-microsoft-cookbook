use std::env;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum LangsmithError {
    #[error("Langsmith API key not found in environment variables")]
    ApiKeyNotFound,
    
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    
    #[error("Failed to get response content")]
    ResponseContentError,
    
    #[error("Error reading from file")]
    FileReadError,
    
    #[error("Error in converting to serde_json::Value")]
    JsonError,
    
    #[error("Response content error: {message}")]
    GenericError {
        message: String,
        detail: String,
    },
}
