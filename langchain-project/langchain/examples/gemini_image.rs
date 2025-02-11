#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatGemini::new("gemini-2.0-flash-exp");

    let file_path = Some("tests/files/breakfast.webp");
    let upload_data = None;
    let display_name = "breakfast.webp";
    let mime_type = "image/webp";

    let prompt = "Write a short and engaging blog post based on this picture.";

    let response = llm
        .media_upload(
            file_path,
            upload_data,
            display_name,
            mime_type,
        )
        .await?
        .invoke(prompt)
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