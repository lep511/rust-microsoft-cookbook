#[allow(dead_code)]
use langchain::anthropic::chat::ChatAnthropic;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example simple shot
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatAnthropic::new("claude-3-5-sonnet-20241022");
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);
    
    let system_prompt = "You can write and execute Rust code by enclosing it in triple backticks, e.g. ```code goes here```. Use this to perform calculations.";
    
    let llm = llm.with_system_prompt(system_prompt);

    let prompt = "Find all real-valued roots of the following polynomial: 3*x**5 - 5*x**4 - 3*x**3 - 7*x - 10.";
    let response = llm.invoke(prompt).await?;

    if let Some(candidates) = response.content {
        candidates.iter()
            .filter_map(|c| c.text.as_ref())
            .for_each(|text| println!("{text}"));
    } else {
        println!("No response choices available");
    }

    Ok(())
}