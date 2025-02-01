#[allow(dead_code)]
use langchain::anthropic::chat::ChatAnthropic;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example simple shot
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatAnthropic::new("claude-3-5-sonnet-20241022")?;

    let prompt = "What is the geometric monthly fecal coliform mean of a \
                  distribution system with the following FC counts: \
                  24, 15, 7, 16, 31 and 23? The result will be inputted \
                  into a NPDES DMR, therefore, \
                  round to the nearest whole number";

    // NOTE: the correct answer is 18

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