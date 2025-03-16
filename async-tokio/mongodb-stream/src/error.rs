use aws_sdk_s3::Error as S3Error;
use mongodb::error::Error as MongoError;
use aws_smithy_runtime_api::client::result::SdkError;
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("S3 error: {0}")]
    S3Error(#[from] S3Error),
    
    #[error("AWS SDK error: {0}")]
    SdkError(String),
    
    #[error("MongoDB error: {0}")]
    MongoError(#[from] MongoError),
        
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("CSV parsing error: {0}")]
    CsvParseError(String),
    
    #[error("Generic error: {0}")]
    Generic(String),
}

impl AppError {
    pub fn generic<T: ToString>(error: T) -> Self {
        AppError::Generic(error.to_string())
    }
    
    pub fn csv_parse<T: ToString>(error: T) -> Self {
        AppError::CsvParseError(error.to_string())
    }
}

// Implement From for SdkError with any operation error and response
impl<E, R> From<SdkError<E, R>> for AppError 
where
    E: std::fmt::Display,
    R: std::fmt::Debug,
{
    fn from(err: SdkError<E, R>) -> Self {
        AppError::SdkError(err.to_string())
    }
}