#[allow(dead_code)]
use langchain_base::openai::{ChatOpenAI, ChatResponse};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatOpenAI::new("gpt-4o-mini")?;
    let llm = llm.with_system_prompt("Don't make assumptions about what values to plug into functions. Ask for clarification if a user request is ambiguous.");

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

    let llm = llm.with_tools(vec![weather_function]);

    // let tool_choice = json!({"type": "function", "function": {"name": "get_current_weather"}});
    // let llm = llm.with_tool_choice(tool_choice);
    
    let prompt = "What is the weather like in Boston today?";
    let response: ChatResponse = llm.invoke(prompt).await?;


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