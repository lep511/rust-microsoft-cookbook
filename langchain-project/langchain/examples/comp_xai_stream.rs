#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use futures::StreamExt;
use futures::pin_mut;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint_url = "https://api.x.ai/v1";
    let model = "grok-2-latest";
    let llm = ChatCompatible::new(endpoint_url, model);

    let prompt = String::from("Tell me how the internet works, but pretend I'm a puppy who only understands squeaky toys.");

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