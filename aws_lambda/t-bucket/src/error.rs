use thiserror::Error;
use aws_sdk_s3tables::Error as S3TablesError;
use aws_sdk_athena::Error as AthenaError;
use aws_sdk_s3tables::error::BuildError as S3TablesBuildError;
use std::env;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum MainError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
        
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
    
    #[error("Response content error: {message}")]
    GenericError {
        message: String,
    },
}