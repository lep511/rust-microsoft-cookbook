#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use langchain::compatible::libs::ChatResponse;
use tokio::time::{Duration, sleep};
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let endpoint_url = "https://api.x.ai/v1";
    let model = "grok-2-1212";
    let llm = ChatCompatible::new(endpoint_url, model);

    let system_prompt = "You are a helpful assistant explaining terms to user";
    let prompt = "Explain deferred chat completions to me.";

    let response: ChatResponse = llm
        .with_max_retries(0)
        .with_deferred(true)
        .with_system_prompt(system_prompt)
        .invoke(prompt)
        .await?;

    let mut response_id = String::new();

    if let Some(id) = response.request_id {
        response_id = id.clone();
        println!("Response id: {}", response_id);
    }

    // Wait for the response to be ready
    println!("Waiting 10 sec for response to be ready...");
    sleep(Duration::from_secs(10)).await;
    
    // Get the deferred from the server
    let llm = ChatCompatible::new(endpoint_url, model);
    let response = llm.get_deferred(&response_id).await?;

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