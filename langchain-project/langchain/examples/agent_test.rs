use langchain::agents::financials_agents::{
    create_financials_agent, run_financials_agent,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Create the financials agent
    let financials_agent = create_financials_agent().await;
    // Print the agent's name and prompt
    println!("Agent Name: {}", financials_agent.name);
    println!("Agent Prompt: {}", financials_agent.instructions);

    run_financials_agent(&financials_agent).await
        .map(|result| println!("Agent Result: {}", result))
        .unwrap_or_else(|err| println!("Error running agent: {}", err));

    Ok(())
}