mod system_bot;
mod grok_api;
use grok_api::{get_grok_response, telegram_get_file_data};
use lambda_runtime::{tracing, Error, LambdaEvent};
use serde::Deserialize;
use serde_json::Value;
use std::env;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct MessageBody {
    message: MessageData,
}

#[derive(Debug, Deserialize, Clone)]
struct MessageData {
    message_id: i64,
    from: UserData,
    // chat: Chat,
    date: i64,
    text: Option<String>,
    photo: Option<Vec<Photo>>,
    location: Option<Location>,
    voice: Option<Voice>,
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

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Location {
    latitude: f64,
    longitude: f64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Voice {
    file_id: String,
    file_unique_id: String,
    duration: i64,
    mime_type: String,
    file_size: i64,
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

    let xai_api_key = match env::var("XAI_API_KEY") {
        Ok(val) => val,
        Err(e) => {
            println!("[ERROR] Error getting environment variable XAI_API_KEY: {}", e);
            return Ok(());
        }
    };

    let telegram_chat_id = body_data.message.from.id;
    let message = body_data.message.clone();
    // let user_id = chat_id.to_string();

    // Verify type of message
    if message.photo.is_some() {
        println!("Photo received");
        let prompt = message.photo.unwrap().last().unwrap().file_id.clone();
        let is_image = true;

        match get_grok_response(
            prompt,
            xai_api_key,
            telegram_bot_token,
            telegram_chat_id.to_string(),
            is_image,
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
    
        match get_grok_response(
            prompt,
            xai_api_key,
            telegram_bot_token,
            telegram_chat_id.to_string(),
            is_image,
        ).await {
            Ok(response) => {
                println!("Text processed: {}", response);
            }
            Err(e) => {
                println!("[ERROR]: {}", e);
            }
        }
    } else if message.location.is_some() {
        println!("Location received");
    } else if message.voice.is_some() {
        println!("Voice received");
        let file_id = message.voice.as_ref().unwrap().file_id.clone();
        let file_data = match telegram_get_file_data(
            file_id,
            telegram_bot_token,
        ).await {
            Ok(url) => url,
            Err(e) => {
                println!("[ERROR]: {}", e);
                return Ok(());
            }
        };
    } else {
        println!("[ERROR] Unknown type of input.");
        return Ok(());
    };

    Ok(())
}