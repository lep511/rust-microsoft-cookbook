use reqwest::{Client, Response};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum TelegramMessage {
    Image {
        file_id: String,
        caption: String,
    },
    Text {
        content: String,
    },
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageBody {
    pub update_id: Option<i64>,
    pub message: Option<MessageData>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageData {
    pub message_id: Option<u32>,
    pub from: Option<User>,
    pub chat: Option<Chat>,
    pub date: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<u32>,
    pub is_bot: Option<bool>,
    pub first_name: Option<String>,
    pub language_code: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
    pub id: Option<u32>,
    pub first_name: Option<String>,
    #[serde(rename = "type")]
    pub chat_type: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhotoData {
    pub file_id: Option<String>,
    pub file_unique_id: Option<String>,
    pub file_size: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelegramGetFile {
    pub ok: bool,
    pub result: Option<FileInfo>,
    pub error_code: Option<i32>,
    pub description: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub file_id: String,
    pub file_unique_id: String,
    pub ile_size: u32,
    pub file_path: Option<String>,
}

/// Sends or edits a message on Telegram using the Bot API
///
/// # Arguments
///
/// * `bot_token` - The authentication token for the Telegram bot
/// * `chat_id` - The unique identifier for the target chat
/// * `text` - The text message to be sent/edited (supports Markdown formatting)
/// * `current_telegram_message_id` - Thread-safe reference to the current message ID. If Some(id), 
///    the function will attempt to edit that message. If None, sends a new message.
///
/// # Examples
///
/// ```
/// let message_id = Arc::new(Mutex::new(None));
/// send_telegram_message(
///     "BOT_TOKEN".to_string(),
///     "CHAT_ID".to_string(), 
///     "Hello World!".to_string(),
///     message_id
/// ).await;
/// ```
///
/// # Notes
///
/// - Uses markdown parsing mode for message formatting
/// - Stores the message ID of newly sent messages in the provided Arc<Mutex>
/// - Prints errors to stderr but does not propagate them
pub async fn send_telegram_message(
    bot_token: String, 
    chat_id: String, 
    text: String,
    current_telegram_message_id: Arc<Mutex<Option<i64>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let telegram_client = Arc::new(Client::new());
    let telegram_api_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let text = replace_raw_escapes(&text);
    
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
            
        let response = telegram_client
            .post(edit_message_url)
            .header("Content-Type", "application/json")
            .json(&message_body)
            .send()
            .await;
    
        match response {
            Ok(res) => {
                if !res.status().is_success(){
                    let err_body = res.text().await.unwrap_or_else(|_| String::from("Error Body couldn't be read"));
                    println!("Failed to edit message to telegram: {}", err_body);
                }
            },
            Err(e) => {
                println!("Error editing message on Telegram: {}", e);
                
            }
        }
    } else {
        let response = telegram_client
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
                    return Ok(());
                } else {
                    println!("Error sending message to Telegram: {:?}", res);
                    return Err("Failed to sending message to Telegram".into());
                }
            },
            Err(e) => {
                println!("Error sending message to Telegram: {}", e);
                return Err(Box::new(e));
            }
        }
    }
    
    Ok(())
}

pub async fn telegram_get_file_data(
    file_id: String, 
    telegram_bot_token: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
    
    let telegram_client = Arc::new(Client::new());
    let telegram_api_url = format!(
        "https://api.telegram.org/bot{}/getFile?file_id={}", 
        telegram_bot_token,
        file_id,
    );
    
    let file_info: TelegramGetFile = telegram_client
        .get(telegram_api_url)
        .send()
        .await?
        .json()
        .await?;

    if file_info.ok {
        let file_path = file_info.result.unwrap().file_path.unwrap();
        let file_url = format!(
            "https://api.telegram.org/file/bot{}/{}", 
            telegram_bot_token, 
            file_path,
        );
        let image_response = telegram_client.get(&file_url).send().await?;
        let image_bytes = image_response.bytes().await?;
        // std::fs::write("image.jpg", &bytes)?;
        // println!("Image saved as image.jpg");
        Ok(STANDARD.encode(image_bytes))
    } else {
        Err(format!("Error getting file info: {:?}", file_info.description).into())
    }
}

pub fn replace_raw_escapes(input: &str) -> String {  
    let result = input
        .replace("\n\n\n\n* ", "\n\n")    
        .replace("\n\n\n* ", "\n\n")
        .replace("\n\n* ", "\n\n")
        .replace("\n* ", "\n")
        .replace("* ", "")
        .replace("**", "*");
    
    result
}