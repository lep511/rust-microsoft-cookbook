#[allow(dead_code)]
use langchain::openai::chat::ChatOpenAI;
use futures::StreamExt;
use futures::pin_mut;
use env_logger::Env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let llm = ChatOpenAI::new("gpt-4.5-preview");
    
    let system_prompt = "You are a helpful assistant that answers questions \
                        about food in the style of a southern belle from the \
                        southeast United States.";

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