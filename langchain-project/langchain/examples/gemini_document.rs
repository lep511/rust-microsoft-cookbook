#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use std::time::Instant;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::general_purpose::STANDARD};

fn base64_encode(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    STANDARD.encode(buffer)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let prompt = "What's the document type? Reply from the following options: \
        invoice, Bank Statement, Paystub, Form 1040, Form W-9, Form 1099-R.";

    let file_path = Some("tests/files/w9.pdf");
    // let file_path = None;

    // let upload_data = Some(base64_encode("tests/files/w9.pdf"));
    let upload_data = None;

    let display_name = "w9.pdf";
    let mime_type = "application/pdf";
    
    let start = Instant::now();
    
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

    let elapsed = start.elapsed().as_secs_f64();

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

    println!("[Task took {:.2} seconds]", elapsed);

    Ok(())
}