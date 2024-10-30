use reqwest::Client;
use serde_json::json;
use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct MessageContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct ApiResponse {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    role: String,
    model: String,
    content: Vec<MessageContent>,
    stop_reason: String,
    stop_sequence: Option<String>,
    usage: Usage,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the API key from environment variables
    let api_key = env::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY environment variable not set.");

    // Create a new HTTP client
    let client = Client::new();

    // Define the prompt for the API
    let system = "Your task is to analyze the provided text and identify any airport codes \
                  mentioned within it. Read through the itinerary carefully and identify all \
                  the cities mentioned. For each city, determine the primary airport code associated \
                  with it. Use your knowledge base to look up this information. Present these airport \
                  codes as a list in the order they appear in the text. If no airport codes are found, \
                  return an empty list. \
                  Before providing the final list, wrap your thought process in <thinking> tags. In this section \
                  ist each city mentioned in the itinerary with a number. For each city, write down potential \
                  airport codes and choose the most likely one. Explain the reasoning behind each choice.";

    let prompt = "My next trip involves flying from Seattle to Amsterdam. I'll be spending a few \
                  days in Amsterdam before heading to Paris for a connecting flight to Rome.";

    // Define the JSON payload
    let payload = json!({
        "model": "claude-3-5-sonnet-20241022",
        "max_tokens": 1024,
        "temperature": 0,
        "system": system,
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });

    // Send the POST request
    let response = client.post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&payload)
        .send()
        .await?;

    // Print the response
    let response_text = response.text().await?;
    //println!("{}", response_text);

    let api_response: ApiResponse = serde_json::from_str(&response_text)
        .expect("Failed to parse API response");

    println!("{:#?}", api_response);

    // Access specific fields
    if let Some(first_content) = api_response.content.first() {
        println!("Assistant says: {}", first_content.text);
    }

    Ok(())
}