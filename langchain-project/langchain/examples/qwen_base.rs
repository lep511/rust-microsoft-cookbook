#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use langchain::compatible::libs::ChatResponse;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://api.deepinfra.com/v1/openai/chat/completions";
    let model = "Qwen/QwQ-32B-Preview";
    let llm = ChatCompatible::new(base_url, model)?;
    
    let prompt = "Explain the Pythagorean theorem to a 10-year-old.";

    let start = Instant::now();
    let response: ChatResponse = llm
        .with_temperature(0.9)
        .with_timeout_sec(120)
        .invoke(prompt)
        .await?;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]", elapsed);

    println!("\n#### Example Groc simple shot ####");
    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let message = candidate.message {
                    println!("{}", message.content);
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}