#[allow(dead_code)]
use crate::gemini::ChatGemini;
use std::fs::File;
use std::io::{Write, Read};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::json;
use std::path::Path;

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let file_path = "tests/files/test.pdf";
    let file_mime = "application/pdf";
    
    let file_url = llm.clone().media_upload(file_path, file_mime).await?;
    let prompt = "Give me a summary of this pdf file.";

    let llm = llm.with_file_url(file_url, file_mime);
    let llm = llm.with_timeout_sec(30);
    let response = llm.invoke(prompt).await?;

    let mut result = String::from("");

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                        result.push_str(&text);
                    }
                }
            }
        }
    };

    // Save result to file.md
    let file_path = "tests/output/test-pdf.md";
    let mut file = File::create(file_path)?;
    file.write_all(result.as_bytes())?;
    println!("Result saved to {}", file_path);

    // Check if the file exists
    let path = Path::new(file_path);
    assert!(path.exists(), "File was not created!");
    println!("File was created successfully!");

    Ok(())
}