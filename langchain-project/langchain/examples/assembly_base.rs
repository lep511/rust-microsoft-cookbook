use langchain::assembly::TranscriptAssemblyAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let audio_url = "https://assemblyaiusercontent.com/playground/6kw4fPUmkxO.mp3";

    // let llm = TranscriptAssemblyAI::new("best")?;
    // let response = llm
    //     .with_language_detection(true)
    //     .transcript(audio_url)
    //     .await?;

    // println!("Response: {:?}", response);

    let transcript_id = "da561260-5f63-4a0e-9091-e1a3222042f6";

    let llm = TranscriptAssemblyAI::new("best")?;
    let response = llm
        .get_transcript(transcript_id)
        .await?;

    println!("Response: {:?}", response);


    Ok(())
}