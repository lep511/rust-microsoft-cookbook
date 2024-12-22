use crate::openai::ChatOpenAI;

pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatOpenAI::new("gpt-4o-mini")?;
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);
    let llm = llm.with_timeout_sec(30);

    let prompt = "Explain the Pythagorean theorem to a 10-year-old.";
    let response = llm.invoke(prompt).await?;

    println!("\n#### Example OpenAI simple shot ####");
    // if let Some(candidates) = response.choices {
    //     for candidate in candidates {
    //         if let message = candidate.message {
    //             println!("{}", message.content);
    //         }
    //     }
    // };
    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                if let message = candidate.message {
                    println!("{}", message.content);
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}