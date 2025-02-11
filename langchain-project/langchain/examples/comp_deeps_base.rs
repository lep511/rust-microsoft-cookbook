#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use langchain::compatible::libs::ChatResponse;
use std::time::Instant;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let base_url = "https://api.deepinfra.com/v1/openai/chat/completions";
    // let model = "deepseek-ai/DeepSeek-R1";
    let model = "meta-llama/Llama-3.3-70B-Instruct-Turbo";
    let llm = ChatCompatible::new(base_url, model);

    let prompt = "Create a story about a young woman who discovers she has the power to control the weather.";

    let start = Instant::now();

    let response: ChatResponse = llm
        .with_max_retries(0)
        .with_temperature(0.9)
        .invoke(prompt)
        .await?;
    
    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]\n", elapsed);

    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let Some(message) = candidate.message {
                    if let Some(content) = message.content {
                        println!("{}", content);
                    }
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}