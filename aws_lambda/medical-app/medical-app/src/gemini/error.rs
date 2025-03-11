use std::env;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum GeminiError {
    #[error("Gemini API key not found in environment variables")]
    ApiKeyNotFound,
    
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    
    #[error("Failed to get response content")]
    ResponseContentError,
    
    #[error("Failed to post chat request")]
    RequestChatError,
    
    #[error("Failed to post upload request")]
    RequestUploadError,
    
    #[error("Failed to upload cache request")]
    RequestCacheError,
    
    #[error("Failed to upload embed request")]
    RequestEmbedError,
    
    #[error("Error in converting to json {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Failed to extract the mime type")]
    InvalidMimeType,
    
    #[error("{message}")]
    GenericError {
        message: String,
        detail: String,
    },
}