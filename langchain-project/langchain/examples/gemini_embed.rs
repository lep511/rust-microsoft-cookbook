#[allow(dead_code)]
// use langchain::gemini::ChatGemini;
use langchain::gemini::embed::EmbedGemini;
use langchain::gemini::libs::TaskType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // The Embedding model is optimized for creating embeddings 
    // with 768 dimensions for text of up to 2,048 tokens.
    let llm = EmbedGemini::new("text-embedding-004")?;
    // let llm = EmbedGemini::new("gemini-2.0-flash-exp")?;
    let input_str = "What is the meaning of life?";
    
    let response = llm
        .with_output_dimensionality(256)
        .with_task_type(TaskType::RetrievalDocument)
        .with_title("About the life")
        .with_max_retries(3)
        .embed_content(input_str)
        .await?;
   
    println!("Response: {:?}", response);

    Ok(())
}