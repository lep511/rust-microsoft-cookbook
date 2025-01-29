#[allow(dead_code)]
use langchain::anthropic::chat::ChatAnthropic;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;

#[allow(dead_code)]
pub async fn get_weather(location: &str, unit: &str) -> Result<String, Box<dyn std::error::Error>> {

    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY not set");
    
    let unit_format: &str;

    if unit == "celsius" {
        unit_format = "metric";
    } else {
        unit_format = "imperial";
    }
    
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units={}&appid={}",
        location, 
        unit_format, 
        api_key
    );

    let client = Client::builder()
                    .use_rustls_tls()
                    .build()?;

    let response = client.
        get(url)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let response_string = serde_json::to_string(&response)?;
    Ok(response_string)
}

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
                    "description":"The city and state, e.g. San Francisco"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "The unit of temperature, either 'celsius' or 'fahrenheit'"
                }
            },
            "required":[
                "location"
            ]
        }
    });
    let tools = vec![tool_data];
    let tool_choice = Some(json!({"type": "tool", "name": "get_weather"}));
    let prompt = "What is the weather like in Montevideo in C°?";

    println!("\n\nQuestion: {}", prompt);

    let response = llm
        .clone()
        .with_tools(
            tools.clone(), 
            tool_choice,
        )
        .with_max_retries(0)
        .invoke(prompt)
        .await?;

    // println!("Response: {:?}", response);

    let mut tool_id = String::from("");
    let mut input_param = Value::Null;
    
    if let Some(candidates) = &response.content {
        for candidate in candidates {
            if candidate.content_type == "tool_use" {
                tool_id = candidate.id.clone().unwrap();
                input_param = candidate.input.clone().unwrap();
            }
        }
    };

    let history = response.chat_history.unwrap_or_else(|| {
        panic!("No chat history found");
    });

    let contents = response.content.unwrap_or_else(|| {
        panic!("No content found");
    });

    let location = input_param.get("location")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let unit = input_param.get("unit")
        .and_then(|v| v.as_str())
        .unwrap_or("celsius");

    let result = get_weather(location, unit).await?;
    let tool_choice = None;

    let response = llm
        .with_chat_history(history)
        .with_tools(tools, tool_choice)
        .with_assistant_content(contents)
        .with_max_retries(0)
        .with_tool_result(
            &tool_id,
            &result,
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