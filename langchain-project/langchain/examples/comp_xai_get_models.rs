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
    
    println!("{:?}", response);
    
    Ok(())
}