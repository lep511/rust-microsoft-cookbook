#[allow(dead_code)]
use crate::groc::{ChatGroc, ChatResponse};
use std::time::Instant;

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGroc::new("llama-3.3-70b-specdec")?;
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);
    let llm = llm.with_timeout_sec(30);

    let prompt = "Explain the Pythagorean theorem to a 10-year-old.";

    let start = Instant::now();
    let response: ChatResponse = llm.invoke(prompt).await?;

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