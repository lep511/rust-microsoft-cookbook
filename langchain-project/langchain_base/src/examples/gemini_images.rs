#[allow(dead_code)]
use crate::gemini::ChatGemini;
// use std::fs::File;
// use std::io::Read;
// use base64::{Engine as _, engine::general_purpose::STANDARD};

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    // // Read first image into a byte vector
    // let mut file_01 = File::open("src/examples/files/image01.jpg")?;
    // let mut buffer_01 = Vec::new();
    // file_01.read_to_end(&mut buffer_01)?;
    
    // // Read second image into a byte vector
    // let mut file_02 = File::open("src/examples/files/image03.png")?;
    // let mut buffer_02 = Vec::new();
    // file_02.read_to_end(&mut buffer_02)?;

    // // Convert to base64
    // let base64_string_01 = STANDARD.encode(&buffer_01);
    // let base64_string_02 = STANDARD.encode(&buffer_02);

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let file_path = "src/examples/files/image03.png";
    let file_mime = "image/png";
    
    let file_url = llm.clone().media_upload(file_path, file_mime).await?;

    println!("file_url: {}", file_url);
    
    // let llm = llm.with_image(
    //     &base64_string_01,
    //     "image/jpeg",
    // );
    // let llm = llm.with_image(
    //     &base64_string_02,
    //     "image/png",
    // );
    // let prompt = "Compare the two pictures provided";

    let llm = llm.with_file_url(
        file_url,
        file_mime,
    );
    let prompt = "Can you describe this photo?";
    let response = llm.invoke(prompt).await?;

    println!("\n#### Example OpenAI images ####");
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