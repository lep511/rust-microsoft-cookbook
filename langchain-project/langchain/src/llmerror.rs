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
    #[error("API Connection Error: Issue connecting to services. Check network settings, proxy configuration, SSL certificates, or firewall rules. {0}")]
    APIConnectionError(String),

    #[error("API Timeout Error: Request timed out. Retry after a brief wait. {0}")]
    APITimeoutError(String),

    #[error("Authentication Error: Invalid, expired or revoked API key/token. Check credentials or generate new ones. {0}")]
    AuthenticationError(String),

    #[error("Bad Request Error: Malformed request or missing parameters. {0}")]
    BadRequestError(String),

    #[error("Conflict Error: Resource was updated by another request. {0}")]
    ConflictError(String),

    #[error("Internal Server Error: Issue on server side. Retry after brief wait. {0}")]
    InternalServerError(String),

    #[error("Not Found Error: Requested resource {0} does not exist")]
    NotFoundError(String),

    #[error("Permission Denied Error: No access to requested resource. Verify API key and resource IDs. {0}")]
    PermissionDeniedError(String),

    #[error("Rate Limit Error: Request quota exceeded. Please pace requests. {0}")]
    RateLimitError(String),

    #[error("Unprocessable Entity Error: Unable to process request despite correct format. {0}")]
    UnprocessableEntityError(String),

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
    #[error("API Connection Error: Issue connecting to services. Check network settings, proxy configuration, SSL certificates, or firewall rules. {0}")]
    APIConnectionError(String),

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

    #[error("Request Too Large Error: Request exceeds the maximum allowed number of bytes. {0}")]
    RequestTooLarge(String),

    #[error("Unprocessable Entity Error: Unable to process request despite correct format. {0}")]
    UnprocessableEntityError(String),

    #[error("ANTHROPIC_API_KEY not found in environment variables")]
    ApiKeyNotFound,
    
    #[error("VOYAGE_API_KEY not found in environment variables")]
    VoyageApiKeyNotFound,
    
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

    #[error("Error in Voyage's API. {0}")]
    VoyageError(String),
    
    #[error("{message}")]
    GenericError {
        code: String,
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


