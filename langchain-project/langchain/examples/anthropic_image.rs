#[allow(dead_code)]
use langchain::anthropic::chat::ChatAnthropic;
use env_logger::Env;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::general_purpose::STANDARD};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Read first image into a byte vector
    let mut file_01 = File::open("tests/files/image01.jpg")?;
    let mut buffer_01 = Vec::new();
    file_01.read_to_end(&mut buffer_01)?;
    
    // Read second image into a byte vector
    let mut file_02 = File::open("tests/files/image03.png")?;
    let mut buffer_02 = Vec::new();
    file_02.read_to_end(&mut buffer_02)?;
    
    // Convert to base64
    let base64_string_01 = STANDARD.encode(&buffer_01);
    let base64_string_02 = STANDARD.encode(&buffer_02);

    let mime_type_jpeg = "image/jpeg";
    let mime_type_png = "image/png";
    
    let llm = ChatAnthropic::new("claude-3-5-sonnet-20241022")?;

    let response = llm
        .with_image(&base64_string_01, mime_type_jpeg)
        .with_image(&base64_string_02, mime_type_png)
        .invoke("Compare the two pictures provided")
        .await?;

    println!("#### Example Anthropic Image Data ####");
    #[allow(irrefutable_let_patterns)]
    if let Some(candidates) = response.content {
        for candidate in candidates {
            match candidate.text {
                Some(text) => println!("{}", text),
                None => println!(""),
            }
        }
    };

    Ok(())
}