#[allow(dead_code)]
use crate::gemini::ChatGemini;

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-1.5-flash")?;
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);

    let prompt = "Explain the Pythagorean theorem to a 10-year-old.";
    let response = llm.invoke(prompt).await?;

    println!("\n#### Example Gemini simple shot ####");
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