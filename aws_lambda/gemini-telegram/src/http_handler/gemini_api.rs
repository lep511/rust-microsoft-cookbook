#[allow(dead_code)]
use aws_sdk_s3::primitives::ByteStream;
use super::telegram_bot::{
    TelegramMessage,
    send_telegram_message, 
    telegram_get_file_data,
    telegram_get_file_url
};
use super::utils::check_mimetype;
use super::chat::ChatGemini;
use super::GEMINI_MODEL;
use super::libs::{Part, Content};
use super::errors::{S3Error};
use std::fs::{self, File};
use std::io::{self, Write, Read};
use futures::StreamExt;
use futures::pin_mut;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::Path;

pub async fn get_gemini_response(
    message: &TelegramMessage,
    telegram_bot_token: &str, 
    telegram_chat_id: u32,
    bucket_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {

    let mut llm = ChatGemini::new(GEMINI_MODEL)?;

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

    let current_telegram_message_id = Arc::new(Mutex::new(None));
    let file_name = format!("chat-history-{}.json", telegram_chat_id);
    let file_path = format!("/tmp/{}", file_name);
    let mut prompt = String::new();

    // First check of type 1/2
    match message {
        TelegramMessage::Text { ref content } => {
            prompt = content.to_string();
            if prompt == "/new" || prompt == "/start" {
                match delete_chat_history (
                    bucket_name,
                    &file_name,
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
        },
        TelegramMessage::Image { file_id: _, caption } => {    
            prompt = caption.clone();
        },
        TelegramMessage::Document { 
            file_name: _,
            file_size,
            file_id: _, 
            caption, 
            mime_type, 
        } => {
            // Check if file size is greater than 12MB
            if *file_size > 12 * 1024 * 1024 {
                let message = "The file size cannot exceed 12MB.";
                send_telegram_message(
                    telegram_bot_token,
                    telegram_chat_id,
                    message,
                    current_telegram_message_id.clone(),
                ).await?;
                println!("Error: {}", message);
                return Err(message.into());
            // Check mime type
            } else if check_mimetype(mime_type) {
                let message = "File type is not supported.";
                send_telegram_message(
                    telegram_bot_token,
                    telegram_chat_id,
                    message,
                    current_telegram_message_id.clone(),
                ).await?;
                println!("Error: {}", message);
                return Err(message.into());
            } else {
                prompt = caption.clone();
            }
        },
    }
    
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
        bucket_name,
        &file_name,
        &file_path,
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

    // Second check of type 2/2
    match message {
        TelegramMessage::Text { ref content } => (),
        TelegramMessage::Image { ref file_id, caption: _ } => {
            let image_base64 = match telegram_get_file_data(
                file_id,
                telegram_bot_token,
            ).await {
                Ok(image) => image,
                Err(e) => {
                    println!("Error getting image: {}", e);
                    return Err(e.into());
                }
            };
            
            let mime_type = "image/jpeg";
            
            llm = llm.with_inline_data(
                &image_base64,
                mime_type,
            );
            let last_content = llm.clone().get_last_content();
            chat_history.push(
                last_content.expect("No last content found")
            );
            llm = llm.with_chat_history(chat_history.clone());
        },
        TelegramMessage::Document { 
            ref file_name, 
            file_size,
            ref file_id, caption: _, 
            ref mime_type 
        } => {
            let upload_data = match telegram_get_file_data(
                file_id,
                telegram_bot_token,
            ).await {
                Ok(docdata) => docdata,
                Err(e) => {
                    println!("Error getting document: {}", e);
                    return Err(e.into());
                }
            };

            let file_path = None;

            llm = llm.media_upload(
                file_path,
                Some(upload_data),
                file_name,
                mime_type,
            ).await?;

            let last_content = llm.clone().get_last_content();
            chat_history.push(
                last_content.expect("No last content found")
            );
            llm = llm.with_chat_history(chat_history.clone());
        },
    }

    let stream = llm
        .with_system_prompt(system_prompt)
        .with_max_tokens(8192)
        .with_top_k(64)
        .stream_response(prompt);

    pin_mut!(stream);

    let mut complete_text = String::from("");

    while let Some(stream_response) = stream.next().await { 
        if let Some(candidates) = stream_response.candidates {
            for candidate in candidates {
                if let Some(content) = candidate.content {
                    for part in content.parts {
                        if let Some(text) = part.text {
                            complete_text = complete_text + &text;
                            match send_telegram_message(
                                telegram_bot_token,
                                telegram_chat_id,
                                &complete_text,
                                current_telegram_message_id.clone(),
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
        &file_path_save,
    ).await {
        Ok(_) => {
            match put_chat_history(
                bucket_name,
                &file_name,
                &file_path_save,
            ).await {
                Ok(_) => (),
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
        telegram_bot_token,
        telegram_chat_id,
        &complete_text,
        current_telegram_message_id,
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

pub async fn get_chat_history(
    bucket_name: &str,
    key: &str,
    file_path: &str,
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
    bucket_name: &str,
    key: &str,
    file_path: &str,
) -> Result<(), S3Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    // Create a ByteStream from the file in the temp directory
    let body = ByteStream::from_path(Path::new(file_path))
        .await
        .map_err(|err| S3Error::new(format!("Failed to start file read stream: {err:?}")))?;

    let _request = client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body)
        .send()
        .await?;

    Ok(())
}

pub async fn delete_chat_history(
    bucket_name: &str,
    key: &str,
) -> Result<(), S3Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    client
        .delete_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await?;

    println!("Object deleted: {}/{}", bucket_name, key);
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

pub async fn save_chat_history(
    chat_history: Vec<Content>, 
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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