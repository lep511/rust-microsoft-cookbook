use std::fs;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::env;

fn read_and_encode(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Read the file contents
    let contents = fs::read(file_path)?;
    
    // Convert to base64
    let base64_string = BASE64.encode(contents);
    
    Ok(base64_string)
}

async fn cached_contents(base64_data: &str) -> String {
    // Get API key from environment variable
    let api_key = env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY environment variable is not set");

    // Construct the URL with the API key
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-002:generateContent?key={}",
        api_key
    );

}

pub async fn get_gemini_response(file_cache_path: &str) {
    let file_content_b64 = match read_and_encode(file_cache_path) {
        Ok(encoded) => encoded,
        Err(e) => {
            println!("[ERROR] Error convert to base64: {}", e);
            String::from("No data.")
        }
    };
}