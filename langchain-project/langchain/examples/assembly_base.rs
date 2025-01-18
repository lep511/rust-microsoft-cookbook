use langchain::assembly::TranscriptAssemblyAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // UPLOAD FILE
    let audio_file = "tests/files/audio.mp3";
    let llm = TranscriptAssemblyAI::new()?;

    let audio_url = llm.upload_file(audio_file).await?;
    println!("Audio URL: {:?}", audio_url);

    // TRANSCRIPT AUDIO QUEUE
    let llm = TranscriptAssemblyAI::new()?;
    let response = llm
        .with_language_detection(true)
        .transcript(&audio_url)
        .await?;

    let transcript_id = match response.id {
        Some(id) => id,
        None => panic!("No transcript id found"),
    };

    println!("Transcript ID: {:?}", transcript_id);

    let mut full_text = String::new();

    // CHECK STATUS AUDIO
    loop {
        let llm = TranscriptAssemblyAI::new()?;
        let response = llm
            .get_transcript(&transcript_id)
            .await?;
        
        println!("Status: {:?}", response.status);

        if response.status.unwrap() == "completed" {
            full_text = response.text.unwrap();
            break;
        } else {
            println!("Waiting for transcript to be completed...");
            tokio::time::sleep(tokio::time::Duration::from_secs(8)).await;
        }
    }
    
    println!("Full text: {:?}", full_text);

    Ok(())
}