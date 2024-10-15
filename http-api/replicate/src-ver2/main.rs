use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get the API token from environment variables
    let api_token = env::var("REPLICATE_API_TOKEN")
        .expect("REPLICATE_API_TOKEN environment variable not set");

    // Create a new HTTP client
    let client = Client::new();

    // Define the JSON payload
    let payload = json!({
        "input": {
            "prompt": "A Baroque-style painting depicting a cat on a wooden rocking chair napping in the sunlight.",
            "aspect_ratio": "1:1",
            "output_format": "png",
            "output_quality": 80,
            "safety_tolerance": 2,
            "prompt_upsampling": true
        }
    });

    // Send the POST request
    let response = client
        .post("https://api.replicate.com/v1/models/black-forest-labs/flux-1.1-pro/predictions")
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .header("Prefer", "wait")
        .json(&payload)
        .send()
        .await?;

    // Parse the response text as JSON
    let response_json: Value = response.json().await?;
    
    // Extract the output URL
    if let Some(url) = response_json.get("output").and_then(|u| u.as_str()) {
        // Download the file
        let file_response = client.get(url).send().await?;
        let bytes = file_response.bytes().await?;
        
        // Get the current date and time
        let date_time_now = Utc::now().format("%Y-%m-%dT%H-%M-%S").to_string();
        let file_name = format!("output-{}.png", date_time_now);
        
        let mut out = File::create(&file_name)?;
        copy(&mut bytes.as_ref(), &mut out)?;
        println!("File downloaded successfully as {}.", file_name);
    } else {
        println!("Output URL not found in response.");
    }

    Ok(())
}