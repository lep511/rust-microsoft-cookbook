use langchain_base::anthropic::ChatAnthropic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Example simple shot
    // claude-3-5-haiku-20241022	
    // claude-3-5-sonnet-20241022

    let llm = ChatAnthropic::new("claude-3-5-haiku-20241022")?;
    let llm = llm.with_max_tokens(1024);
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);
    // let llm = llm.with_stream(true);
    let llm = llm.with_timeout_sec(30);
    let prompt = "Explain the Pythagorean theorem to a 10-year-old.";
    let response = llm.invoke(prompt).await?;

    println!("#### Example Anthropic Simple shot ####");
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