use langchain::gemini::chat::ChatGemini;
use env_logger::Env;
use langchain::gemini::libs::{Part, FunctionCall, FunctionResponse, FunctionContent};
use serde_json::json;

async fn example_tools() -> Result<(), Box<dyn std::error::Error>> {

    let llm = ChatGemini::new("gemini-2.0-flash-exp");
    
    let function1 = json!({
        "name":"get_product_info",
        "description":"Get the stock amount and identifier for a given product",
        "parameters":{
            "type":"OBJECT",
            "properties":{
                "product_name":{
                    "type":"STRING",
                    "description":"Product name"
                }
            }
        }
    });

    let function2 = json!({
        "name":"get_store_location",
        "description":"Get the location of the closest store",
        "parameters":{
            "type":"OBJECT",
            "properties":{
                "location":{
                    "type":"STRING",
                    "description":"Location"
                }
            }
        }
    });

    let function3 = json!({
        "name":"place_order",
        "description":"Place an order",
        "parameters":{
            "type":"OBJECT",
            "properties":{
                "product":{
                    "type":"STRING",
                    "description":"Product name"
                },
                "address":{
                    "type":"STRING",
                    "description":"Shipping address"
                }
            }
        }
    });

    let function_dec = vec![json!({
        "functionDeclarations":[
            function1,
            function2,
            function3
        ]
    })];

    let tool_config = json!({
        "function_calling_config":{
            "mode":"ANY",
            "allowed_function_names":[
                "get_product_info",
                "get_store_location"
            ]
        }
    });

    let question = "Do you have the Pixel 8 Pro in stock";
    
    let response = llm.clone()
        .with_temperature(0.0)
        .with_tools(function_dec.clone())
        .with_tool_config(tool_config)
        .invoke(question)
        .await?;
    
    let mut function_name = String::new();
    let mut product_name = String::new();

    println!("Question: {}", question);
    if let Some(candidates) = &response.candidates {
        for candidate in candidates {
            if let Some(content) = &candidate.content {
                for part in &content.parts {
                    if let Some(function_call) = &part.function_call {
                        function_name = function_call.name.clone();
                        product_name = function_call.args.get("product_name").unwrap_or(&json!("")).to_string();
                        println!("Function name: {}", function_name);
                        println!("Product name: {}", product_name);
                    }
                }
            }
        }
    };

    let function_call_assistant = FunctionCall {
        name: function_name.clone(),
        args: json!({
            "product_name": product_name
        }),
    };

    let assistant_response = Part {
        text: None,
        function_call: Some(function_call_assistant),
        function_response: None,
        inline_data: None,
        file_data: None,
    };

    let content_response = json!({"sku": "GA04834-US", "stock": -2});
    println!("\n\nSample response: {}", content_response);

    let function_content = FunctionContent {
        name: function_name.clone(),
        content: content_response,
    };

    let function_response = FunctionResponse {
        name: function_name,
        response: function_content,
    };

    let response = llm
        .with_assistant_response(vec![assistant_response])
        .with_function_response(function_response)
        .with_tools(function_dec)
        .invoke(question)
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