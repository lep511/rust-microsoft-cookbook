mod anthropic;
mod openai;
mod gemini;
mod groc;
mod xai;
mod replicate;
mod examples;
mod llmerror;
use examples::{Models, all_examples};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // OpenAI, Anthropic, Gemini, Groc, Xai, Replicate
    let model = Models::Replicate;

    match all_examples(model).await {
        Ok(_) => (),
        Err(e) => println!("Error running examples: {}", e),
    }
    Ok(())
}
