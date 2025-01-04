#[allow(dead_code)]
use langchain_base::anthropic::EmbedVoyage;
use langchain_base::anthropic::MultiTypeInput;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // The Embedding model is optimized for creating embeddings 
    // with 768 dimensions for text of up to 2,048 tokens.
    let llm = EmbedVoyage::new("voyage-3")?;
    
    // let llm = llm.with_output_dimensionality(256);
    // let input_str = MultiTypeInput::String("What is the meaning of life?".to_string());
    // let response = llm.embed_content(input_str).await?;

    let array_string = vec![
        "What is the meaning of life?".to_string(),
        "How do I make a cake?".to_string(),
        "What is the best way to learn a new language?".to_string(),
    ];

    let input_array = MultiTypeInput::Array(array_string);
    let response = llm.embed_content(input_array).await?;

    println!("Response: {:?}", response);

    Ok(())
}