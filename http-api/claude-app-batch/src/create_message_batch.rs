use reqwest::Client;
use serde_json::json;
use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct MessageBatch {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    processing_status: String,
    request_counts: RequestCounts,
    ended_at: Option<String>,
    created_at: String,
    expires_at: String,
    archived_at: Option<String>,
    cancel_initiated_at: Option<String>,
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

    // Create a new HTTP client
    let client = Client::new();

    // Define prompt
    let first_prompt = "What is the capital of France?";
    let second_prompt = "What is the capital of Germany?";

    let body = json!({
        "requests": [
            {
                "custom_id": "my-first-request",
                "params": {
                    "model": "claude-3-5-sonnet-20241022",
                    "max_tokens": 1024,
                    "messages": [
                        {"role": "user", "content": first_prompt}
                    ]
                }
            },
            {
                "custom_id": "my-second-request",
                "params": {
                    "model": "claude-3-5-sonnet-20241022",
                    "max_tokens": 1024,
                    "messages": [
                        {"role": "user", "content": second_prompt}
                    ]
                }
            }
        ]
    });

    // Send the POST request
    let response = client.post("https://api.anthropic.com/v1/messages/batches")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("anthropic-beta", "message-batches-2024-09-24")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    // Print the response
    let response_text = response.text().await?;
    //println!("{}", response_text);

    let message_batch: MessageBatch = serde_json::from_str(&response_text)?;
    println!("{:#?}", message_batch);

    Ok(())
}