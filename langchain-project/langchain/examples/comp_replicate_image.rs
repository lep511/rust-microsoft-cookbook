#[allow(dead_code)]
use reqwest::Client;
use langchain::compatible::chat::ChatCompatible;
use std::io::copy;
use std::fs::File;
use serde_json::{json, Value};
use env_logger::Env;

async fn flux_api() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint_url = "https://api.replicate.com/v1";
    let model = "models/black-forest-labs/flux-1.1-pro-ultra/predictions";

    let llm = ChatCompatible::new(endpoint_url, model);

    let prompt = "A close-up shot captures a winter wonderland scene - soft snowflakes \
            fall on a snow-covered forest floor. Behind a frosted pine branch, a red \
            squirrel sits, its bright orange fur a splash of color against the white. \
            It holds a small hazelnut. As it enjoys its meal, it seems oblivious to \
            the falling snow.";

    let input_data = json!({
        "raw": false,
        "prompt": prompt,
        "aspect_ratio": "3:2",
        "output_format": "jpg",
        "safety_tolerance": 2
    });

    let response: Value = llm
        .with_max_retries(0)
        .with_input_replicate(input_data)
        .await?;

    if let Some(output_url) = response.get("output").and_then(|url| url.as_str()) {
            let client = Client::new();
            let response = client.get(output_url).send().await?;
            let bytes = response.bytes().await?;
    
            // Create a file to save the image
            let file_name = match output_url.split("/").last() {
                Some(name) => name,
                None => "output.jpg",
            };
    
            let file_path = format!("tests/output/{}", file_name);
    
            // Create a file to save the image
            let mut file = File::create(&file_path)?;
    
            // Copy the bytes to the file
            copy(&mut bytes.as_ref(), &mut file)?;
            
            println!("Image saved as {}", file_path);
        }

    Ok(())
}

async fn image3_api() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint_url = "https://api.replicate.com/v1";
    let model = "models/google/imagen-3/predictions";

    let llm = ChatCompatible::new(endpoint_url, model);

    let prompt = "the serene interior of a cave, where a pool of water is nestled \
        amidst an abundance of greenery. Sunlight filters through the cave opening, \
        casting a soft glow on the surrounding foliage and the tranquil water below";

    let input_data = json!({
        "prompt": prompt,
        "aspect_ratio": "1:1",
        "safety_filter_level": "block_medium_and_above"
    });

    let response: Value = llm
        .with_max_retries(0)
        .with_input_replicate(input_data)
        .await?;

    if let Some(output_url) = response.get("output").and_then(|url| url.as_str()) {
        let client = Client::new();
        let response = client.get(output_url).send().await?;
        let bytes = response.bytes().await?;

        // Create a file to save the image
        let file_name = match output_url.split("/").last() {
            Some(name) => name,
            None => "output.jpg",
        };

        let file_path = format!("tests/output/{}", file_name);

        // Create a file to save the image
        let mut file = File::create(&file_path)?;

        // Copy the bytes to the file
        copy(&mut bytes.as_ref(), &mut file)?;
        
        println!("Image saved as {}", file_path);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    flux_api().await?;

    // image3_api().await?;
    
    Ok(())
}