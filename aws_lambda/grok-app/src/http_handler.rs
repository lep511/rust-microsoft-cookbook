mod system_bot;
mod grok_api;
use grok_api::get_grok_response;

use lambda_runtime::{tracing, Error, LambdaEvent};
use serde::Deserialize;
use serde_json::Value;
use std::env;

#[derive(Debug, Deserialize)]
struct MessageBody {
    message: MessageData,
}

#[derive(Debug, Deserialize)]
struct MessageData {
    message_id: i64,
    from: UserData,
    // chat: Chat,
    date: i64,
    text: Option<String>,
    photo: Option<Vec<Photo>>,
    location: Option<Location>,
}

#[derive(Debug, Deserialize)]
struct Photo {
    file_id: String,
    file_unique_id: String,
    file_size: i64,
    width: i64,
    height: i64,
    caption: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UserData {
    id: i32,
    is_bot: bool,
    first_name: String,
    language_code: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: f64,
    longitude: f64,
}

pub(crate)async fn function_handler(event: LambdaEvent<Value>) -> Result<(), Error> {
    let payload = event.payload;
    let payload_body = payload["body"].as_str().unwrap_or("no content");
    println!("Payload: {:?}", payload_body);

    if payload_body == "no content" {
        println!("[ERROR] Body is empty");
        return Ok(());
    };

    let body_data: MessageBody = match serde_json::from_str(payload_body) {
        Ok(update) => update,
        Err(e) => {
            println!("[ERROR] Error parsing JSON: {}", e);
            return Ok(());
        }
    };

    let telegram_bot_token = match env::var("TELEGRAM_BOT_TOKEN")  {
        Ok(token) => token,
        Err(e) => {
            println!("[ERROR] Error getting environment variable TELEGRAM_BOT_TOKEN: {}", e);
            return Ok(());
        }
    };

    let xai_api_key = match env::var("XAI_API_KEY") {
        Ok(val) => val,
        Err(e) => {
            println!("[ERROR] Error getting environment variable XAI_API_KEY: {}", e);
            return Ok(());
        }
    };

    let telegram_chat_id = body_data.message.from.id;
    // let user_id = chat_id.to_string();

    // Verify type of message
    match (body_data.message.photo.is_some(), body_data.message.text.as_ref()) {
        (true, _) => {
            println!("Photo received");
            let is_image = true;
            // Send image_id as prompt
            let prompt = body_data.message.photo.unwrap().last().unwrap().file_id.clone();
            let message: String = match get_grok_response(
                prompt,
                xai_api_key,
                telegram_bot_token,
                telegram_chat_id.to_string(),
                is_image,
            ).await {
                Ok(response) => response,
                Err(e) => {
                    println!("[ERROR]: {}", e);
                    return Ok(());
                }
            };
            return Ok(());
        }
        (false, Some(text)) => {
            println!("Text received.");
            let is_image = false;
            let prompt = text.to_string();
            let message: String = match get_grok_response(
                prompt,
                xai_api_key,
                telegram_bot_token,
                telegram_chat_id.to_string(),
                is_image,   
            ).await {
                Ok(response) => response,
                Err(e) => {
                    println!("[ERROR]: {}", e);
                    return Ok(());
                }
            };
            println!("{}", message);
        }
        (false, None) => {
            println!("[ERROR] Unknown type of input.");
        }
    };
    
    Ok(())
}