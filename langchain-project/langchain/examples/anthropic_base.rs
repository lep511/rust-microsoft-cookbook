use langchain::anthropic::chat::ChatAnthropic;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Example simple shot
    // claude-3-5-haiku-20241022	
    // claude-3-5-sonnet-20241022
    
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatAnthropic::new("claude-3-5-haiku-20241022")?;
    let response = llm
        .with_max_tokens(1024)
        .with_temperature(0.9)
        .with_max_tokens(2048)
        .with_timeout_sec(30)
        .with_max_retries(0)
        .invoke("Explain the Pythagorean theorem to a 10-year-old.")
        .await?;

    if let Some(candidates) = response.content {
        for candidate in candidates {
            match candidate.text {
                Some(text) => println!("{}", text),
                None => println!(""),
            }
        }
    };

    Ok(())
}