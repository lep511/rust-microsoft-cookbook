#[allow(dead_code)]
use reqwest;
use langchain::compatible::chat::ChatCompatible;
use langchain::compatible::libs::ChatResponse;
use serde_json::{json, Value, Map};

const BASE_URL: &str = "https://api.open-meteo.com/v1";

async fn get_weather(latitude: f64, longitude: f64) -> Result<Value, Box<dyn std::error::Error>> {
    // Build the URL with formatted parameters
    let url = format!(
        "{}/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m",
        BASE_URL,
        latitude, 
        longitude,
    );

    // Make the GET request
    let response = reqwest::get(&url).await?;
    
    // Parse response to JSON
    let data: Value = response.json().await?;
    
    Ok(data)
}

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
            "name": "get_weather",
            "description": "Get current temperature for provided coordinates in celsius.",
            "parameters": {
                "type": "object",
                "properties": {
                    "latitude": {"type": "number"},
                    "longitude": {"type": "number"}
                },
                "required": ["latitude", "longitude"],
            }
        }
    });

    let tools = vec![weather_function];
    let prompt = "What's the weather like in Paris today?";
    let tool_choice = json!({"type": "function", "function": {"name": "get_weather"}});

    let response: ChatResponse = llm
        .clone()
        .with_max_retries(0)
        .with_tools(tools)
        .with_tool_choice(tool_choice)
        .with_system_prompt(system_prompt)
        .invoke(prompt)
        .await?;

    let mut latitude: f64 = 0.0;
    let mut longitude: f64 = 0.0;
   
     if let Some(choices) = &response.choices {
          for choice in choices {
             if let Some(message) = &choice.message {
                if let Some(tool_calls) = &message.tool_calls {
                      for tool_call in tool_calls {
                        if let Some(function) = tool_call.get("function") {

                            println!("Tool call: {:?}", function);
                            
                            if let Some(arguments) = function.get("arguments") {
                                // Parse the JSON string arguments into a serde_json::Value
                                // Convert Option<&str> to Result with error message if None
                                let parsed: Value = serde_json::from_str(arguments.as_str()
                                    .ok_or("Arguments string is None")?)?;
                            
                                // Convert the parsed JSON into a Map (dictionary)
                                // Return error if JSON is not an object
                                let arguments_map: &Map<String, Value> = parsed.as_object()
                                    .ok_or("JSON is not an object")?;

                                println!("Arguments map: {:?}", arguments_map);
                                
                                // Extract the latitude from the arguments map
                                latitude = match arguments_map.get("latitude") {
                                    Some(latitude) => {
                                        if let Some(latitude_value) = latitude.as_f64() {
                                            latitude_value
                                        } else {
                                            return Err("Latitude is not a number".into());
                                        }
                                    }
                                    // If location key doesn't exist, return empty string
                                    None => 0.0,
                                };

                                // Extract the longitude from the arguments map
                                longitude = match arguments_map.get("longitude") {
                                    Some(longitude) => {
                                        if let Some(longitude_value) = longitude.as_f64() {
                                            longitude_value
                                        } else {
                                            return Err("Longitude is not a number".into());
                                        }
                                    }
                                    // If location key doesn't exist, return empty string
                                    None => 0.0,
                                };
                            } 
                        }
                    }
                }
            }
        }
    };
    
    println!("latitude: {:?}", latitude);
    println!("longitude: {:?}", longitude);

    let weather_data: Value = get_weather(latitude, longitude).await?;
    let weather_data_string = serde_json::to_string(&weather_data)?;

    let prompt_fmt = format!(
        "Based on this information: \n \
        {} \n \
        Answer this question: \n \
        {} \n",
        weather_data_string,
        prompt,
    );

    let response: ChatResponse = llm
        .with_temperature(0.9)
        .with_system_prompt(system_prompt)
        .invoke(&prompt_fmt)
        .await?;

    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let Some(message) = candidate.message {
                    if let Some(content) = message.content {
                        println!("{}", content);
                    }
                }
            }
        }
        None => println!("No response choices available"),
    };

    Ok(())
}