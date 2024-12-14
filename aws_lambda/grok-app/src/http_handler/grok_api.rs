use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use super::system_bot::guideline_bot;
use serde_json::json;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

static LANGUAGE_MODEL: &str = "grok-2-1212";
const MAX_TOKENS_STREAM: u8 = 12;

#[derive(Serialize)]
pub struct ChatCompletionRequest {
    messages: Vec<Message>,
    model: String,
    stream: bool,
    temperature: f64,
}

#[derive(Serialize)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct StreamResponse {
    choices: Vec<StreamChoice>,
}

#[derive(Deserialize, Debug)]
struct StreamChoice {
    delta: StreamDelta,
}

#[derive(Deserialize, Debug)]
struct StreamDelta {
    content: Option<String>,
}

pub async fn get_grok_response(
    prompt: String, 
    xai_api_key: String, 
    telegram_bot_token: String, 
    telegram_chat_id: String
) -> Result<String, Box<dyn std::error::Error>> {

    let client = Client::new();
    let telegram_client = Arc::new(Client::new());

    // Get system data from system_bot file
    let system_data = guideline_bot().expect("Failed to load system data");

    let request_body = ChatCompletionRequest {
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_data,
            },
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        model: LANGUAGE_MODEL.to_string(),
        stream: true,
        temperature: 0.9,
    };

    let response: Response = match client
        .post("https://api.x.ai/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", xai_api_key))
        .json(&request_body)
        .send()
        .await {
            Ok(response) => {
                if response.status().is_success() {
                    response
                } else {
                    return Err(format!("Request failed with status: {}", response.status()).into());
                }
            },
            Err(e) => return Err(e.into()),
        };  

    if response.status().is_success() {
        let mut stream = response.bytes_stream();
        let current_telegram_message_id = Arc::new(Mutex::new(None));
        let mut complete_text = String::from("");
        let mut counter = 0;

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    let str_chunk = String::from_utf8_lossy(&bytes);
                    let parts: Vec<&str> = str_chunk.split("\n\n").collect();
                    for part in parts {
                        if !part.is_empty() && part.starts_with("data:") {
                            let json_part = part.trim_start_matches("data:");
                            if json_part.trim() == "[DONE]" {
                                send_telegram_message(
                                    telegram_client.clone(),
                                    telegram_bot_token.clone(),
                                    telegram_chat_id.clone(),
                                    complete_text.clone(),
                                    current_telegram_message_id.clone()
                                ).await;
                                let message_response = String::from("Stream completed.");
                                return Ok(message_response);
                            }

                            match serde_json::from_str::<StreamResponse>(json_part) {
                                Ok(stream_response) => {
                                    if let Some(content) = stream_response.choices[0].delta.content.as_ref() {
                                        complete_text = complete_text + content;
                                        if counter > MAX_TOKENS_STREAM {
                                            send_telegram_message(
                                                telegram_client.clone(),
                                                telegram_bot_token.clone(),
                                                telegram_chat_id.clone(),
                                                complete_text.clone(),
                                                current_telegram_message_id.clone()
                                            ).await;
                                            counter = 0;
                                        } else {
                                            counter += 1;
                                        }
                                    }
                                },
                                Err(e) => eprintln!("Error parsing JSON chunk: {}", e),
                            }
                        }
                    }
                },
                Err(e) => eprintln!("Error receiving chunk: {}", e),
            }
        }
        println!();
    }
    else {
        let error_body = response.text().await.expect("Failed to read error body");
        println!("API Error: {}", error_body);
    }

    let message_response = String::from("Message sent successfully");
    Ok(message_response)
}

async fn send_telegram_message(
    client: Arc<Client>, 
    bot_token: String, 
    chat_id: String, 
    text: String, 
    current_telegram_message_id: Arc<Mutex<Option<i64>>>
) {
    let telegram_api_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let mut message_body = json!({
        "chat_id": chat_id,
        "text": text,
        "parse_mode": "markdown"
    });

    let current_message_id = *current_telegram_message_id.lock().await;
    
    if let Some(message_id) = current_message_id {
        let edit_message_url = format!("https://api.telegram.org/bot{}/editMessageText", bot_token);
        message_body = json!({
            "chat_id": chat_id,
            "text": text,
            "message_id": message_id,
            "parse_mode": "markdown"
        });
            
        let response = client
            .post(edit_message_url)
            .header("Content-Type", "application/json")
            .json(&message_body)
            .send()
            .await;
    
        // match response {
        //     Ok(res) => {
        //         if !res.status().is_success(){
        //             let err_body = res.text().await.unwrap_or_else(|_| String::from("Error Body couldn't be read"));
        //             eprintln!("Failed to edit message to telegram: {}", err_body);
        //         }
        //     },
        //     Err(e) => eprintln!("Error editing message on Telegram: {}", e),
        // }
    }else {
        let response = client
            .post(telegram_api_url)
            .header("Content-Type", "application/json")
            .json(&message_body)
            .send()
            .await;
        match response {
            Ok(res) => {
                if res.status().is_success(){
                    let response_body: serde_json::Value = res.json().await.unwrap();
                    let message_id = response_body["result"]["message_id"].as_i64().unwrap();
                    *current_telegram_message_id.lock().await = Some(message_id);
                }
            },
            Err(e) => eprintln!("Error sending message to Telegram: {}", e),
        }
    }
}