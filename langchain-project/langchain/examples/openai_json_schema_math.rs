#[allow(dead_code)]
use langchain::openai::chat::ChatOpenAI;
use langchain::openai::libs::ChatResponse;
use langchain::openai::utils::generate_schema;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
struct MathReasoning {
    steps: Vec<Step>,
    final_answer: String,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
struct Step {
    explanation: String,
    output: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatOpenAI::new("gpt-4o-mini")?;

    let name_schema = "math_reasoning";
    let is_strict = true;
    let is_additional_properties = false;
    let is_sub_struct = true;
    
    // Generate schema for Step
    let schema_step = schemars::schema_for!(Step);
    let json_schema_step = generate_schema(
        schema_step,
        name_schema,
        is_strict,
        is_additional_properties,
        is_sub_struct,
    )?;

    let is_sub_struct = false;

    // Generate schema for MathReasoning
    let schema_mathr = schemars::schema_for!(MathReasoning);
    let mut json_schema = generate_schema(
        schema_mathr,
        name_schema,
        is_strict,
        is_additional_properties,
        is_sub_struct,
    )?;
    json_schema["schema"]["properties"]["steps"] = json_schema_step;

    let prompt = "You are a helpful math tutor. You will be provided with a math problem, \
                and your goal will be to output a step by step solution, along with \
                a final answer. \
                For each step, just provide the output as an equation use the explanation \
                field to detail the reasoning.";

    let response: ChatResponse = llm
        .with_json_schema(json_schema)
        .with_retry(0)
        .invoke(prompt)
        .await?;


    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let Some(content) = candidate.message.content {
                    println!("{}", content);
                }
            }
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}