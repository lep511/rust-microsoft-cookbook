#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://api.x.ai/v1";
    let model = "";
    let llm = ChatCompatible::new(base_url, model);

    let response = llm
        .get_models("language-models")
        .await?;
    
    if let Some(models) = response.get("models").and_then(|m| m.as_array()) {
        for model in models {
            println!("ID: {}", model["id"].as_str().unwrap_or_default());
            println!("Version: {}", model["version"].as_str().unwrap_or_default());
            println!("Owner: {}", model["owned_by"].as_str().unwrap_or_default());
            
            // Print aliases if any
            if let Some(aliases) = model["aliases"].as_array() {
                println!("Aliases: {}", aliases.iter()
                    .filter_map(|a| a.as_str())
                    .collect::<Vec<_>>()
                    .join(", "));
            }
            
            // Print prices
            println!("Text completion price: {}", model["completion_text_token_price"].as_i64().unwrap_or_default());
            println!("Text prompt price: {}", model["prompt_text_token_price"].as_i64().unwrap_or_default());
            println!("Image prompt price: {}", model["prompt_image_token_price"].as_i64().unwrap_or_default());
            
            println!("-------------------");
        }
    }
    
    Ok(())
}