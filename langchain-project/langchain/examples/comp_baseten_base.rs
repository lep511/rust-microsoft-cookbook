#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use std::time::Instant;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model_id = "6wgpr66w";
    let base_url = format!(
        "https://model-{}.api.baseten.co/environments/production/predict",
        model_id,
    );

    let llm = ChatCompatible::new(&base_url, model_id)?;

    let prompt = "Which is heavier, a pound of bricks or a pound of feathers?";

    let start = Instant::now();
    let response: Value = llm
        .with_retry(0)
        .baseten_invoke(prompt)
        .await?;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]", elapsed);

    println!("{:?}", response);
    Ok(())
}