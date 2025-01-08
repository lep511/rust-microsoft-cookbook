#[allow(dead_code)]
use langchain_base::gemini::ChatGemini;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::general_purpose::STANDARD};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Read into a byte vector
    let mut file_01 = File::open("tests/files/apolo11.txt")?;
    let mut buffer_01 = Vec::new();
    file_01.read_to_end(&mut buffer_01)?;

    // Convert to base64
    let base64_string_01 = STANDARD.encode(&buffer_01);

    let llm = ChatGemini::new("gemini-1.5-flash-001")?;

    let instruction = "You are an expert at analyzing transcripts.";
    let ttl = 300;

    let url_cache = match llm.clone().cache_upload(
        base64_string_01,
        "text/plain",
        instruction,
        ttl,
    ).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let response = llm
        .with_cached_content(url_cache)
        .invoke("What is the main theme of the transcript?")
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