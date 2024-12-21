mod openai;
use openai::ChatOpenAI;
// mod gemini;
// use gemini::ChatGemini;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = ChatOpenAI::new("gpt-4o-mini")?;

    let model = model.with_max_tokens(1024);
    let model = model.with_temperature(0.9);
    let model = model.with_max_tokens(2048);

    let prompt = "Explain the Pythagorean theorem to a 10-year-old.";
    let response = model.invoke(prompt).await?;
    println!("Response: {:?}", response);

    
    // let model = ChatGemini::new("gemini-1.5-flash")?;
    
    // let model = model.with_temperature(0.9);
    // let model = model.with_max_tokens(2048);

    // let prompt = "Explain the Pythagorean theorem to a 10-year-old.";
    // let response = model.invoke(prompt).await?;

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
