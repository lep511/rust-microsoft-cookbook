use reqwest::{ Client, Body };
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::File;

pub async fn get_gemini_response(
    prompt: &str, 
    google_api_token: String
    ) -> Result<String, Box<dyn std::error::Error>> {
    
    println!("Api token: {}", google_api_token);
    // Construct the URL with the API key
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
        google_api_token
    );

    let tool_json = json!({
        "function_declarations": [
            {
                "name": "lighting",
                "description": "Turn lights on and off, and set the color",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "description": "The action to perform, either 'on' or 'off'",
                            "enum": ["on", "off"]
                        },
                        "color": {
                            "type": "string",
                            "description": "The light color as a 6-digit hex string, e.g. ff0000 for red.",
                        }
                    },
                    "required": ["action", "color"]
                }
            }
        ]
    });

    // Prepare the request body
    let request_body = json!({
        "system_instruction": {
            "parts": {"text": "You are a helpful lighting system bot. You can turn lights on and off, and you can set the color. Do not perform any other tasks."}
        },
        "tools": [tool_json],
        "tool_config": {
            "function_calling_config": {"mode": "none"}
        },
        "contents": {
            "role": "user",
            "parts": {
                "text": "Turn the light off"
            }
        }
    });

    // Create a reqwest client
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;  

    // Send the POST request
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    // Print status code
    println!("Status: {}", response.status());

    // Print headers if needed
    // println!("Headers: {:#?}", response.headers());

    if response.status().as_u16() > 299 {
        println!("Error: {}", response.status());
        println!("Response: {:#?}", response);
        return Err("Error in Gemini API".into());
    }

    let result: Value = response.json().await?;
    println!("{:#?}", result);

    // Read the response body
    // let body_str: String = response.text().await?;
    let body_str: String = String::from("test");

    Ok(body_str)
}