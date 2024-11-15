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

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let action = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("action"))
        .unwrap_or("no_action");

    let secret_name = "MongoDBCredentials";

    let config = aws_config::load_from_env().await;
    let client = secretsmanager::Client::new(&config);

    let response = client 
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;
    
    let secret_string = response.secret_string().unwrap();
    let credentials: Credentials = serde_json::from_str(secret_string).unwrap();         
                        
    let uri: String = format!(
        "mongodb+srv://{}:{}@cluster0.tf9ci.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0",
        credentials.username,
        credentials.password
    );

    let database = "sample_restaurants";
    let collection = "restaurants";

    match action {
        "insert" => {
            let _ = insert_documents(uri, database, collection).await;
        }
        "no_action" => {
            return Ok(Response::builder()
                .status(404)
                .header("content-type", "text/html")
                .body("No action specified".into())
                .map_err(Box::new)?);
        }
        _ => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/html")
                .body(format!("Bad action: {}", action).into())
                .map_err(Box::new)?);
        } 
    }

    let message = format!("Action processed successfully: {}", action);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}