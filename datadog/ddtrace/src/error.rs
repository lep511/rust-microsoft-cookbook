use std::env;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum DatadogError {
    #[error("API Timeout Error: Request timed out. Retry after a brief wait. {0}")]
    APITimeoutError(String),

    #[error("Authentication Error: Invalid, expired or revoked API key/token. Check credentials or generate new ones. {0}")]
    AuthenticationError(String),

    #[error("Bad Request Error: There was an issue with the format or content of your request. Malformed request or missing parameters. {0}")]
    BadRequestError(String),

    #[error("Conflict Error: Resource was updated by another request. {0}")]
    ConflictError(String),

    #[error("Internal Server Error: Anthropic's API is temporarily overloaded. Retry after brief wait. {0}")]
    OverloadedServerError(String),

    #[error("Not Found Error: Requested resource {0} does not exist")]
    NotFoundError(String),

    #[error("Permission Denied Error: No access to requested resource. Verify API key and resource IDs. {0}")]
    PermissionDeniedError(String),

    #[error("Rate Limit Error: Request quota exceeded. Please pace requests. {0}")]
    RateLimitError(String),

    #[error("ANTHROPIC_API_KEY not found in environment variables")]
    ApiKeyNotFound,
       
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    
    #[error("Failed to get response content")]
    ResponseContentError,
        
    #[error("Failed to open file: {0}")]
    FileError(String),
    
    #[error("Error in converting to json {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("{message}")]
    GenericError {
        code: String,
        message: String,
        detail: String,
    },
}