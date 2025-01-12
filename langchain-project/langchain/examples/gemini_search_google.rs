use langchain::gemini::chat::ChatGemini;
use serde_json::json;

async fn example_tools() -> Result<(), Box<dyn std::error::Error>> {

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let question = "Who won the Super Bowl this actual year?";
    
    let response = llm
        .with_temperature(1.0)
        .with_google_search()
        .invoke(question)
        .await?;

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                    }
                }
            }
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    example_tools().await?;
    Ok(())
}