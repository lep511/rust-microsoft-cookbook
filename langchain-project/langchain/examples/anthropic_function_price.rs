#[allow(dead_code)]
use langchain::anthropic::chat::ChatAnthropic;
use env_logger::Env;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
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
    let tool_choice = Some(json!({"type": "auto"}));
    let llm = llm.with_tools(tools, tool_choice);

    let prompt = "How much is Tesla stock trading for? Before answering, explain your reasoning step-by-step in tags.";
    let response = llm.invoke(prompt).await?;

    println!("\n#### Example Anthropic Function Call ####");
    println!("Response: {:?}", response);

    if let Some(contents) = &response.content {
        let function_content = &contents[1];
        assert_eq!(function_content.name, Some("get_stock_price".to_string()));
        assert_eq!(function_content.content_type, Some("tool_use".to_string()));
        if let Some(input) = &function_content.input {
            assert_eq!(input["ticker"], "TSLA");
        } else {
            panic!("Input should not be None");
        }
    } else {
        panic!("Content should not be None");
    }

    Ok(())
}