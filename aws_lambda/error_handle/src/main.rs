use lambda_runtime::{service_fn, Diagnostic, Error, tracing, LambdaEvent};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use thiserror;

#[derive(Deserialize)]
struct Request {}

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("transient database error: {0}")]
    DatabaseError(String),
    #[error("unexpected error: {0}")]
    Unexpected(String),
}

impl From<ExecutionError> for Diagnostic {
    fn from(value: ExecutionError) -> Diagnostic {
        let (error_type, error_message) = match value {
            ExecutionError::DatabaseError(err) => ("Retryable", err.to_string()),
            ExecutionError::Unexpected(err) => ("NonRetryable", err.to_string()),
        };
        Diagnostic {
            error_type: error_type.into(),
            error_message: error_message.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewIceCreamEvent {
  pub flavors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewIceCreamResponse {
  pub flavors_count: usize,
}

/// This is the main body for the Lambda function
async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    
    if let Ok(event_json) = serde_json::from_value::<NewIceCreamEvent>(event) {
        println!("Parsed event: {:?}", event_json);
        let response = NewIceCreamResponse {
            flavors_count: event_json.flavors.len(),
        };
        // let response_json = serde_json::to_string(&response)?;
        Ok(json!({ "message": format!("Flavors added count: {}", response.flavors_count) }))
    } else {
        return Err(ExecutionError::Unexpected("Failed to parse JSON data".to_string()).into())
    }

}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    lambda_runtime::run(service_fn(function_handler)).await
}
