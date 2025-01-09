#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use langchain::gemini::libs::Part;

#[derive(Debug)]
struct TestCase<'a> {
    text: &'a str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-1.5-pro-latest")?;
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);

    let llm = llm.with_system_prompt("You are a helpful assistant.");

    let test_cases = vec![
        TestCase {
            text: "Identify all brand names mentioned in the input. Multiple products will be separated by commas.",
        },
        TestCase {
            text: "input: What did TSLA, GOOG and DIS do today?",
        },
        TestCase {
            text: "output: Tesla, Google, Walt Disney Co",
        },
        TestCase {
            text: "input: Tick-tock goes the clock for Rolex",
        },
        TestCase {
            text: "output: Rolext",
        },
        TestCase {
            text: "input: Three stocks to watch",
        },
        TestCase {
            text: "output: (none)",
        },
        TestCase {
            text: "input: Reebok pumps are back in vogue this season",
        },
        TestCase {
            text: "output: Reebok",
        },
    ];
    
    let mut parts: Vec<Part> = Vec::new();
    for test_case in test_cases {
        let part = Part {
            text: Some(test_case.text.to_string()),
            function_call: None,
            inline_data: None,
            file_data: None,
        };
        parts.push(part);
    }

    let llm = llm.with_multiple_parts(parts);
    let prompt = "Peep the latest dog trends in vogue mag and NY times";
    let response = llm.clone().invoke(prompt).await?;

    let mut response_model = String::new();

    println!("\n#### Example Gemini Find Brands ####");
    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                        response_model = text.to_string();
                    }
                }
            }
        }
    };

    let chat_history = match response.chat_history {
        Some(chat_history) => chat_history,
        None => {
            println!("No chat history");
            Vec::new()
        }
    };

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;
    let llm = llm.with_chat_history(chat_history);
    let llm = llm.with_assistant_response(&response_model);

    let prompt = "NY:GOOG, GOOGL, Google stocks to watch";
    let response = llm.invoke(prompt).await?;

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