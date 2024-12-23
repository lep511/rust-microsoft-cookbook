mod anthropic;
mod openai;
mod gemini;
mod groc;
mod examples;
use examples::all_examples;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let model = "groc";

    match all_examples(model).await {
        Ok(_) => (),
        Err(e) => println!("Error running examples: {}", e),
    }
    Ok(())
}
