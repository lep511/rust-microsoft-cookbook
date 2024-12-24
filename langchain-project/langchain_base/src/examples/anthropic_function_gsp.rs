#[allow(dead_code)]
use crate::anthropic::ChatAnthropic;
use serde_json::json;

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatAnthropic::new("claude-3-5-sonnet-20241022")?;
    let tool_data = json!({
        "name": "get_stock_price",
        "description": "Retrieves the current stock price for a given ticker symbol. \
                        The ticker symbol must be valid for a publicly traded company \
                        on a major US stock exchange like NYSE or NASDAQ. \
                        The tool will return the latest trade price in USD. \
                        It should be used when the user asks about the current \
                        or most recent price of a specific stock. \
                        It will not provide any other information \
                        about the stock or company.",
        "input_schema": {
          "type": "object",
          "properties": {
            "ticker": {
              "type": "string",
              "description": "The stock ticker symbol, e.g. AAPL for Apple Inc."
            }
          },
          "required": ["ticker"]
        }
    });

    let tools = vec![tool_data];
    let tool_choice = json!({"type": "auto"});
    let llm = llm.with_tools(tools, tool_choice);

    let prompt = "How much is Tesla stock trading for? Before answering, explain your reasoning step-by-step in tags.";
    let response = llm.invoke(prompt).await?;

    println!("\n#### Example Anthropic Function Call ####");
    println!("Response: {:?}", response);

    Ok(())
}