#[allow(dead_code)]
use langchain::anthropic::ChatAnthropic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example simple shot
    let llm = ChatAnthropic::new("claude-3-5-sonnet-20241022")?;
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);
    
    let system_prompt = "You can write and execute Rust code by enclosing it in triple backticks, e.g. ```code goes here```. Use this to perform calculations.";
    
    let llm = llm.with_system_prompt(system_prompt);

    let prompt = "Find all real-valued roots of the following polynomial: 3*x**5 - 5*x**4 - 3*x**3 - 7*x - 10.";
    let response = llm.invoke(prompt).await?;

    println!("#### Example Anthropic Code execution ####");
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