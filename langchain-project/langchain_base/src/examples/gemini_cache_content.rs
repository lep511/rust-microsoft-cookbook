#[allow(dead_code)]
use crate::gemini::ChatGemini;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::general_purpose::STANDARD};

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-1.5-flash-001")?;
    // Read first image into a byte vector
    let mut file_01 = File::open("tests/files/apolo11.txt")?;
    let mut buffer_01 = Vec::new();
    file_01.read_to_end(&mut buffer_01)?;
    
    // Convert to base64
    let base64_string_01 = STANDARD.encode(&buffer_01);

    let system_instruction = "You are an expert at analyzing transcripts.";
    
    let cache_url = llm.clone().cache_upload(
        base64_string_01,
        "text/plain",
        system_instruction
    ).await?;

    println!("cache_url: {}", cache_url);

    let llm = llm.with_cached_content(cache_url);
    let prompt = "Summarize briefly this transcript";

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