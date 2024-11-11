use reqwest::Client;
use serde_json::json;
use std::error::Error;
use std::env;
use std::fs;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Response {
    id: String,
    texts: Vec<String>,
    embeddings: Vec<Vec<f32>>,
}

// Function to calculate cosine similarity
fn cosine_similarity(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (magnitude_a * magnitude_b)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://api.cohere.com/v1/embed";
    let token = env::var("COHERE_API_KEY")
        .expect("COHERE_API_KEY environment variable not set.");

    let mut texts = Vec::new();

    // List of file names
    let file_names = vec!["text1.txt", "text2.txt", "text3.txt"];

    // Read contents of each file and add to the texts vector
    for file_name in file_names {
        let file_content = fs::read_to_string(file_name)?;
        texts.extend(file_content.lines().map(|line| line.to_string()));
    }

    let client = Client::new();
    let response = client
        .post(url)
        .header("content-type", "application/json")
        .header("Authorization", format!("BEARER {}", token))
        .json(&json!({
            "model": "embed-english-v3.0",
            "texts": ["When are you open?", "Do you have a yogur?", "When do you close?", "What are the hours?", "Are you open on weekends?", "Are you available on holidays?", "How much is a burger?", "What\'s the price of a meal?", "How much for a few burgers?", "Do you have a vegan option?", "Do you have vegetarian?", "Do you serve non-meat alternatives?", "Do you have milkshakes?", "Milkshake", "Do you have desert?", "Can I bring my child?", "Are you kid friendly?", "Do you have booster seats?", "Do you do delivery?", "Is there takeout?", "Do you deliver?", "Can I have it delivered?", "Can you bring it to me?", "Do you have space for a party?", "Can you accommodate large groups?", "Can I book a party here?"],
            "input_type": "classification",
            "truncate": "NONE"
        }))
        .send()
        .await?;

    let status = response.status();
    println!("Status: {}", status);

    let body = response.text().await?;
    let response: Response = serde_json::from_str(&body).expect("JSON was not well-formatted");

    println!("Response id: {}", response.id);
    //println!("First text: {:?}", response.texts[1]);
    //println!("First embeddings: {:?}", response.embeddings[1]);

    let first_embed = &response.embeddings[0];
    let mut similarities: Vec<(f32, usize)> = response.embeddings.iter()
        .enumerate()
        .map(|(i, embed)| (cosine_similarity(embed, first_embed), i))
        .collect();
    
    // Sort by similarity score in descending order
    similarities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // Get top 3 similarities
    let top_3 = &similarities[1..4];  // Skip the first (most similar to itself)

    println!("Question: {}", response.texts[0]);

    for (similarity, index) in top_3 {
        println!("Similarity score: {}, Text: {}", similarity, response.texts[*index]);
    }

    Ok(())
}