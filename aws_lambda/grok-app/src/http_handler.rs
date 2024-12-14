mod system_bot;
mod grok_api;
use grok_api::get_grok_response;

use lambda_runtime::{tracing, Error, LambdaEvent};
use serde::Deserialize;
use serde_json::Value;
use tracing::{info, error};
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
    text: String,
}

#[derive(Debug, Deserialize)]
struct UserData {
    id: i32,
    is_bot: bool,
    first_name: String,
    language_code: String,
}

pub(crate)async fn function_handler(event: LambdaEvent<Value>) -> Result<(), Error> {
    let payload = event.payload;
    let payload_body = payload["body"].as_str().unwrap_or("no content");
    info!("Payload: {:?}", payload_body);

    if payload_body == "no content" {
        error!("[ERROR] Body is empty");
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

    let prompt = body_data.message.text;
    let telegram_chat_id = body_data.message.from.id;
    // let user_id = chat_id.to_string();
    
    let message: String = match get_grok_response(
        prompt.to_string(),
        xai_api_key,
        telegram_bot_token,
        telegram_chat_id.to_string(),       
    ).await {
        Ok(response) => response,
        Err(e) => {
            error!("[ERROR]: {}", e);
            return Ok(());
        }
    };

    println!("{}", message);

    Ok(())
}