use lambda_runtime::{tracing, Error, LambdaEvent};
use gemini_lib::{ LlmResponse, OrderState, generate_content };
use mongodb_lib::{ MongoResponse, mongodb_connect, mongodb_update };
use telegram_bot::{ send_message, hold_on_message };
use serde::Deserialize;
use serde_json::Value;
use std::env;

mod gemini_lib;
mod mongodb_lib;
mod telegram_bot;
mod bot;

#[derive(Debug, Deserialize)]
struct MessageBody {
    update_id: i64,
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
    tracing::info!("Payload: {:?}", payload_body);

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

    let token = match env::var("TELEGRAM_BOT_TOKEN")  {
        Ok(token) => token,
        Err(e) => {
            println!("[ERROR] Error getting environment variable TELEGRAM_BOT_TOKEN: {}", e);
            return Ok(());
        }
    };

    let prompt = body_data.message.text;
    let chat_id = body_data.message.from.id;
    let user_id = chat_id.to_string();

    match hold_on_message(&token.as_str(), chat_id).await {
        Ok(_) => println!("Message hold on sent successfully"),
        Err(e) => println!("Error sending message: {}", e),
    };

    let mongo_result: MongoResponse = match mongodb_connect(&user_id).await {
        Ok(mongo_result) => mongo_result,
        Err(e) => {
            println!("[ERROR] Initial connection to MongoDB fails: {}", e);
            return Ok(());
        }
    };

    // let input_text = mongo_result.user_data;
    let nc_count = mongo_result.chat_count;
    let input_text = format!(
        "{}Input {}\nCustomer: {}",
        mongo_result.user_data,
        nc_count,
        prompt
    );

    let llm_result: LlmResponse = match generate_content(&input_text).await {
        Ok(llm_result) => llm_result,
        Err(e) => {
            println!("[ERROR] Error generating content: {}", e);
            return Ok(());
        }
    };

    let text_parts = llm_result.gemini_response.candidates[0].content.parts[0].text.clone();
    let update_chat = format!("{}\nResponse {}\n\n{}\n", input_text, nc_count, text_parts); 
    // println!("{}", update_chat);

    let resp: OrderState = match serde_json::from_str(&text_parts) {
        Ok(resp) => resp,
        Err(e) => {
            println!("[ERROR] Error parsing JSON: {}", e);
            return Ok(());
        }
    };

    let mongo_result: MongoResponse = match mongodb_update(&user_id, &update_chat, nc_count).await {
        Ok(mongo_result) => mongo_result,
        Err(e) => {
            println!("[ERROR] Error when updating data in MongoDB: {}", e);
            return Ok(());
        }
    };

    //println!("Ok {:?}", mongo_result);
    let message = resp.response.ok_or("Response is missing")?;
    let text_msg = message.as_str();

    match send_message(token.as_str(), chat_id, text_msg).await {
        Ok(_) => println!("Message sent successfully"),
        Err(e) => println!("Error sending message: {}", e),
    }
    
    Ok(())
}