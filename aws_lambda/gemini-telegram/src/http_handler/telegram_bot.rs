use reqwest::{Client, Response};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use futures::StreamExt;
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
    Document {
        file_name: String,
        file_size: i64,
        file_id: String,
        caption: String,
        mime_type: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentData>,
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
pub struct DocumentData {
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_id: Option<String>,
    pub file_unique_id: Option<String>,
    pub file_size: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelegramGetFile {
    pub ok: bool,
    pub result: Option<FileInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: u32,
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
    bot_token: &str, 
    chat_id: u32, 
    text_raw: &str,
    current_telegram_message_id: Arc<Mutex<Option<i64>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let telegram_client = Arc::new(Client::new());
    let telegram_api_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let text = replace_raw_escapes(&text_raw);
    
    let mut message_body = json!({
        "chat_id": chat_id,
        "text": text,
        "parse_mode": "markdownV2"
    });

    let current_message_id = *current_telegram_message_id.lock().await;
    
    if let Some(message_id) = current_message_id {
        let edit_message_url = format!("https://api.telegram.org/bot{}/editMessageText", bot_token);
        message_body = json!({
            "chat_id": chat_id,
            "text": text,
            "message_id": message_id,
            "parse_mode": "markdownV2"
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

pub async fn telegram_get_file_url(
    file_id: &str, 
    telegram_bot_token: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
    
    let telegram_client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let telegram_api_url = format!(
        "https://api.telegram.org/bot{}/getFile", 
        telegram_bot_token,
    );

    let message_body = json!({
        "file_id": file_id
    });
    
    let response = telegram_client
        .post(telegram_api_url)
        .header("Content-Type", "application/json")
        .json(&message_body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    // println!("response: {:?}", res);
    let file_info: TelegramGetFile = serde_json::from_value(response)
        .map_err(|e| format!("Error parsing JSON: {}", e))?;
    
    let file_path = file_info.result
        .ok_or("Missing result in response")?
        .file_path
        .ok_or("Missing file path")?;
        
    let file_url = format!(
        "https://api.telegram.org/file/bot{}/{}", 
        telegram_bot_token, 
        file_path
    );
    
    Ok(file_url)

}

pub async fn telegram_get_file_data(
    file_id: &str, 
    telegram_bot_token: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {

    let telegram_client = Client::builder()
        .use_rustls_tls()
        .build()?;
            
    let file_url = telegram_get_file_url(
        file_id, 
        telegram_bot_token
    ).await?;
    
    Ok(telegram_client
        .get(&file_url)
        .send()
        .await?
        .bytes()
        .await
        .map(|bytes| STANDARD.encode(bytes))?)

}

pub fn replace_raw_escapes(sentence: &str) -> String {  
    const ESCAPE_CHARS: [char; 18] = [
        '\\', '_', '*', '[', ']', '(', ')', '~', '>', '#', 
        '+', '-', '=', '|', '{', '}', '.', '!'
    ];
    
    // Preallocate string with estimated capacity
    let estimated_size = sentence.len() * 2;
    let mut new_sentence = String::with_capacity(estimated_size);
    
    // Escape special characters
    sentence.chars().for_each(|c| {
        if ESCAPE_CHARS.contains(&c) {
            new_sentence.push('\\');
            new_sentence.push(c);
        } else {
            new_sentence.push(c);
        }
    });
    
    // Convert to UTF-16
    let utf16_string: Vec<u16> = new_sentence.encode_utf16().collect();
    
    let decoded_string = String::from_utf16(&utf16_string).unwrap_or_default();
    let decoded_string = decoded_string.replace("\\*\\*", "*");
    decoded_string
}