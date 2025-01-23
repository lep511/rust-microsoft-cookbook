use langchain::langsmith::client::LangsmithClient;
use langchain::langsmith::libs::LangsmithResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = LangsmithClient::new()?;
    
    let response: LangsmithResponse = client
        .create_dataset("Rev New dataset")
        .with_description("A new dataset")
        .invoke()
        .await?;

    println!("Response: {:?}", response);  

    Ok(())
}