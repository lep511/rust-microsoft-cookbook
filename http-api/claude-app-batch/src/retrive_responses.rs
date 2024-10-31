use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct MessageBatch {
    processing_status: String,
    request_counts: RequestCounts,
    results_url: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct RequestCounts {
    processing: u32,
    succeeded: u32,
    errored: u32,
    canceled: u32,
    expired: u32,
}

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

    let client = Client::new();
    let response = client.get(url)
        .headers(headers.clone())
        .send()
        .await?;

    let body = response.text().await?;
    
    let message_batch: MessageBatch = serde_json::from_str(&body)?;

    println!("Processing Status: {}", message_batch.processing_status);
    println!("Request Counts: {:?}", message_batch.request_counts);
    if let Some(results_url) = message_batch.results_url {
        println!("Results URL: {}", results_url);

        // Second request to fetch the results
        let results_response = client.get(&results_url)
            .headers(headers)
            .send()
            .await?;

        let results_body = results_response.text().await?;
        println!("{}", results_body);
    } else {
        println!("Results URL not found");
    }

    Ok(())
}