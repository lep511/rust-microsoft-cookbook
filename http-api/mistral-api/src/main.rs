use reqwest::{ Client, Body };
use csv::ReaderBuilder;
use std::error::Error;
use serde::Deserialize;
use serde_json::json;
use std::{ thread, time, env };

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CsvData {
    transaction_id: String,
    customer_id: String,
    payment_amount: f64,
    payment_date: String,
    payment_status: String,
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

async fn read_csv(file_path: &str) -> Result<Vec<CsvData>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut data = Vec::new();

    for result in rdr.deserialize() {
        let record: CsvData = result?;
        data.push(record);
    }

    Ok(data)
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
    println!("Status: {}", response.status());

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
    let data_csv = read_csv("data.csv").await.map_err(|e| {
        eprintln!("Error reading CSV: {}", e);
        e
    });
   
    // println!("Data from CSV: {:?}", data_csv);

    let messages: serde_json::Value = json!([
        {
            "role": "user",
            "content": "What's the status of my transaction T1001?"
        }
    ]);

    let tools: serde_json::Value = json!([
        {
            "type": "function",
            "function": {
                "name": "retrieve_payment_status",
                "description": "Get payment status of a transaction",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "transaction_id": {
                            "type": "string",
                            "description": "The transaction id.",
                        }
                    },
                    "required": ["transaction_id"],
                },
            },
        },
        {
            "type": "function",
            "function": {
                "name": "retrieve_payment_date",
                "description": "Get payment date of a transaction",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "transaction_id": {
                            "type": "string",
                            "description": "The transaction id.",
                        }
                    },
                    "required": ["transaction_id"],
                },
            },
        }
    ]);

    match generate_content(messages, tools).await {
        Ok(response) => {
            if let Some(first_choice) = response.choices.first() {
                if first_choice.message.content.len() != 0 {
                    println!("Message content: {}", first_choice.message.content);
                };
                if let Some(calls) = &first_choice.message.tool_calls {
                    if let Some(first_call) = calls.first() {
                        let function_name = &first_call.function.name;
                        let function_arguments = &first_call.function.arguments;
                        
                        println!("Function name: {}", function_name);
                        println!("Arguments: {}", function_arguments);
                    }
                }
            };
            println!("Total tokens: {}", response.usage.total_tokens);
        }
        Err(e) => eprintln!("Error sending request: {}", e),
    }

    let messages: serde_json::Value = json!([
        {
            "role": "user",
            "content": "Who is the best French painter? Answer in one short sentence."
        }
    ]);

    let tools: serde_json::Value = json!([]);

    // Wait two seconds to avoid error 422
    let two_seconds = time::Duration::from_secs(2);
    thread::sleep(two_seconds);

    match generate_content(messages, tools).await {
        Ok(response) => {
            if let Some(first_choice) = response.choices.first() {
                println!("Message content: {}", first_choice.message.content);
            };
            println!("Total tokens: {}", response.usage.total_tokens);
        }
        Err(e) => eprintln!("Error sending request: {}", e),
    }
}