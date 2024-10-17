use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use aws_config::{self, BehaviorVersion, Region};
use aws_sdk_secretsmanager::Error as secret_man_error;
use aws_sdk_secretsmanager::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewSecret {
  pub host: String,
  pub username: String,
  pub password: String,
  pub dbname: String,
}

async fn show_secret() -> Result<String, secret_man_error> {
    let secret_name = "DemoWorkshopSecret";
    let region = Region::new("us-east-1");

    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .region(region)
        .load()
        .await;

    let asm = Client::new(&config);

    let response = asm
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;
    // For a list of exceptions thrown, see
    // https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html

    match response.secret_string() {
        Some(secret) => Ok(secret.to_string()),
        None => Ok("not secret found".to_string()),
    }
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    match show_secret().await {
        Ok(secret) => {
            let secret_message:NewSecret = serde_json::from_str(&secret).unwrap();
            println!(
                "Secret host: {}, username: {}, password: {}, dbname: {}",
                secret_message.host,
                secret_message.username,
                secret_message.password,
                secret_message.dbname
            );
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }


    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
