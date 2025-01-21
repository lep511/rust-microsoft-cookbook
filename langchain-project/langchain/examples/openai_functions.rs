#[allow(dead_code)]
use langchain::openai::chat::ChatOpenAI;
use langchain::openai::libs::ChatResponse;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatOpenAI::new("gpt-4o-mini")?;

    let prompt = "What is the weather like in Boston today?";

    let system_promp = "Don't make assumptions about what values to plug into functions. Ask for clarification if a user request is ambiguous.";

    let weather_function = json!( {
        "type": "function",
        "function": {
            "name": "get_current_weather",
            "description": "Get the current weather in a given location",
            "parameters": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"],
                        "default": "celsius"
                    }
                },
                "required": ["location"]
            }
        }
    });
    
    let tool_choice = json!({"type": "function", "function": {"name": "get_current_weather"}});

    let response: ChatResponse = llm
        .with_system_prompt(system_promp)
        .with_tools(vec![weather_function])
        .with_tool_choice(tool_choice)
        .invoke(prompt)
        .await?;

    println!("\n#### Example OpenAI functions ####");
    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let message = candidate.message {
                    println!("{:?}", message.tool_calls);
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}