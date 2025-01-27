#[allow(dead_code)]
use langchain::openai::chat::ChatOpenAI;
use langchain::openai::libs::ChatResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatOpenAI::new("gpt-4o")?;
    
    let prompt = "What is in this image?";

    let image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg";

    let response: ChatResponse = llm
        .with_image_url(image_url)
        .invoke(prompt)
        .await?;

    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let Some(content) = candidate.message.content {
                    println!("{}", content);
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}