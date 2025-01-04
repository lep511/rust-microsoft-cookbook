#[allow(dead_code)]
use langchain_base::gemini::ChatGemini;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);

    let llm = llm.with_system_prompt("You are a helpful assistant.");
    let prompt = "Only say It's a test.";
    let response = llm.invoke(prompt).await?;

    println!("\n#### Example Gemini simple shot ####");
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