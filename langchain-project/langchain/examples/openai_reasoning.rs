// https://platform.openai.com/docs/guides/reasoning?lang=curl
#[allow(dead_code)]
use langchain::openai::response::ChatOpenAI;
use langchain::openai::lib_response::OutputItem;
use std::time::Instant;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatOpenAI::new("o3-mini");
    
    let prompt = "Are semicolons optional in Rust";

    let start = Instant::now();

    let response = llm
        .with_prompt(prompt)
        .invoke()
        .await?;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]\n", elapsed);

    for output_item in response.output {
        // Check if the item is a 'Message' variant
        if let OutputItem::Message(message_data) = output_item {
            // Iterate through the 'content' array within the message
            for content_item in message_data.content {
                println!("{}", content_item.text);
                // If there could be multiple content items per message
                // and you only want the *first* one, you could add a break here:
                // break;
            }
        }
        // We ignore OutputItem::Reasoning variants
    }

    Ok(())
}