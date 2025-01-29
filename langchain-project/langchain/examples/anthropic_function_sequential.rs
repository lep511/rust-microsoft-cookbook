use langchain::anthropic::chat::ChatAnthropic;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let llm = ChatAnthropic::new("claude-3-5-sonnet-20241022")?;

    let tool_data = json!({
        "name":"get_weather",
        "description":"Get the current weather in a given location",
        "input_schema":{
            "type":"object",
            "properties":{
                "location":{
                    "type":"string",
                    "description":"The city and state, e.g. San Francisco, CA"
                },
                "unit":{
                    "type":"string",
                    "enum":[
                        "celsius",
                        "fahrenheit"
                    ],
                    "description":"The unit of temperature, either \"celsius\" or \"fahrenheit\""
                }
            },
            "required":[
                "location"
            ]
        }
    });

    let tools = Some(vec![tool_data]);
    let tool_choice = Some(json!({"type": "tool", "name": "get_weather"}));
    let prompt = "What is the weather like in San Francisco?";

    let response = llm
        .with_tools(
            tools.clone(), 
            tool_choice,
        )
        .with_retry(0)
        .invoke(prompt)
        .await?;

    let mut tool_id = String::from("");
    
    if let Some(candidates) = &response.content {
        for candidate in candidates {
            if candidate.content_type == "tool_use" {
                tool_id = candidate.id.clone().unwrap_or_else(|| {
                    panic!("No tool id found");
                });
            }
        }
    };

    let history = response.chat_history.unwrap_or_else(|| {
        panic!("No chat history found");
    });

    let contents = response.content.unwrap_or_else(|| {
        panic!("No content found");
    });

    println!("Content: {:?}", history);

    // Sample response from the function get_weather
    let response_function = "15 degrees C°";
    let tool_choice = None;

    let llm = ChatAnthropic::new("claude-3-5-sonnet-20241022")?;

    let response = llm
        .with_chat_history(history)
        .with_tools(tools, tool_choice)
        .with_assistant_content(contents)
        .with_max_retries(0)
        .with_tool_result(
            &tool_id,
            response_function,
        )
        .await?;

    if let Some(candidates) = response.content {
        for candidate in candidates {
            match candidate.text {
                Some(text) => println!("{}", text),
                None => println!(""),
            }
        }
    };
        
    Ok(())
}