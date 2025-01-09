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
}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum ChatGrocChatError {
    #[error("Groc API key not found in environment variables")]
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