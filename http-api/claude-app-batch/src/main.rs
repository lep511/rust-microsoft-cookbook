use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Response {
    custom_id: String,
    result: ResponseResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct ResponseResult {
    #[serde(rename = "type")]
    result_type: Option<String>,
    message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Message {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    role: String,
    model: String,
    content: Vec<Content>,
    stop_reason: String,
    stop_sequence: Option<String>,
    usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Content {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
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

        let results_data = results_response.text().await?;
        //println!("{}", results_data);

        // Split input into individual JSON objects and parse each one
        let responses: Vec<Response> = results_data
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                match serde_json::from_str(line) {
                    Ok(response) => Some(response),
                    Err(e) => {
                        eprintln!("Error parsing JSON: {}", e);
                        None
                    }
                }
            })
            .collect();

        // Print the parsed responses
        for response  in &responses {

            if let Some(result_type) = &response.result.result_type {
                if result_type == "succeeded" {
                    println!("\nSuccess! {}", response.custom_id);
                    println!("Result: {:?}", response.result.message.content[0].text);
                } else if result_type == "errored" {
                    println!("Validation or server error: {}", response.custom_id);
                } else if result_type == "expired" {
                    println!("Expired: {:?}", response.result);
                }
            }
        }

    } else {
        println!("Results URL not found");
    }

    Ok(())
}