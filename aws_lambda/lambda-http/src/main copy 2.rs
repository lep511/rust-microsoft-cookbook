use lambda_http::{run, http::{StatusCode, Response}, service_fn, Error, RequestExt, IntoResponse, Request};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(tracing::Level::INFO)
        .init();

    run(service_fn(function_handler)).await
}

pub async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let name = event.query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or_else(|| "stranger")
        .to_string();

    // Represents an HTTP response
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!({
            "message": format!("Hello, {}!", name),
          }).to_string())
        .map_err(Box::new)?;

    Ok(response)
}