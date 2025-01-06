#[allow(dead_code)]
use langchain_base::gemini::ChatGemini;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let response = llm
        .with_temperature(0.2)
        .with_max_tokens(100)
        .with_top_k(20)
        .with_top_p(0.95)
        .with_candidate_count(1)
        .with_stop_sequences(vec!["STOP!".to_string()])
        .with_system_prompt("You are a helpful assistant.")
        .invoke("Tell me how the internet works, but pretend I'm a puppy who only understands squeaky toys.")
        .await?;

    // println!("{:?}", response);

    let mut n = 1;
    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            println!("\n\nCandidate: {}", n);
            n += 1;
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