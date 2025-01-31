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

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum OpenAIError {
    #[error("API Connection Error: Issue connecting to services. Check network settings, proxy configuration, SSL certificates, or firewall rules")]
    APIConnectionError,

    #[error("API Timeout Error: Request timed out. Retry after a brief wait")]
    APITimeoutError,

    #[error("Authentication Error: Invalid, expired or revoked API key/token. Check credentials or generate new ones. {0}")]
    AuthenticationError(String),

    #[error("Bad Request Error: Malformed request or missing parameters. {0}")]
    BadRequestError(String),

    #[error("Conflict Error: Resource was updated by another request")]
    ConflictError,

    #[error("Internal Server Error: Issue on server side. Retry after brief wait. {0}")]
    InternalServerError(String),

    #[error("Not Found Error: Requested resource {0} does not exist")]
    NotFoundError(String),

    #[error("Permission Denied Error: No access to requested resource. Verify API key and resource IDs")]
    PermissionDeniedError,

    #[error("Rate Limit Error: Request quota exceeded. Please pace requests")]
    RateLimitError,

    #[error("Unprocessable Entity Error: Unable to process request despite correct format")]
    UnprocessableEntityError,

    #[error("OpenAI API key not found in environment variables")]
    ApiKeyNotFound,
    
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    
    #[error("Error in converting to json {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Failed to get response content")]
    ResponseContentError,
    
    #[error("{message}")]
    GenericError {
        code: String,
        message: String,
        detail: String,
    },
}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum ReplicateError {
    #[error("Replicate API key not found in environment variables")]
    ApiKeyNotFound,
    
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    
    #[error("Failed to get response content")]
    ResponseContentError,
    
    #[error("Failed to create file")]
    FileCreateError,
    
    #[error("Failed to copy content to file")]
    FileCopyError,
}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum AnthropicError {
    #[error("ANTHROPIC API key not found in environment variables")]
    ApiKeyNotFound,
    
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    
    #[error("Failed to get response content")]
    ResponseContentError,
    
    #[error("Unsupported media type")]
    MediaTypeError,
    
    #[error("Failed to open file: {0}")]
    FileError(String),
    
    #[error("Error in converting to json {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("{message}")]
    GenericError {
        message: String,
        detail: String,
    },
}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum CompatibleChatError {
    #[error("COMPATIBLE_API_KEY not found in environment variables")]
    ApiKeyNotFound,
    
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    
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

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum XAIChatError {
    #[error("X-AI API key not found in environment variables")]
    ApiKeyNotFound,
    
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    
    #[error("Failed to get response content")]
    ResponseContentError,
}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum AssemblyAIError {
    #[error("AssemblyAI API key not found in environment variables")]
    ApiKeyNotFound,
    
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    
    #[error("Failed to get response content")]
    ResponseContentError,
    
    #[error("The model must be best or nano")]
    InvalidModel,
    
    #[error("Error reading from file")]
    FileReadError,
}

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


