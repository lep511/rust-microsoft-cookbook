#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use langchain::compatible::libs::ChatResponse;
use serde_json::{json};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://api.x.ai/v1/chat/completions";
    let model = "grok-2-latest";
    let llm = ChatCompatible::new(base_url, model)?;

    let system_prompt = "Don't make assumptions about what values to plug into functions. \
                        Ask for clarification if a user request is ambiguous.";

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

    let tools = vec![weather_function];
    let prompt = "What is the weather like in Boston today?";
    let tool_choice = json!({"type": "function", "function": {"name": "get_current_weather"}});

    let response: ChatResponse = llm
        .with_retry(0)
        .with_tools(tools)
        .with_tool_choice(tool_choice)
        .with_system_prompt(system_prompt)
        .invoke(prompt)
        .await?;

    let mut location = "";
    
    if let Some(choices) = &response.choices {
        for choice in choices {
            if choice.finish_reason == "tool_calls" {
                println!("Tool use: {:?}", choice.message.tool_calls);
            }
        }
    };
    // println!("Response: {:?}", response);
    
    Ok(())
}