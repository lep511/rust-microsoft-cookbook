#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use env_logger::Env;
use futures::StreamExt;
use futures::pin_mut;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatGemini::new("gemini-2.5-pro-preview-03-25");

    let prompt = String::from("Tell me how the internet works, but pretend I'm a puppy who only understands squeaky toys.");

    let stream = llm
        .stream_response(prompt);
    
    pin_mut!(stream);

    while let Some(stream_response) = stream.next().await { 
        if let Some(candidates) = stream_response.candidates {
            for candidate in candidates {
                if let Some(content) = candidate.content {
                    for part in content.parts {
                        if let Some(text) = part.text {
                            print!("{}", text);
                        }
                    }
                }
            }
        };
    }

    Ok(())
}