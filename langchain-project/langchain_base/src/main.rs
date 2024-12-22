mod anthropic;
mod openai;
mod gemini;
mod examples;
use examples::all_examples;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match all_examples().await {
        Ok(_) => (),
        Err(e) => println!("Error running examples: {}", e),
    }
    Ok(())
}
