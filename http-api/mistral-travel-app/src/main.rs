use reqwest::{ Client, Body };
use std::error::Error;
use serde::Deserialize;
use serde_json::json;
use std::{ thread, time, env };

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct TravelRoute {
    origin_city: String,
    origin_code: String,
    destination_city: String,
    destination_code: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ChatResponse {
    object: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Choice {
    index: i32,
    message: Message,
    finish_reason: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Message {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCall>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ToolCall {
    id: String,
    #[serde(rename = "type")]
    call_type: String,
    function: FunctionCall,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct FunctionCall {
    name: String,
    arguments: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: i32,
    total_tokens: i32,
    completion_tokens: i32,
}

async fn generate_content(messages: serde_json::Value, tools: serde_json::Value) -> Result<ChatResponse, Box<dyn Error>> {
    // Get API key from environment variable
    let api_key = env::var("MISTRAL_API_KEY")
        .expect("MISTRAL_API_KEY environment variable is not set");

    let url = "https://api.mistral.ai/v1/chat/completions".to_string();
        
    // Prepare the request body
    let request_body = json!({
        "messages": messages,
        "model": "mistral-small-latest",
        "stream": false,
        "tools": tools
    });

    let request_body = serde_json::to_string(&request_body)?;
    let body: Body = Body::wrap(request_body);
    
    // Create a reqwest client
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;  

    // Send the POST request
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(body)
        .send()
        .await?;
    
    // Print status code
    // println!("Status: {}", response.status());

    // Read the response body
    let body_str: String = response.text().await?;

    // Parse and print the response
    // println!("Response: {}", body_str);

    match serde_json::from_str::<ChatResponse>(&body_str) {
        Ok(chat_response) => return Ok(chat_response),
        Err(e) => {
            println!("Error parsing JSON: {}", e);
            return Err(Box::new(e));
        }
    }
}

#[tokio::main]
async fn main() {
    let question = "When is the next flight from Rio de Janeiro to Seattle?";
    // let question = "When is the next flight from Miami to Seattle?";
   
    let messages: serde_json::Value = json!([
        {
            "role": "system",
            "content": "You are a helpful assistant that help users to find information about traveling, how to get to places and the different transportations options. You care about the environment and you always have that in mind when answering inqueries"
        },
        {
            "role": "user",
            "content": question
        }
    ]);

    let tools: serde_json::Value = json!([
        {
            "type": "function",
            "function": {
                "name": "get_flight_info",
                "description": "Returns information about the next flight between two cities. This includes the name of the airline, flight number and the date and time of the next flight",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "origin_city": {
                            "type": "string",
                            "description": "The name of the city where the flight originates"
                        },
                        "origin_code": {
                            "type": "string",
                            "description": "The IATA code of the city where the flight originates"
                        },
                        "destination_city": {
                            "type": "string",
                            "description": "The flight destination city"
                        },
                        "destination_code": {
                            "type": "string",
                            "description": "The IATA code of the flight destination city"
                        }
                    },
                    "required": [
                        "origin_city",
                        "origin_code",
                        "destination_city",
                        "destination_code"
                    ],
                },
            },
        }
    ]);

    let mut function_name = String::from("none_select");
    println!("Question: {}", question);

    match generate_content(messages, tools).await {
        Ok(response) => {
            if let Some(first_choice) = response.choices.first() {
                if first_choice.message.content.len() != 0 {
                    println!("Message content: {}", first_choice.message.content);
                };
                if let Some(calls) = &first_choice.message.tool_calls {
                    if let Some(first_call) = calls.first() {
                        function_name = first_call.function.name.clone();
                        let function_arguments = first_call.function.arguments.as_str();
                        let route: TravelRoute = serde_json::from_str(function_arguments).unwrap();
                        println!("Route: {:?}", route);
                    }
                }
            };
            println!("Total tokens: {}", response.usage.total_tokens);
        }
        Err(e) => {
            println!("Error sending request: {}", e);
            return;
        }
    }

    println!("Function name: {}", function_name);

    // Wait two seconds to avoid error 422
    let two_seconds = time::Duration::from_secs(2);
    thread::sleep(two_seconds);

    // Check FUNCTIONS
}