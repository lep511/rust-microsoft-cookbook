use langchain::gemini::chat::ChatGemini;
use env_logger::Env;
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetStockPrice {
    company: String,
}

async fn example_tools() -> Result<(), Box<dyn std::error::Error>> {

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Get Company Name ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let prompt = "How many shares of Nvidia can I buy with $500?";

    let get_stock_price = json!({
        "name":"get_stock_price",
        "description":"Retrieves the current stock price for a given company",
        "parameters":{
            "type":"OBJECT",
            "properties":{
                "company":{
                    "type":"STRING",
                    "description":"The company name to fetch stock data for",

                }
            }
        }
    });

    let function_dec = vec![json!({
        "functionDeclarations":[
            get_stock_price
        ]
    })];

    let tool_config = json!({
        "function_calling_config":{
            "mode":"ANY",
            "allowed_function_names":[
                "get_stock_price"
            ]
        }
    });

    let response = llm
        .with_tools(function_dec)
        .with_tool_config(tool_config)
        .invoke(prompt)
        .await?;
    
    let mut function_args = Value::Null;

    println!("Question: {}", prompt);
    if let Some(candidates) = &response.candidates {
        for candidate in candidates {
            if let Some(content) = &candidate.content {
                for part in &content.parts {
                    if let Some(function_call) = &part.function_call {
                        function_args = function_call.args.clone();
                    }
                }
            }
        }
    };

    let get_share: GetStockPrice = serde_json::from_value(function_args)?;
    println!("Company: {}", get_share.company);

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Get Company share ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;
    
    let question = format!(
        "What is the current share price of {}",
        get_share.company,
    );
    
    let response = llm
        .with_google_search()
        .invoke(&question)
        .await?;

    let mut response_string = String::from("");

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                        response_string.push_str(&text);
                    }
                }
            }
        }
    };

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Final response ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;
    
    let final_prompt = format!(
        "{}\n\nInformation about the shares:\n{}\n\nImportant:\n \
        * If there are many brokers on the share price, average the values.\n \
        * Extract the results in a single line. \n \
        * Do not include any other information in the response.",
        prompt,
        response_string,
    );

    let response = llm
        .invoke(&final_prompt)
        .await?;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    example_tools().await?;
    Ok(())
}