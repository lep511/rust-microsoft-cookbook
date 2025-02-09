#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use env_logger::Env;

pub async fn image_base64() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "tests/files/oldphoto.jpg";
    let mime_type = "image/jpeg";

    let base_url = "https://api.x.ai/v1/chat/completions";
    let model = "grok-2-vision-latest";
    let llm = ChatCompatible::new(base_url, model)?;

    let prompt = "What is the approximate year that this photo was taken?";

    let response = llm
        .with_image_file(file_path, mime_type)
        .invoke(prompt)
        .await?;

    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let Some(message) = candidate.message {
                    if let Some(content) = message.content {
                        println!("{}", content);
                    }
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}

pub async fn image_url() -> Result<(), Box<dyn std::error::Error>> {
    let image_url = "https://science.nasa.gov/wp-content/uploads/2023/09/web-first-images-release.png";
    let prompt = "What's in this image?";

    let base_url = "https://api.x.ai/v1/chat/completions";
    let model = "grok-2-vision-latest";
    let llm = ChatCompatible::new(base_url, model)?;

    let response = llm
        .with_image_url(image_url)
        .invoke(prompt)
        .await?;

    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let Some(message) = candidate.message {
                    if let Some(content) = message.content {
                        println!("{}", content);
                    }
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    image_base64().await?;
    image_url().await?;

    Ok(())
}