#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use futures::StreamExt;
use futures::pin_mut;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://api.deepinfra.com/v1/openai/chat/completions";
    let model = "deepseek-ai/DeepSeek-R1-Distill-Llama-70B";
    let llm = ChatCompatible::new(base_url, model)?;

    let prompt = String::from("Tell me how the internet works in few words, but pretend I'm a puppy who only understands squeaky toys.");

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
    
    Ok(())
}