mod mongodb_op;

use mongodb_op::insert_documents;
use lambda_http::{ Body, Request, RequestExt, Response, Error };
use lambda_runtime::Diagnostic;
use aws_config;
use aws_sdk_secretsmanager as secretsmanager;
use serde::{Deserialize, Serialize};
use thiserror;

#[derive(Serialize, Deserialize, Debug)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("transient database error: {0}")]
    DatabaseError(String),
    #[error("unexpected error: {0}")]
    Unexpected(String),
    #[error("unexpected error: {0}")]
    ResourceNotFoundException(String)
}

impl From<ExecutionError> for Diagnostic {
    fn from(value: ExecutionError) -> Diagnostic {
        let (error_type, error_message) = match value {
            ExecutionError::DatabaseError(err) => ("Retryable", err.to_string()),
            ExecutionError::Unexpected(err) => ("NonRetryable", err.to_string()),
            ExecutionError::ResourceNotFoundException(err) => ("NonRetryable", err.to_string()),
        };
        Diagnostic {
            error_type: error_type.into(),
            error_message: error_message.into(),
        }
    }
}

fn parse_credentials(secret: &str) -> Result<Credentials, serde_json::Error> {
    serde_json::from_str(secret)
}

async fn get_secret() -> Result<Option<String>, secretsmanager::Error> {
    let secret_name = "mongodb/Cluster0";

    let config = aws_config::load_from_env().await;
    let client = secretsmanager::Client::new(&config);

    let response = client 
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;

    Ok(response.secret_string().map(|s| s.to_string()))
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    
    let uri: String = match get_secret().await {
        Ok(Some(secret_string)) => {
            match parse_credentials(&secret_string) {
                Ok(credentials) => {
                    format!(
                        "mongodb+srv://{}:{}@cluster0.tf9ci.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0",
                        credentials.username,
                        credentials.password
                    )
                }
                Err(e) => {
                    let message_error = format!("Error parsing credentials: {}", e);
                    return Err(ExecutionError::Unexpected(message_error).into())
                }
            }
        }
        Ok(None) => return Err(ExecutionError::Unexpected("No credentials found".to_string()).into()),
        Err(e) => {
            let message_error = format!("Error getting secret: {}", e);
            return Err(ExecutionError::ResourceNotFoundException(message_error).into())
        }
    };

    let database = "bedrock";
    let collection = "agenda";
    
    let _ = insert_documents(uri, database, collection).await;
    
    let message = "Ok".to_string();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}