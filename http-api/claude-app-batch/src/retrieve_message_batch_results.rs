use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use std::env;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the API key from environment variables
    let api_key = env::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY environment variable not set.");

    let message_batch_id = "msgbatch_01RHsA7PMjckLM59ZtQ6nqRw";
    let url = format!("https://api.anthropic.com/v1/messages/batches/{}", message_batch_id);

    let mut headers = HeaderMap::new();
    headers.insert("x-api-key", HeaderValue::from_str(&api_key)?);
    headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
    headers.insert("anthropic-beta", HeaderValue::from_static("message-batches-2024-09-24"));

    let client = reqwest::Client::new();
    let response = client.get(url)
        .headers(headers)
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}