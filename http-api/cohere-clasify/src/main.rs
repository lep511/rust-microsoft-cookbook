use reqwest::Client;
use serde_json::json;
use std::error::Error;
use std::env;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ResponseObject {
    classifications: Vec<Classification>,
    // id: String,
    // meta: Meta,
}

#[derive(Deserialize, Debug)]
struct Classification {
    classification_type: String,
    confidence: f64,
    confidences: Vec<f64>,
    id: String,
    input: String,
    labels: HashMap<String, Label>,
    prediction: String,
    predictions: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Label {
    confidence: f64,
}

// #[derive(Deserialize, Debug)]
// struct Meta {
//     api_version: ApiVersion,
//     billed_units: BilledUnits,
// }

// #[derive(Deserialize, Debug)]
// struct ApiVersion {
//     version: String,
// }

// #[derive(Deserialize, Debug)]
// struct BilledUnits {
//     classifications: u32,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://api.cohere.com/v1/classify";
    let token = env::var("COHERE_API_KEY")
        .expect("COHERE_API_KEY environment variable not set.");

    let client = Client::new();
    let response = client
        .post(url)
        .header("content-type", "application/json")
        .header("Authorization", format!("BEARER {}", token))
        .json(&json!({
            "model": "embed-english-v3.0",
            "inputs": ["Any decent trader spends 90% of their efforts exploring how they could be wrong. That should apply to everyone’s decision making.", "Congratulations to all of this years @OliverAwards winners!"],
            "examples": [{"label": "Business news", "text": "Elon Musk says Twitter Blue subscribers should be able to pay with dogecoin"}, {"label": "Business news", "text": "Probability of a US recession in the next 12 months, via WSJ"}, {"label": "Business news", "text": "European futures slide"}, {"label": "Business news", "text": "NASDAQ rises 2% to ATH"}, {"label": "Business news", "text": "FTX Founder is one of the world\'s richest crypto billionaires, with a fortune valued at $20 billion."}, {"label": "Cooking", "text": "Sweet Potato Macaroni Cheese is #RecipeOfTheDay, and I’m very happy about it!"}, {"label": "Cooking", "text": "3-Ingredient Slow Cooker recipes"}, {"label": "Cooking", "text": "This is by far the BEST biscuit recipe I’ve ever tried"}, {"label": "Cooking", "text": "Baking my first loaf of banana bread..."}, {"label": "Cooking", "text": "From the queen of Italian cooking, this is one of the most iconic tomato sauce recipes ever"}, {"label": "Arts & Culture", "text": "I’ve actually read this book and it was extremely insightful. A quick read as well and available as a free audiobook through many libraries."}, {"label": "Arts & Culture", "text": "Today’s Daily Cartoon"}, {"label": "Arts & Culture", "text": "Get a glimpse of the stage adaptation of Hayao Miyazaki’s 2001 Oscar-winning animated feature Spirited Away"}, {"label": "Arts & Culture", "text": "The #Banksy Exhibit in Cambridge, MA is absolutely terrific."}, {"label": "Arts & Culture", "text": "“A Whisper in Time” large abstract paining 48’ x 48’"}]
        }))
        .send()
        .await?;

    let status = response.status();
    println!("Status: {}", status);

    let body = response.text().await?;
    let response: ResponseObject = serde_json::from_str(&body).expect("Error deserializing JSON");

    // println!("ID: {:?}", response.id);
    // println!("Meta: {:?}", response.meta);

    for classification in &response.classifications {
        println!("Classification Type: {:?}", classification.classification_type);
        println!("Prediction: {:?}", classification.prediction);
        println!("Confidence: {:?}", classification.confidence);
        println!();
    }
 
    Ok(())
}