#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use langchain::compatible::libs::ChatResponse;
use futures::StreamExt;
use futures::pin_mut;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("error")).init();

    let base_url = "https://DeepSeek-R1-mfvjj.northcentralus.models.ai.azure.com/v1/chat/completions";
    let model = "R1";
    let llm = ChatCompatible::new(base_url, model);

    let prompt = "What is the answer to life and universe?".to_string();

    let stream = llm
        .stream_response(prompt);
    
    pin_mut!(stream);

    while let Some(stream_response) = stream.next().await { 
        if let Some(choices) = stream_response.choices {
            for choice in choices {
                if let Some(delta) = choice.delta {
                    if let Some(content) = delta.content {
                        if content.is_empty() {
                            continue;
                        }
                        print!("{}", content);
                    }
                }
            }
        };
    }
    println!("\n\n");
    
    Ok(())
}