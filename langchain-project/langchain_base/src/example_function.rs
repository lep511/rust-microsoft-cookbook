mod anthropic;
use anthropic::ChatAnthropic;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: MainWeather,
    name: String,
}

#[derive(Debug, Deserialize)]
struct MainWeather {
    temp: f64,
    feels_like: f64,
    humidity: i32,
    pressure: i32,
}

#[derive(Debug, Deserialize)]
struct WeatherParams {
    location: String,
    unit: String,
}

async fn get_weather(params: WeatherParams) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    println!("Getting weather for {} in {}", params.location, params.unit);

    let api_key = env::var("OPENWEATHER_API_KEY")
    .expect("OPENWEATHER_API_KEY environment variable not set");
    
    let units = match params.unit.as_str() {
        "celsius" => "metric",
        "fahrenheit" => "imperial",
        _ => "metric", // default to metric
    };

    println!("Location: {}", params.location);

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units={}&appid={}",
        params.location, units, api_key
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

    match serde_json::from_value::<WeatherResponse>(response) {
        Ok(weather) => Ok(weather),
        Err(e) => Err(Box::new(e)),
    }
}

async fn sample_anthropic_function_gw() -> Result<(), Box<dyn std::error::Error>> {
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
    let tool_choice = json!({"type": "tool", "name": "get_weather"});
    let llm = llm.with_tools(tools, tool_choice);

    let prompt = "What is the weather like in Montevideo in C°?";
    let response = llm.invoke(prompt).await?;

    // println!("Response: {:?}", response);

    match response.content {
        Some(candidates) => {
            for candidate in candidates {
                println!("Response: {:?}", candidate);
                let function_input = candidate.input.clone();
                match candidate.input {
                    Some(input) => {
                        if let Ok(params) = serde_json::from_value::<WeatherParams>(input) {
                            match get_weather(params).await {
                                Ok(weather) => {
                                    println!("Weather {}", weather.name);
                                    println!("Weather {:?}", weather.main);
                                }
                                Err(e) => println!("Error fetching weather: {}", e),
                            }
                        }
                    }
                    None => {
                        println!("No input");
                    }
                };
            }
        }
        None => {
            println!("No response");
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    let _ = sample_anthropic_function_gw().await;
}