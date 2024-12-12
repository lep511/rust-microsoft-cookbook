use reqwest::{ Client, Body };
use serde_json::json;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

fn get_random_loading_phrase() -> Option<&'static str> {

    let mut rng = thread_rng();

    // 60% chance to generate a phrase
    if rng.gen_bool(0.6) {
        let loading_phrases = [
            "Processing your request...",
            "Hold on a moment, please...", 
            "Just a second, please...",
            "Give me a moment...",
            "Please wait a second...",
            "One moment, please...",
            "I'm working on it...",
            "Just a moment, please...",
            "Please hold on a moment...",
            "Hang tight, please...",
            "I'm on it, just a moment...",
            "Hold tight for a second...",
            "Bear with me for a moment...",
            "Please stand by...",
            "This will just take a moment...",
            "Just hang on a second...",
            "One sec, please...",
            "I'll get that for you right away...",
            "I'm handling your request...",
            "Thank you for your patience...",
            "I'll be right with you...",
            "Just a bit longer, please...",
            "Working on it, give me a sec...",
            "I'll take care of that shortly...",
            "Please give me a moment...",
        ];
        Some(loading_phrases.choose(&mut rng).unwrap())
    } else {
        None
    }
}

pub async fn hold_on_message(token: &str, chat_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    match get_random_loading_phrase() {
        Some(phrase) => {
            let _ = send_message(token, chat_id, phrase).await?;
        },
        None => println!("No phrase generated")
    };
    
    Ok(())
}

pub async fn send_message(token: &str, chat_id: i32, text_message: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // https://api.telegram.org/bot7796241975:AAEnE3G8IaUhx-HydXlp5Yc0Fr8OQ0nHE3k/getUpdates
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let client = Client::new();
    
    // Prepare the request body
    let request_body = json!({
        "chat_id": chat_id,
        "text": text_message,
        "parse_mode": "markdown",
    });
    
    let request_body = serde_json::to_string(&request_body)?;
    let body: Body = Body::wrap(request_body);
    
    // Send the POST request
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    // Print status code
    println!("Status telegram bot: {}", response.status());
    
    let json = response.json().await?;
    Ok(json)
}