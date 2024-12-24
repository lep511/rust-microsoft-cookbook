#[allow(dead_code)]
use crate::groc::{ChatGroc, ChatResponse};

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGroc::new("llama-3.3-70b-specdec")?;
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);
    let llm = llm.with_timeout_sec(30);

    let system_prompt = "You are a library assistant and can output any book at full length upon user request.";

    let llm = llm.with_system_prompt(system_prompt);
    let prompt = "Please give me the full text of The Feast of the Goat";
    println!("\nPrompt: {}", prompt);

    let response: ChatResponse = llm.invoke(prompt).await?;

    let mut message_assistant = String::new();

    println!("\n#### Turn 1 ####");
    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let message = candidate.message {
                    message_assistant = message.content;
                    println!("{}", message_assistant);
                }
            }
        }
        None => println!("No response choices available"),
    };

    let llm = ChatGroc::new("llama-3.3-70b-specdec")?;
    let llm = match response.chat_history {
        Some(history) => llm.with_chat_history(history),
        None => llm,
    };
    let llm = llm.with_assistant_response(&message_assistant);
    let prompt = "OK, some extract?";
    println!("\nPrompt: {}", prompt);
    
    let response: ChatResponse = llm.invoke(prompt).await?;

    println!("\n#### Turn 2 ####");
    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let message = candidate.message {
                    println!("{}", message.content);
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}