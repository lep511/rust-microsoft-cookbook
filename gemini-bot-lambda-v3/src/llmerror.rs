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

#[derive(Debug)]
pub struct S3Error(String);
impl S3Error {
    pub fn new(value: impl Into<String>) -> Self {
        S3Error(value.into())
    }

    pub fn add_message(self, message: impl Into<String>) -> Self {
        S3Error(format!("{}: {}", message.into(), self.0))
    }
}

impl<T: aws_sdk_s3::error::ProvideErrorMetadata> From<T> for S3Error {
    fn from(value: T) -> Self {
        S3Error(format!(
            "{}: {}",
            value
                .code()
                .map(String::from)
                .unwrap_or("unknown code".into()),
            value
                .message()
                .map(String::from)
                .unwrap_or("missing reason".into()),
        ))
    }
}

impl std::error::Error for S3Error {}

impl std::fmt::Display for S3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}