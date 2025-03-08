#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use futures::StreamExt;
use futures::pin_mut;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let endpoint_url = "https://api.deepinfra.com/v1/openai";
    let model = "meta-llama/Llama-3.3-70B-Instruct-Turbo";
    let llm = ChatCompatible::new(endpoint_url, model);

    let prompt = "Create a story about a young woman who discovers she has the power to control the weather.".to_string();

    let stream = llm
        .stream_response(prompt);

    pin_mut!(stream);

    println!("\n");

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