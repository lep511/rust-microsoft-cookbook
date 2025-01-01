#[allow(dead_code)]
use crate::gemini::ChatGemini;
use std::fs::File;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::json;

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let mut llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let type_file = "video";
    let mut prompt = "Describe this image";

    match type_file {
        "image" => {
            let file_path = "tests/files/image01.jpg";
            llm = llm.media_upload(file_path, "auto").await?;
        },
        "video" => {
            let file_path = "tests/files/sample.mp4";
            llm = llm.media_upload(file_path, "auto").await?;
            prompt = "Describe this video clip";
        },
        "pdf" => {
            let file_path = "tests/files/test.pdf";
            llm = llm.media_upload(file_path, "auto").await?;
            prompt = "Summarize this document";
        },
        "audio" => {
            let file_path = "tests/files/sample.mp3";
            llm = llm.media_upload(file_path, "auto").await?;
            prompt = "Summarize in a few lines this audio clip";
        },
        _ => {
            let file_path = "tests/files/sample.csv";
            llm = llm.media_upload(file_path, "auto").await?;
            prompt = "Summarize this document";
        }
    }

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