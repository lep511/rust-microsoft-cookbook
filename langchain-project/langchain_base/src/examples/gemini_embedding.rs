#[allow(dead_code)]
// use crate::gemini::ChatGemini;
use crate::gemini::EmbedGemini;
use crate::gemini::TaskType;

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {

    // The Embedding model is optimized for creating embeddings 
    // with 768 dimensions for text of up to 2,048 tokens.
    let llm = EmbedGemini::new("text-embedding-004")?;
    
    let llm = llm.with_output_dimensionality(256);
    let llm = llm.with_task_type(TaskType::RetrievalDocument);
    let llm = llm.with_title("About the life");
    let input_str = "What is the meaning of life?";
    
    let response = llm.embed_content(input_str).await?;

    println!("Response: {:?}", response);

    Ok(())
}