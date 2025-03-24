use thiserror::Error;
use crate::openai::error::OpenAIError;
use aws_sdk_s3tables::Error as S3TablesError;
use aws_sdk_athena::Error as AthenaError;
use aws_sdk_s3tables::error::BuildError as S3TablesBuildError;
use tokio::io::Error as TokioIoError;
use std::env;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum MainError {
    // #[error("IO error: {0}")]
    // Io(#[from] std::io::Error),

    #[error("Tokio IO error: {0}")]
    TokioIo(#[from] TokioIoError),

    #[error("Tokio IO error with string: {0}")]
    TokioIoString(String),
        
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),

    #[error("Failed to open file: {0}")]
    FileError(String),
    
    #[error("Failed to get response content")]
    ResponseContentError,

    #[error("S3Tables error: {0}")]
    S3Tables(#[from] S3TablesError),

    #[error("S3Tables build error: {0}")]
    S3TablesBuildError(#[from] S3TablesBuildError),
    
    #[error("Athena error: {0}")]
    Athena(#[from] AthenaError),
    
    #[error("Error in converting to json {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Dynamic error: {0}")]
    DynamicError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("OpenAI error: {0}")]
    OpenAIError(#[from] OpenAIError),
    
    #[error("Response content error: {message}")]
    GenericError {
        message: String,
    },
}