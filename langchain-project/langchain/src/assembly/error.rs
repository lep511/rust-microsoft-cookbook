use std::env;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum AssemblyError {
    #[error("AssemblyAI API key not found in environment variables")]
    ApiKeyNotFound,
    
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    
    #[error("Failed to get response content")]
    ResponseContentError,
    
    #[error("The request failed due to an invalid request.")]
    BadRequest,

    #[error("The requested resource doesn’t exist.")]
    NotFound,

    #[error("Too many request were sent to the API. See Rate limits for more information.")]
    TooManyRequest,
    
    #[error("Something went wrong on AssemblyAI’s end.")]
    InternalServerError,
    
    #[error("Missing or invalid API key.")]
    Unauthorized,

    #[error("Failed to post upload request")]
    RequestUploadError,

    #[error("Error in converting to json {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Error reading from file")]
    FileReadError,
    
    #[error("{message}")]
    GenericError {
        message: String,
        detail: String,
    },
}