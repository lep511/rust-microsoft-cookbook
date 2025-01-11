#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let response = llm
        .stream_invoke("Tell me how the internet works, but pretend I'm a puppy who only understands squeaky toys.")
        .await?;

    println!("{}", response);

    // if let Some(candidates) = response.candidates {
    //     for candidate in candidates {
    //         if let Some(content) = candidate.content {
    //             for part in content.parts {
    //                 if let Some(text) = part.text {
    //                     println!("{}", text);
    //                 }
    //             }
    //         }
    //     }
    // };

    Ok(())
}