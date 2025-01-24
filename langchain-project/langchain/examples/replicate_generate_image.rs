#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use std::time::Instant;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let base_url = "https://api.replicate.com/v1";
    let model = "models/black-forest-labs/flux-1.1-pro-ultra/predictions";
    let prefer = "wait";
    
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
        .with_retry(0)
        .with_input_replicate(
            input_data,
            prefer,
        )
        .await?;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]", elapsed);

    println!("{:?}", response);
    
    Ok(())
}