use reqwest::Client;
use scraper::{Html, Selector};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let url = "https://www.anthropic.com/news/third-party-testing";
    // Make HTTP request
    let response = client
        .request(reqwest::Method::GET, url)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err("Request failed".into());
    }

    let string_response = response.text().await?;

    // Parse HTML
    let document = Html::parse_document(&string_response);
    
    // Create a selector for the elements you want to extract
    let selector = Selector::parse("p, h1, h2, h3, h4").unwrap();
    let mut extracted_text = String::new();
    
    // Extract and print text from selected elements
    for element in document.select(&selector) {
        extracted_text.push_str(&element.text().collect::<String>());
        extracted_text.push_str("\n");
    }

    fs::write("anthropic_web_scraping.txt", extracted_text)?;

    Ok(())
}