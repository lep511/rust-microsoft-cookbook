#[allow(dead_code)]
use langchain::openai::EmbedOpenAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = EmbedOpenAI::new("text-embedding-3-small")?;
    let llm = llm.with_dimensions(256);
    let input_str = "What is the meaning of life?";
    
    let response = llm.embed_content(input_str).await?;

    println!("Response: {:?}", response);
    // len of vector
    println!("Lenght: {}", response.data[0].embedding.len());

    Ok(())
}