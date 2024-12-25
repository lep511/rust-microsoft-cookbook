mod anthropic;
mod openai;
mod gemini;
mod groc;
mod xai;
mod examples;
use examples::{Models, all_examples};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let model = Models::Xai;

    match all_examples(model).await {
        Ok(_) => (),
        Err(e) => println!("Error running examples: {}", e),
    }
    Ok(())
}
