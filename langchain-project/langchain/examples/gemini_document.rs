#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let file_path = "tests/files/w9.pdf";

    let prompt = "What's the document type? Reply from the following options: \
        nvoice, Bank Statement, Paystub, Form 1040, Form W-9, Form 1099-R.";
    
    let start = Instant::now();
    
    let response = llm
        .media_upload(file_path, "auto")
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