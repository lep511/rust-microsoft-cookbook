#[allow(dead_code)]
use langchain::anthropic::EmbedMultiVoyage;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::general_purpose::STANDARD};

async fn example_url_image() -> Result<(), Box<dyn std::error::Error>> {
    // === URL IMAGE ===
    let llm = EmbedMultiVoyage::new("voyage-multimodal-3")?;
    let imag_url = "https://raw.githubusercontent.com/voyage-ai/voyage-multimodal-3/refs/heads/main/images/banana.jpg";
    let llm = llm.with_inline_data_url(imag_url);

    let message_input = "This is a banana.";
    let response = llm.embed_content(message_input).await?;

    println!("Response: {:?}", response);

    Ok(())
}

async fn example_base64_image() -> Result<(), Box<dyn std::error::Error>> {
    // === BASE64 IMAGE ===
    // Read image into a byte vector
    let mut file_01 = File::open("tests/files/image01.jpg")?;
    let mut buffer_01 = Vec::new();
    file_01.read_to_end(&mut buffer_01)?;

    // Convert to base64
    let image_base64 = STANDARD.encode(&buffer_01);
    let media_type = "image/jpeg";
    
    let llm = EmbedMultiVoyage::new("voyage-multimodal-3")?;
    let llm = llm.with_inline_data_base64(&image_base64, media_type);

    let message_input = "This is an office with a lot of people working in the evening.";
    let response = llm.embed_content(message_input).await?;

    println!("Response: {:?}", response);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    example_url_image().await?;
    // example_base64_image().await?;

    Ok(())
}