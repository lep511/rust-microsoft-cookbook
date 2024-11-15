mod mongodb_op;

use mongodb_op::manage_mongodb;
use lambda_http::{ Body, Request, RequestExt, Response, Error };
use aws_config;
use aws_sdk_secretsmanager as secretsmanager;
use serde::{ Deserialize, Serialize };
use std::fs::{ write, read_to_string, remove_file };
use std::path::Path;
use std::env;

// Credentials for Secret Manager
#[derive(Serialize, Deserialize, Debug)]
struct Credentials {
    username: String,
    password: String,
}

/// Retrieves and constructs a MongoDB connection URI using credentials from AWS Secrets Manager
///
/// This function first checks for cached credentials in '/tmp/.credentials'. If found, it uses
/// those credentials to construct the URI. Otherwise, it fetches credentials from AWS Secrets
/// Manager and caches them for subsequent use.
///
/// # Arguments
///
/// * `secret_name` - The name/identifier of the secret in AWS Secrets Manager containing
///                   MongoDB credentials in JSON format with username and password fields
//////
/// # Cache
///
/// Credentials are cached at '/tmp/.credentials' to minimize AWS Secrets Manager API calls
///
/// # Errors
///
/// This function will return an error if:
/// * AWS Secrets Manager API call fails
/// * The secret string cannot be parsed as valid JSON
/// * The credentials JSON lacks required username or password fields
async fn get_mongouri(secret_name: &str) -> Result<String, secretsmanager::Error> {

    let base_uri = "mongodb+srv://<db_username>:<db_password>@cluster0.tf9ci.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";
    let path = Path::new("/tmp/.credentials");
    
    if path.exists() {
        println!("Using cached credentials...");
        let content = read_to_string("/tmp/.credentials").unwrap();
        let credentials: Credentials = serde_json::from_str(&content).unwrap();
        let uri = base_uri
                .replace("<db_username>", &credentials.username)
                .replace("<db_password>", &credentials.password);
        return Ok(uri);
    } else {
        let config = aws_config::load_from_env().await;
        let client = secretsmanager::Client::new(&config);

        let response = client
            .get_secret_value()
            .secret_id(secret_name)
            .send()
            .await?;

        let secret_string = response.secret_string().unwrap();

        let _ = write("/tmp/.credentials", secret_string);
        let credentials: Credentials = serde_json::from_str(secret_string).unwrap();

        let uri: String = format!(
            "mongodb+srv://{}:{}@cluster0.tf9ci.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0",
            credentials.username,
            credentials.password
        );

        Ok(uri)
    }
}

/// Handles HTTP requests for MongoDB operations
///
/// This function processes incoming HTTP requests and performs MongoDB operations based
/// on the 'action' query parameter. 
/// 
/// # Arguments
///
/// * `event` - The incoming HTTP request containing query parameters
///
/// # Returns
///
/// * `Result<Response<Body>, Error>` - Returns an HTTP response with appropriate status code:
///   - 200: Action processed successfully
///   - 400: Invalid action specified
///   - 403: Get URI error (Secret Manager error, or other)
///   - 404: No action specified
///   - 500: MongoDB connection or operation error
///
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let secret_name = env::var("MONGODB_SECRET_NAME").expect("MONGODB_SECRET_NAME environment variable not set.");
    let action = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("action"))
        .unwrap_or("no_action");   
                        
    let database = "sample_restaurants";
    let collection = "restaurants";

    match action {
        "insert" | "find" => {
            let uri = match get_mongouri(&secret_name).await {
                Ok(uri) => uri,
                Err(err) => {
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "text/html")
                        .body(format!("Error: {}", err).into())
                        .map_err(Box::new)?);
                }
            };
            match manage_mongodb(uri, action, &database, &collection).await {
                Ok(_) => {
                    let message = format!("Action processed successfully: {}", action);

                    let resp = Response::builder()
                        .status(200)
                        .header("content-type", "text/html")
                        .body(message.into())
                        .map_err(Box::new)?;

                    return Ok(resp)
                }
                Err(err) => {
                    let _ = remove_file("/tmp/.credentials"); // Delete cache file
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "text/html")
                        .body(format!("Error: {}", err).into())
                        .map_err(Box::new)?);
                }
            }
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
}