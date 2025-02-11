#[allow(dead_code)]
use langchain::openai::chat::ChatOpenAI;
use langchain::openai::libs::ChatResponse;
use serde_json::json;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatOpenAI::new("gpt-4o-mini");

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

    match response.choices {
        Some(candidates) => {
            candidates.iter()
                .filter_map(|candidate| {
                    candidate.message.as_ref().and_then(|msg| 
                        if let Some(tool_calls) = &msg.tool_calls {
                            Some(tool_calls.iter().for_each(|call| {
                                if let Some(func) = call.get("function") {
                                    if let Some(name) = func.get("name") {
                                        println!("Function name: {}", name);
                                    }
                                    if let Some(args) = func.get("arguments") {
                                        println!("Arguments: {}", args);
                                    }
                                }
                            }))
                        } else {
                            msg.content.as_ref().map(|content| println!("{}", content))
                        }
                    )
                })
                .count();
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}