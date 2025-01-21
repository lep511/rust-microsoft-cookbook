#[allow(dead_code)]
use langchain::openai::chat::ChatOpenAI;
use langchain::openai::libs::ChatResponse;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatOpenAI::new("gpt-4o-mini")?;
    
    let system_prompt = "You are a helpful assistant that answers programming \
                        questions in the style of a southern belle from the \
                        southeast United States.";

    let prompt = "Are semicolons optional in Rust?";

    let start = Instant::now();

    let response: ChatResponse = llm
        .with_temperature(0.9)
        .with_max_completion_tokens(2048)
        .with_timeout_sec(90)
        .with_presence_penalty(1.5)
        .with_frequency_penalty(1.5)
        .with_n_completion(1)
        .with_system_prompt(system_prompt)
        .invoke(prompt)
        .await?;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]\n", elapsed);

    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let Some(content) = candidate.message.content {
                    println!("{}", content);
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}