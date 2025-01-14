use lambda_runtime::{tracing, Error, LambdaEvent};
use gemini_api::get_gemini_response;
use libs::{Part, Content};
use serde::Deserialize;
use serde_json::Value;
use std::env;

pub mod gemini_api;
pub mod chat;
pub mod embed;
pub mod gen_config;
pub mod libs;
pub mod utils;
pub mod requests;
pub mod llmerror;
pub mod errors;

pub static GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
pub static UPLOAD_BASE_URL: &str = "https://generativelanguage.googleapis.com/upload/v1beta";

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct MessageBody {
    message: MessageData,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct MessageData {
    message_id: i64,
    from: UserData,
    // chat: Chat,
    date: i64,
    text: Option<String>,
    photo: Option<Vec<Photo>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Photo {
    file_id: String,
    file_unique_id: String,
    file_size: i64,
    width: i64,
    height: i64,
    caption: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct UserData {
    id: i32,
    is_bot: bool,
    first_name: String,
    language_code: String,
}

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

    let telegram_chat_id = body_data.message.from.id;
    let message = body_data.message.clone();
    // let user_id = chat_id.to_string();
    
    // Verify type of message
    if message.photo.is_some() {
        println!("Photo received");
        println!("Photo: {:?}", message.photo);
        let prompt = message.photo.unwrap().last().unwrap().file_id.clone();
        let is_image = true;

        match get_gemini_response(
            prompt,
            telegram_bot_token,
            telegram_chat_id.to_string(),
            is_image,
            bucket_name,
        ).await {
            Ok(response) => {
                println!("Photo processed: {}", response);
            }
            Err(e) => {
                println!("[ERROR]: {}", e);
            }
        }
    } else if message.text.is_some() {
        println!("Text received");
        let is_image = false;
        let prompt = message.text.as_ref()
            .ok_or("Text message is empty")?
            .to_string();
    
        match get_gemini_response(
            prompt,
            telegram_bot_token,
            telegram_chat_id.to_string(),
            is_image,
            bucket_name,
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
