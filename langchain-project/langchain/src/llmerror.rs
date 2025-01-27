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
    #[error("Failed to extract the mime type")]
    InvalidMimeType,
    #[error("{message}")]
    GenericError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum OpenAIError {
    #[error("OpenAI API key not found in environment variables")]
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


