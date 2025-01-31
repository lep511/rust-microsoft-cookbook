#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let response = llm
        .with_temperature(0.2)
        .with_max_tokens(1024)
        .with_top_k(20)
        .with_top_p(0.95)
        .with_candidate_count(2)
        .with_max_retries(3)
        .with_stop_sequences(vec!["STOP!".to_string()])
        .with_system_prompt("You are a helpful assistant.")
        .invoke("Tell me how the internet works, but pretend I'm a puppy who only understands squeaky toys.")
        .await?;

    // println!("{:?}", response);
   
    let mut n = 1;
    response.candidates.as_ref().map(|candidates| {
        candidates.iter().for_each(|candidate| {
            println!("\n\nCandidate: {}\n=============\n", n);
            n += 1;
            candidate.content.as_ref().map(|content| {
                content.parts.iter().for_each(|part| {
                    part.text.as_ref().map(|text| {
                        println!("{}", text);
                    });
                });
            });
        });
    });

    Ok(())
}