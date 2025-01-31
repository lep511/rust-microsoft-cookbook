use langchain::gemini::chat::ChatGemini;
use env_logger::Env;
use serde_json::json;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Unknown,
}

fn calculator(operation: Operation, operand1: f64, operand2: f64) -> f64 {
    match operation {
        Operation::Add => operand1 + operand2,
        Operation::Subtract => operand1 - operand2,
        Operation::Multiply => operand1 * operand2,
        Operation::Divide => operand1 / operand2,
        Operation::Unknown => panic!("Unknown operation"),
    }
}

async fn example_tools() -> Result<(), Box<dyn std::error::Error>> {

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;
    
    let function_schema = json!({
        "name":"calculator",
        "description":"A simple calculator that performs basic arithmetic operations.",
        "parameters":{
            "type":"OBJECT",
            "properties":{
                "operation":{
                    "type":"STRING",
                    "description": "The arithmetic operation to perform.",
                    "enum": ["Add", "Subtract", "Multiply", "Divide"],
                },
                "operand1":{
                    "type":"NUMBER",
                    "description":"The first operand.",
                },
                "operand2":{
                    "type":"NUMBER",
                    "description":"The second operand.",
                }
            },
            "required": ["operation", "operand1", "operand2"]
        }
    });

    let function_dec = vec![json!({
        "functionDeclarations":[
            function_schema,
        ]
    })];

    let tool_config = json!({
        "function_calling_config":{
            "mode":"ANY",
            "allowed_function_names":[
                "calculator"
            ]
        }
    });

    let question = "Multiply 1984135 by 9343116. Only respond with the result";
    
    let response = llm.clone()
        .with_temperature(0.0)
        .with_tools(function_dec.clone())
        .with_tool_config(tool_config)
        .invoke(question)
        .await?;
    
    let mut operation: Operation = Operation::Add;
    let mut operand1_string = String::new();
    let mut operand2_string = String::new();

    println!("Question: {}", question);
    if let Some(candidates) = &response.candidates {
        for candidate in candidates {
            if let Some(content) = &candidate.content {
                for part in &content.parts {
                    if let Some(function_call) = &part.function_call {
                        // let get_operation = function_call.args.get("operation").unwrap_or(&json!(""));
                        if let Some(get_operation) = function_call.args.get("operation") {
                            match get_operation.as_str().unwrap_or("") {
                                "Add" => operation = Operation::Add,
                                "Subtract" => operation = Operation::Subtract,
                                "Multiply" => operation = Operation::Multiply,
                                "Divide" => operation = Operation::Divide,
                                _ => operation = Operation::Unknown,
                            }
                        }
                        operand1_string = function_call.args.get("operand1").unwrap_or(&json!("")).to_string();
                        operand2_string = function_call.args.get("operand2").unwrap_or(&json!("")).to_string();
                    }
                }
            }
        }
    };

    // Convert String to f64
    let operand1: f64 = operand1_string.parse().unwrap();
    let operand2: f64 = operand2_string.parse().unwrap();

    let result = calculator(operation, operand1, operand2);
    println!("The result is: {}", result);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    example_tools().await?;
    Ok(())
}