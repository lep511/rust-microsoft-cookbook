use langchain::assembly::engine::TranscriptAssemblyAI;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let transcript_id = "abc55793-e758-4efa-8364-d16a51405794";

    let llm = TranscriptAssemblyAI::new();
    let response = llm
        .get_transcript(&transcript_id)
        .await?;
    
    println!("Status: {:?}", response.status);

    if response.status.unwrap() == "completed" {
        let full_text = response.text.unwrap();
        println!("Full text: {:?}", full_text);

    } else {
        println!("Transcript is not completed...");
    }

    Ok(())
}