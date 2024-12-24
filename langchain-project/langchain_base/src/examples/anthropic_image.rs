use crate::anthropic::ChatAnthropic;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::general_purpose::STANDARD};

pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    // Read first image into a byte vector
    let mut file_01 = File::open("src/examples/files/image01.jpg")?;
    let mut buffer_01 = Vec::new();
    file_01.read_to_end(&mut buffer_01)?;
    
    // Read second image into a byte vector
    let mut file_02 = File::open("src/examples/files/image03.png")?;
    let mut buffer_02 = Vec::new();
    file_02.read_to_end(&mut buffer_02)?;
    
    // Convert to base64
    let base64_string_01 = STANDARD.encode(&buffer_01);
    let base64_string_02 = STANDARD.encode(&buffer_02);

    let llm = ChatAnthropic::new("claude-3-5-sonnet-20241022")?;
    let llm = llm.with_image_jpeg(&base64_string_01);
    let llm = llm.with_image_png(&base64_string_02);
    let prompt = "Compare the two pictures provided";
    let response = llm.invoke(prompt).await?;

    println!("#### Example Anthropic Image Data ####");
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