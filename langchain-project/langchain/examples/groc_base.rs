#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use langchain::compatible::libs::ChatResponse;
use std::time::Instant;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let endpoint_url = "https://api.groq.com/openai/v1/chat/completions";
    let model = "llama-3.3-70b-specdec";
    let llm = ChatCompatible::new(endpoint_url, model);

    let prompt = "Explain the Pythagorean theorem to a 10-year-old.";

    let start = Instant::now();

    let response: ChatResponse = llm.
        .with_temperature(0.9)
        .with_max_tokens(2048)
        .with_timeout_sec(30)
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