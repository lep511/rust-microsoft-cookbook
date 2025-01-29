#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use futures::StreamExt;
use futures::pin_mut;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://api.deepinfra.com/v1/openai/chat/completions";
    let system_prompt = "Respond like a Michelin-starred chef.";
    let model = "deepseek-ai/DeepSeek-R1";
    let llm = ChatCompatible::new(base_url, model)?;

    let prompt = String::from("Can you name at least two different techniques to cook lamb?");

    let stream = llm
        .with_system_prompt(system_prompt)
        .with_max_tokens(4096)
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
    
    Ok(())
}