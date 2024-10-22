use reqwest::Client;
use serde_json::{ Value, json };
use std::env;
use std::fs;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the API token from the environment variable
    let api_token = env::var("REPLICATE_API_TOKEN")?;

    // Read the prompt and system prompt from files
    let prompt = fs::read_to_string("prompt.txt")?;
    let system_prompt = fs::read_to_string("system_prompt.txt")?;
    
    println!("Generating response...\n");

    // Create a new HTTP client
    let client = Client::new();

    // Define the JSON payload
    let payload = json!({
        "stream": false,
        "input": {
            "top_k": 50,
            "top_p": 0.9,
            "prompt": prompt.trim(),
            "max_tokens": 512,
            "min_tokens": 0,
            "temperature": 0.6,
            "system_prompt": system_prompt.trim(),
            "presence_penalty": 0,
            "frequency_penalty": 0
        }
    });

    // Make the POST request
    let response = client.post("https://api.replicate.com/v1/models/ibm-granite/granite-3.0-8b-instruct/predictions")
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .header("Prefer", "wait")
        .json(&payload)
        .send()
        .await?
        .text()
        .await?;

    // Parse the response as JSON
    let response_json: Value = serde_json::from_str(&response)?;

    // Extract the "output" field
    if let Some(output) = response_json["output"].as_array() {
        let output_text = output.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("");
        println!("{}", output_text);
    } else {
        println!("No output found");
    }

    Ok(())
}