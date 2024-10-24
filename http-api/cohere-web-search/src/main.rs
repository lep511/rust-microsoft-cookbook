use reqwest::Client;
use serde_json:: { Value, json };
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://api.cohere.com/v1/chat";
    let token = env::var("COHERE_API_KEY")
        .expect("COHERE_API_KEY environment variable not set.");

    let client = Client::new();
    let response = client
        .post(url)
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .header("Authorization", format!("bearer {}", token))
        .json(&json!({
            "message": "Is Matterport doing well?",
            "connectors": [
                {
                  "id": "web-search",
                  "options": {
                    "site": "https://investors.matterport.com/node/9501/html"
                  }
                }
            ],
            "model": "command-r-plus-08-2024",
            "temperature": 0.9,
            "max_tokens": 2048,
            "preamble": "You are an expert research assistant. Here is a document you will answer questions about:\n\nFirst, find the site from the document that are most relevant to answering the question, and then print them in numbered order. Quotes should be relatively short.\n\nIf there are no relevant quotes, write \"No relevant quotes\" instead.\n\nThen, answer the question, starting with \"Answer:\". Do not include or reference quoted content verbatim in the answer. Don’t say \"According to Quote [1]\" when answering. Instead make references to quotes relevant to each section of the answer solely by adding their bracketed numbers at the end of relevant sentences.\n\nThus, the format of your overall response should look like what’s shown between the tags. Make sure to follow the formatting and spacing exactly.\nQuotes:\n[1] \"Company X reported revenue of $12 million in 2021.\"\n[2] \"Almost 90% of revenue came from widget sales, with gadget sales making up the remaining 10%.\"\n\nAnswer:\nCompany X earned $12 million. [1] Almost 90% of it was from widget sales. [2]\n\n\nIf the question cannot be answered by the document, say so.",
            "search_queries_only": false,
        }))
        .send()
        .await?;

    let status = response.status();
    println!("Status: {}", status);

    let body = response.text().await?;
    let json: Value = serde_json::from_str(&body)?;

    if let Some(text) = json["text"].as_str() {
        println!("Response: {}", text);
    } else {
        println!("Text field not found");
    }

    Ok(())
}