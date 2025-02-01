#[allow(dead_code)]
use reqwest::Client;
use langchain::compatible::chat::ChatCompatible;
use std::time::Instant;
use std::io::copy;
use std::fs::File;
use serde_json::{json, Value};
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let base_url = "https://api.replicate.com/v1";
    let model = "models/black-forest-labs/flux-1.1-pro-ultra/predictions";
    // let prefer = "wait";
    
    let llm = ChatCompatible::new(base_url, model)?;

    let imege_prompt = "the serene interior of a cave, where a pool of water is nestled \
                    amidst an abundance of greenery. Sunlight filters through the cave opening, \
                    casting a soft glow on the surrounding foliage and the tranquil water below";


    let input_data = json!({
        "raw": false,
        "prompt": imege_prompt,
        "aspect_ratio": "3:2",
        "output_format": "jpg",
        "safety_tolerance": 2
    });

    let start = Instant::now();

    let response: Value = llm
        .with_max_retries(0)
        .with_input_replicate(input_data)
        .await?;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]", elapsed);
    // println!("{:?}", response);

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