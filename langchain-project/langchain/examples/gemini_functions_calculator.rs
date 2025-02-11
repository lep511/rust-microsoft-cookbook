use langchain::gemini::chat::ChatGemini;
use langchain::gemini::libs::ChatResponse;
use env_logger::Env;
use serde_json::json;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Unknown,
}

fn calculator(
    operation: Operation, 
    operand1: f64, 
    operand2: f64
) -> f64 {
    match operation {
        Operation::Add => operand1 + operand2,
        Operation::Subtract => operand1 - operand2,
        Operation::Multiply => operand1 * operand2,
        Operation::Divide => operand1 / operand2,
        Operation::Unknown => panic!("Unknown operation"),
    }
}

fn get_function_call_args(
    response: &ChatResponse
) -> Option<&serde_json::Map<String, serde_json::Value>> {
    response.candidates.as_ref()?
        .iter()
        .find_map(|c| c.content.as_ref())?
        .parts.iter()
        .find_map(|p| p.function_call.as_ref())?
        .args.as_object()
}

fn get_arg_value(
    args: &Option<&serde_json::Map<String, serde_json::Value>>, 
    key: &str
) -> String {
    args.and_then(|a| a.get(key))
        .unwrap_or(&json!(""))
        .to_string()
}

async fn example_tools() -> Result<(), Box<dyn std::error::Error>> {

    let llm = ChatGemini::new("gemini-2.0-flash-exp");
    
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

    let args = get_function_call_args(&response);

    let operation = args
        .and_then(|a| a.get("operation"))
        .map(|op| match op.as_str().unwrap_or("") {
            "Add" => Operation::Add,
            "Subtract" => Operation::Subtract,
            "Multiply" => Operation::Multiply,
            "Divide" => Operation::Divide,
            _ => Operation::Unknown,
        })
        .unwrap_or(Operation::Unknown);

    let operand1_string = get_arg_value(&args, "operand1");
    let operand2_string = get_arg_value(&args, "operand2");

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