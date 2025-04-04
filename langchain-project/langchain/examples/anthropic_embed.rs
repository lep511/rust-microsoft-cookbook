#[allow(dead_code)]
use langchain::anthropic::embed::EmbedVoyage;
use langchain::anthropic::libs::InputEmbed;
use env_logger::Env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = EmbedVoyage::new("voyage-3");
    
    // let llm = llm.with_dimensions(256);
    // let input_str = InputEmbed::String("What is the meaning of life?".to_string());
    // let response = llm.embed_content(input_str).await?;

    let array_string = vec![
        "What is the meaning of life?".to_string(),
        "How do I make a cake?".to_string(),
        "What is the best way to learn a new language?".to_string(),
    ];

    let input_array = InputEmbed::Array(array_string);
    let response = llm.embed_content(input_array).await?;

    println!("Response: {:?}", response);
    Ok(())
}