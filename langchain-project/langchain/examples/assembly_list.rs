use langchain::assembly::TranscriptAssemblyAI;
use langchain::assembly::ListTranscriptParameters;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // TRANSCRIPT AUDIO QUEUE
    let llm = TranscriptAssemblyAI::new()?;

    let param = Some(ListTranscriptParameters {
        status: Some("completed".to_string()),
        limit: Some(3),
        created_on: None,
        before_id: None,
        after_id: None,
        throttled_only: None
    });
    
    let response = llm
        .list_transcripts(param)
        .await?;

    for transcript in response.transcripts {
        println!("Transcript id: {:?}", transcript.id);
        println!("Transcript status: {:?}", transcript.status);
        println!("Transcript created: {:?}", transcript.created);
        println!("-------------------------------------------------");
    }
    
    Ok(())
}