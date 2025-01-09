#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let file_path = "tests/files/breakfast.webp";
    llm = llm.media_upload(file_path, "auto").await?;

    let prompt = "Write a short and engaging blog post based on this picture.";


    let response = llm.invoke(prompt).await?;

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