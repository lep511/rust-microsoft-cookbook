use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::anthropic::embed::EmbedVoyage;
use crate::anthropic::libs::{InputEmbed, EmbedResponse};
use serde::{ Deserialize, Serialize };
use tokio::time::sleep;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsData {
    pub link: String,
    pub headline: String,
    pub category: String,
    pub short_description: String,
    pub authors: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsDataEmbed {
    pub link: String,
    pub headline: String,
    pub category: String,
    pub short_description: String,
    pub authors: String,
    pub date: String,
    pub news_embedding: Vec<f32>,
}

pub async fn get_embedding(text: &str) -> Result<EmbedResponse, Box<dyn std::error::Error>> {
    
    let llm = EmbedVoyage::new("voyage-3-large");
    let input_str = InputEmbed::String(text.to_string());

    let response = llm.with_dimensions(2048)
        .embed_content(input_str).await?;
    
    // Wait 2 sec to avoid API overload
    sleep(tokio::time::Duration::from_secs(2)).await;
    Ok(response)
}

pub async fn handle_file<P: AsRef<Path>>(
    path: P
) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    Ok(contents)
}

pub format_news_embed(news: &NewsData) -> String {
    let response = format!(
        "Headline: {}\nCategory: {}\nDescription: {}\nAuthors: {}\nDate: {}",
        news.headline,
        news.category,
        news.short_description,
        news.authors,
        news.date
    );

    response
}

pub async fn read_json_file(file_path: &str) -> Option<String> {  
    match handle_file(file_path).await {
        Ok(data) => {
            println!("JSON file read successfully: {}", file_path);
            Some(data)
        }
        Err(e) => {
            println!("Error reading or parsing JSON file: {}", e);
            None
        }
    }
}