use langchain::gemini::chat::ChatGemini;
use serde_json::{json, Value};

async fn example_tools() -> Result<(), Box<dyn std::error::Error>> {

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;
    
    let question = "Look up the weather in Paris and set my climate control appropriately.";
    
    let response = llm
        .with_google_search()
        .invoke(question)
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

    let function_climate = json!({
        "name":"set_climate",
        "description":"Switches the local climate control equipment to the specified parameters.",
        "parameters":{
            "type":"OBJECT",
            "properties":{
                "mode":{
                    "type":"STRING",
                    "description":"Mode for the climate unit - whether to heat, cool or just blow air.",
                    "enum":[
                        "hot",
                        "cold",
                        "fan",
                        "off"
                    ]
                },
                "strength":{
                    "type":"INTEGER",
                    "description":"Intensity of the climate to apply, 0-10 (0 is off, 10 is MAX)."
                }
            }
        }
    });

    let function_dec = vec![json!({
        "functionDeclarations":[
            function_climate
        ]
    })];

    let tool_config = json!({
        "function_calling_config":{
            "mode":"ANY",
            "allowed_function_names":[
                "set_climate"
            ]
        }
    });

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let response = llm
        .with_tools(function_dec)
        .with_tool_config(tool_config)
        .invoke(&response_string)
        .await?;
    
    let mut function_name = String::new();
    let mut function_args = Value::Null;

    println!("Question: {}", question);
    if let Some(candidates) = &response.candidates {
        for candidate in candidates {
            if let Some(content) = &candidate.content {
                for part in &content.parts {
                    if let Some(function_call) = &part.function_call {
                        function_name = function_call.name.clone();
                        function_args = function_call.args.clone();
                        println!("Function name: {}", function_name);
                        println!("Functions args: {:?}", function_args);
                    }
                }
            }
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    example_tools().await?;
    Ok(())
}