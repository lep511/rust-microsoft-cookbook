#[allow(dead_code)]
use aws_sdk_s3::primitives::ByteStream;
use reqwest::{Client, Response};
use super::chat::ChatGemini;
use super::libs::{Part, Content};
use super::errors::{S3Error};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, File};
use std::io::{self, Write, Read};
use futures::StreamExt;
use futures::pin_mut;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::Path;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct TelegramGetFile {
    ok: bool,
    result: Option<FileInfo>,
    error_code: Option<i32>,
    description: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct FileInfo {
    file_id: String,
    file_unique_id: String,
    file_size: i64,
    file_path: Option<String>,
}

pub async fn get_gemini_response(
    mut prompt: String, 
    telegram_bot_token: String, 
    telegram_chat_id: String,
    is_image: bool,
    bucket_name: String,
) -> Result<String, Box<dyn std::error::Error>> {

    let file_name = format!("chat-history-{}.json", telegram_chat_id);
    let file_path = format!("/tmp/{}", file_name);

    if prompt == "" {
        return Ok("The prompt is empty.".to_string());
    } else if prompt == "/new" || prompt == "/start" {
        match delete_chat_history (
            bucket_name.clone(),
            file_name.clone(),
        ).await {
            Ok(_) => {
                println!("Chat history deleted.");
                prompt = "Hi tehere!".to_string();
            },
            Err(e) => {
                println!("Error deleting chat history: {}", e);
            }
        }
    }

    let client = Client::new();
    let mut llm = ChatGemini::new("learnlm-1.5-pro-experimental")?;

    let system_prompt = "You are a tutor helping a student prepare for a test. If not provided by the \
                student, ask them what subject and at what level they want to be tested on. \
                Then, \
                \
                *   Generate multiple choice practice questions (A, B, C, D). Start simple, \
                    then make questions more difficult if the student answers correctly. \
                *   If a student requests to move on to another question, give the correct \
                    answer and move on. \
                *   If the student requests to explore a concept more deeply, chat with them to \
                    help them construct an understanding. \
                *   After 10 questions ask the student if they would like to continue with more \
                    questions or if they would like a summary of their session. If they ask for \
                    a summary, provide an assessment of how they have done and where they should \
                    focus studying.";
    
    let content = Content {
        role: "user".to_string(),
        parts: vec![Part {
            text: Some(prompt.clone()),
            function_call: None,
            function_response: None,
            inline_data: None,
            file_data: None,
        }],
    };
    let mut chat_history = vec![content.clone()];

    match get_chat_history(
        bucket_name.clone(),
        file_name.clone(),
        file_path.clone(),
    ).await {
        Ok(bytes) => {
            println!("File temp saved. Wrote {} bytes.", bytes);
            chat_history = read_chat_history(file_path).await?;
            chat_history.push(content.clone());
            llm = llm.with_chat_history(chat_history.clone());
        },
        Err(e) => {
            println!("Proceed without file chat history. {}", e);
        }
    }

    if is_image {
        println!("Is image: {}", is_image);
        let file_id = prompt.clone();
        prompt = "Explain this image.".to_string();
        
        let image_base64 = match telegram_get_file_data(
            file_id,
            telegram_bot_token.clone(),
        ).await {
            Ok(image) => image,
            Err(e) => return Err(e.into()),
        };
        
        let mime_type = "image/jpeg";
        
        llm = llm.with_image(
            &image_base64,
            mime_type,
        );
    }

    let stream = llm
        .with_system_prompt(system_prompt)
        .with_max_tokens(8192)
        .with_top_k(64)
        .stream_response(prompt);

    pin_mut!(stream);

    let mut complete_text = String::from("");
    let current_telegram_message_id = Arc::new(Mutex::new(None));

    while let Some(stream_response) = stream.next().await { 
        if let Some(candidates) = stream_response.candidates {
            for candidate in candidates {
                if let Some(content) = candidate.content {
                    for part in content.parts {
                        if let Some(text) = part.text {
                            complete_text = complete_text + &text;
                            match send_telegram_message(
                                telegram_bot_token.clone(),
                                telegram_chat_id.clone(),
                                complete_text.clone(),
                                current_telegram_message_id.clone()
                            ).await {
                                Ok(_) => (),
                                Err(e) => {
                                    println!("Error sending message: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        };
    }

    let content_model = Content {
        role: "model".to_string(),
        parts: vec![Part {
            text: Some(complete_text.clone()),
            function_call: None,
            function_response: None,
            inline_data: None,
            file_data: None,
        }],
    };

    chat_history.push(content_model);
    let file_path_save = format!("/tmp/created-{}", file_name);
    match save_chat_history(
        chat_history.clone(), 
        file_path_save.clone(),
    ).await {
        Ok(_) => {
            println!("Chat history saved to temp successfully");
            match put_chat_history(
                bucket_name.clone(),
                file_name.clone(),
                file_path_save.clone(),
            ).await {
                Ok(_) => {
                    println!("Chat history uploaded to bucket successfully");
                },
                Err(e) => {
                    println!("Error saving chat history to bucket: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Error saving chat history to temp: {}", e);
        }
    }
    
    match send_telegram_message(
        telegram_bot_token.clone(),
        telegram_chat_id.clone(),
        complete_text.clone(),
        current_telegram_message_id.clone(),
    ).await {
        Ok(_) => (),
        Err(e) => {
            println!("Error sending text/plain message: {}", e);
            return Err("Error sending text/plain message".into());
        }
    }

    let message_response = String::from("Message sent successfully");

    Ok(message_response)   
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

fn replace_raw_escapes(input: &str) -> String {  
    let result = input
        .replace("\n\n\n\n* ", "\n\n")    
        .replace("\n\n\n* ", "\n\n")
        .replace("\n\n* ", "\n\n")
        .replace("\n* ", "\n")
        .replace("* ", "")
        .replace("**", "*");
    
    result
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

pub async fn get_chat_history(
    bucket_name: String,
    key: String,
    file_path: String,
) -> Result<usize, S3Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    let mut file = File::create(file_path).map_err(|err| {
        S3Error::new(format!(
            "Failed to initialize file for saving S3 download: {err:?}"
        ))
    })?;

    let mut object = client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await?;
        
        let mut byte_count = 0_usize;
        while let Some(bytes) = object.body.try_next().await.map_err(|err| {
            S3Error::new(format!("Failed to read from S3 download stream: {err:?}"))
        })? {
            let bytes_len = bytes.len();
            file.write_all(&bytes).map_err(|err| {
                S3Error::new(format!(
                    "Failed to write from S3 download stream to local file: {err:?}"
                ))
            })?;
            byte_count += bytes_len;
        }
    
        Ok(byte_count)
}

pub async fn put_chat_history(
    bucket_name: String,
    key: String,
    file_path: String,
) -> Result<(), S3Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    // Create a ByteStream from the file in the temp directory
    let body = ByteStream::from_path(Path::new(&file_path))
        .await
        .map_err(|err| S3Error::new(format!("Failed to start file read stream: {err:?}")))?;

    let request = client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body)
        .send()
        .await?;

    Ok(())
}

pub async fn delete_chat_history(
    bucket_name: String,
    key: String,
) -> Result<(), S3Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    client
        .delete_object()
        .bucket(&bucket_name)
        .key(&key)
        .send()
        .await?;

    println!("Object deleted: {}/{}", &bucket_name, &key);
    Ok(())
}

pub async fn read_chat_history(
    file_path: String,
) -> Result<Vec<Content>, Box<dyn std::error::Error>> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return Err(Box::new(e));
        }
    };

    let mut json = String::new();
    match file.read_to_string(&mut json) {
        Ok(_) => {
            let chat_history: Vec<Content> = serde_json::from_str(&json)?;
            Ok(chat_history)
        }
        Err(e) => {
            println!("Error reading from file: {}", e);
            Err(Box::new(e))
        }
    }
}

pub async fn save_chat_history(chat_history: Vec<Content>, file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(&chat_history)?;
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating file: {}", e);
            return Err(Box::new(e));
        }
    };

    match file.write_all(json.as_bytes()) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            println!("Error writing to file: {}", e);
            Err(Box::new(e))
        }
    }
}

// pub async fn telegram_get_file_data(
//     file_id: String, 
//     telegram_bot_token: String,
//     ) -> Result<String, Box<dyn std::error::Error>> {
    
//     let telegram_client = Arc::new(Client::new());
//     let telegram_api_url = format!(
//         "https://api.telegram.org/bot{}/getFile?file_id={}", 
//         telegram_bot_token,
//         file_id,
//     );
    
//     let file_info: TelegramGetFile = telegram_client
//         .get(telegram_api_url)
//         .send()
//         .await?
//         .json()
//         .await?;

//     if file_info.ok {
//         let file_path = file_info.result.unwrap().file_path.unwrap();
//         let file_url = format!(
//             "https://api.telegram.org/file/bot{}/{}", 
//             telegram_bot_token, 
//             file_path,
//         );
//         let image_response = telegram_client.get(&file_url).send().await?;
//         let image_bytes = image_response.bytes().await?;
//         // std::fs::write("image.jpg", &bytes)?;
//         // println!("Image saved as image.jpg");
//         println!("File url get successfully");
//         Ok(STANDARD.encode(image_bytes))
//     } else {
//         Err(format!("Error getting file info: {:?}", file_info.description).into())
//     }
// }