mod openai;
mod gemini;
mod anthropic;

use openai::ChatOpenAI;
use gemini::ChatGemini;
use anthropic::ChatAnthropic;

use serde_json::json;
use serde::{Deserialize, Serialize};

async fn sample_anthropic_function_gsp() -> Result<(), Box<dyn std::error::Error>> {
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

async fn sample_anthropic() -> Result<(), Box<dyn std::error::Error>> {
    // Example simple shot
    let llm = ChatAnthropic::new("claude-3-5-sonnet-20241022")?;
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
            println!("{:?}", candidate.text);
        }
    };

    Ok(())
}

async fn sample_gemini() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-1.5-flash")?;
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);

    let prompt = "Explain the Pythagorean theorem to a 10-year-old.";
    let response = llm.invoke(prompt).await?;

    println!("\n#### Example Gemini simple shot ####");
    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                    }
                }
            }
        }
    };

    Ok(())
}

async fn sample_openai() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatOpenAI::new("gpt-4o-mini")?;
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);

    let prompt = "Explain the Pythagorean theorem to a 10-year-old.";
    let response = llm.invoke(prompt).await?;

    println!("\n#### Example OpenAI simple shot ####");
    if let Some(candidates) = response.choices {
        for candidate in candidates {
            if let message = candidate.message {
                println!("{}", message.content);
            }
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // sample_anthropic().await?;
    // sample_gemini().await?;
    // sample_openai().await?;
    sample_anthropic_function_gw().await?;
    // sample_anthropic_function().await?;

    Ok(())
}
