use lambda_runtime::{tracing, Error, LambdaEvent};
use gemini_api::get_gemini_response;
use libs::{Part, Content};
use telegram_bot::{MessageBody, TelegramMessage};
use serde::Deserialize;
use serde_json::Value;
use std::env;

pub mod gemini_api;
pub mod telegram_bot;
pub mod chat;
pub mod gen_config;
pub mod libs;
pub mod utils;
pub mod requests;
pub mod llmerror;
pub mod errors;

pub static GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
pub static UPLOAD_BASE_URL: &str = "https://generativelanguage.googleapis.com/upload/v1beta";
pub static GEMINI_MODEL: &str = "learnlm-1.5-pro-experimental";

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;

pub(crate)async fn function_handler(event: LambdaEvent<Value>) -> Result<(), Error> {
    let payload = event.payload;
    let payload_body = payload["body"].as_str().unwrap_or("no content");
    println!("Payload: {:?}", payload_body);

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

    let bucket_name = match env::var("BUCKET_NAME")  {
        Ok(bucket_name) => bucket_name,
        Err(e) => {
            println!("[ERROR] Error getting environment variable BUCKET_NAME: {}", e);
            return Ok(());
        }
    };

    let message = match body_data.message {
        Some(message) => message,
        None => {
            println!("[ERROR] No message found in the body");
            return Ok(());
        }
    };

    let telegram_chat_id = match message.chat {
        Some(chat_id) => {
            match chat_id.id {
                Some(chat_id) => chat_id,
                None => {
                    println!("[ERROR] No id found in message.chat");
                    return Ok(());
                }
            }
        }
        None => {
            println!("[ERROR] No message.chat found in the body");
            return Ok(());
        }
    };
    
    // Verify type of message
    if let Some(photo) = message.photo {
        println!("Photo received");

        let default_caption = "Analyze this image in few lines.".to_string();
        let caption = message.caption.unwrap_or(default_caption);
        let file_id = match photo.last() {
            Some(photo) => {
                match &photo.file_id {
                    Some(file_id) => file_id,
                    None => {
                        println!("[ERROR] No file_id found in the body");
                        return Ok(());
                    }
                }
            }
            None => {
                println!("[ERROR] No photo found in the body");
                return Ok(());
            }
        };

        let message_type = TelegramMessage::Image {
            file_id: file_id.to_string(),
            caption: caption,
        };

        match get_gemini_response(
            &message_type,
            &telegram_bot_token,
            telegram_chat_id,
            &bucket_name,
        ).await {
            Ok(response) => {
                println!("Photo processed: {}", response);
            }
            Err(e) => {
                println!("[ERROR]: {}", e);
            }
        }
    } else if let Some(document) = message.document {
        println!("Document received");

        let file_name = document.file_name.unwrap_or("filetemp001".to_string());
        let file_size = document.file_size.unwrap_or(0);
        let default_caption = "Analyze this document in few lines.".to_string();
        let caption = message.caption.unwrap_or(default_caption);

        let file_id = match &document.file_id {
            Some(file_id) => file_id,
            None => {
                println!("[ERROR] No file_id found in the body");
                return Ok(());
            }
        };

        let mime_type = document.mime_type.unwrap_or("application/pdf".to_string());

        let message_type = TelegramMessage::Document {
            file_name: file_name,
            file_size: file_size,
            file_id: file_id.to_string(),
            caption: caption,
            mime_type: mime_type,
        };

        match get_gemini_response(
            &message_type,
            &telegram_bot_token,
            telegram_chat_id,
            &bucket_name,
        ).await {
            Ok(response) => {
                println!("Document processed: {}", response);
            }
            Err(e) => {
                println!("[ERROR]: {}", e);
            }
        }
    } else if let Some(text) = message.text {
        println!("Text received");

        let message_type = TelegramMessage::Text {
            content: text.to_string(),
        };
        
        match get_gemini_response(
            &message_type,
            &telegram_bot_token,
            telegram_chat_id,
            &bucket_name,
        ).await {
            Ok(response) => {
                println!("Text processed: {}", response);
            }
            Err(e) => {
                println!("[ERROR]: {}", e);
            }
        }
    } else {
        println!("[ERROR] Unknown type of input.");
        return Ok(());
    };

    Ok(())
}
