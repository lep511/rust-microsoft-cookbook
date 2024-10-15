use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use chrono::Utc;

async fn get_response(api_token: &str) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();

    let payload = json!({
        "input": {
            "prompt": "Against a black backdrop, a middle-aged Tongan woman twirls, her skin glowing and curly hair flowing. She wears an outfit resembling a whirlwind of marble and porcelain, illuminated by shard gleams, creating a dreamlike, fragmented yet fluid appearance.",
            "aspect_ratio": "1:1",
            "output_format": "png",
            "output_quality": 80,
            "safety_tolerance": 2,
            "prompt_upsampling": true
        }
    });

    let response = client
        .post("https://api.replicate.com/v1/models/black-forest-labs/flux-1.1-pro/predictions")
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .header("Prefer", "wait")
        .json(&payload)
        .send()
        .await?;

    let response_json: Value = response.json().await?;
    Ok(response_json)
}

async fn download_image(url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let file_response = client.get(url).send().await?;
    let bytes = file_response.bytes().await?;

    let date_time_now = Utc::now().format("%Y-%m-%dT%H-%M-%S").to_string();
    let file_name = format!("output-{}.png", date_time_now);

    let mut out = File::create(&file_name)?;
    copy(&mut bytes.as_ref(), &mut out)?;
    println!("File downloaded successfully as {}.", file_name);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_token = env::var("REPLICATE_API_TOKEN")
        .expect("REPLICATE_API_TOKEN environment variable not set");

    let response_json = get_response(&api_token).await?;
    
    if let Some(url) = response_json.get("output").and_then(|u| u.as_str()) {
        download_image(url).await?;
    } else {
        println!("Output URL not found in response.");
    }

    Ok(())
}