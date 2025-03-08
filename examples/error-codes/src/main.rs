use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Bad Request: The request is missing information or is malformed")]
    BadRequest,

    #[error("Unauthorized: The request lacks valid authentication credentials")]
    Unauthorized,

    #[error("Not Found: The requested resource cannot be found")]
    NotFound,

    #[error("Internal Server Error: We had a problem with our server")]
    InternalServerError,

    #[error("Service Unavailable: The requested service doesn't exist or isn't available")]
    ServiceUnavailable,

    // Optional: You might want to handle successful cases differently,
    // but we can include them as variants if they need to be part of the error flow
    #[error("Accepted: Successfully submitted request to bulk data server")]
    Accepted,

    #[error("Unexpected status code: {0}")]
    Unexpected(u16),
}

// Example usage with a function that converts an HTTP status code to our error type
impl ApiError {
    pub fn from_status_code(status: u16) -> Result<(), Self> {
        match status {
            200 => Ok(()), // Success case - no error
            202 => Err(ApiError::Accepted),
            400 => Err(ApiError::BadRequest),
            401 => Err(ApiError::Unauthorized),
            404 => Err(ApiError::NotFound),
            500 => Err(ApiError::InternalServerError),
            503 => Err(ApiError::ServiceUnavailable),
            code => Err(ApiError::Unexpected(code)),
        }
    }
}

// Example usage in code
fn handle_api_response(status: u16) -> Result<(), ApiError> {
    ApiError::from_status_code(status)?;
    Ok(())
}

fn main() {
    match handle_api_response(200) {
        Ok(()) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
}