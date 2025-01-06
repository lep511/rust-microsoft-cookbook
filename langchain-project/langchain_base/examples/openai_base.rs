#[allow(dead_code)]
use langchain_base::openai::{ChatOpenAI, ChatResponse};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatOpenAI::new("gpt-4o-mini")?;
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_completion_tokens(2048);
    let llm = llm.with_timeout_sec(30);
    let llm = llm.with_presence_penalty(1.5);
    let llm = llm.with_frequency_penalty(1.5);
    let llm = llm.with_n_completion(1);

    let llm = llm.with_top_p(0.4); // Recommend altering top_p with temperature but not both.

    let system_prompt = "You are a helpful assistant.";
    let llm = llm.with_system_prompt(system_prompt);

    let prompt = "Only say It's a test.";

    let start = Instant::now();
    let response: ChatResponse = llm.invoke(prompt).await?;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]", elapsed);

    println!("\n#### Example OpenAI simple shot ####");
    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let message = candidate.message {
                    println!("{:?}", message.content);
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}