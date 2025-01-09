#[allow(dead_code)]
use langchain::xai::{ChatXAI, ChatResponse};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let llm = ChatXAI::new("grok-2-1212")?;
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);
    let llm = llm.with_timeout_sec(30);
    let llm = llm.with_presence_penalty(1.5);
    let llm = llm.with_frequency_penalty(1.5);
    let llm = llm.with_n_completion(2);

    let llm = llm.with_top_p(0.4); // Recommend altering top_p with temperature but not both.

    let s_stop = vec!["\n\n".to_string()];
    let llm = llm.with_stop(s_stop);
    
    // let llm = llm.with_system_prompt("You are Grok, a chatbot inspired by the Hitchhikers Guide to the Galaxy.");
    // let prompt = "What is the meaning of life, the universe, and everything?";
    
    let llm = llm.with_system_prompt("Always be truthful. If you are unsure, say \"I don't know\"");
    let prompt = "Explain the Pythagorean theorem to a 10-year-old.";
        
    let start = Instant::now();
    let response: ChatResponse = llm.invoke(prompt).await?;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]", elapsed);

    println!("\n#### Example X-AI simple shot ####");
    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let message = candidate.message {
                    println!("{:?}", message.content);
                    println!("\n-----------------------------------------\n");
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}