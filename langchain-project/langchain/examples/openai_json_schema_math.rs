#[allow(dead_code)]
use langchain::openai::chat::ChatOpenAI;
use langchain::openai::libs::ChatResponse;
use langchain::openai::utils::generate_schema;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use env_logger::Env;

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
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
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

    let system_prompt = "You are a helpful math tutor. You will be provided with \
                a math problem, and your goal will be to output a step by step \
                solution, along with a final answer. \
                For each step, just provide the output as an equation use the explanation \
                field to detail the reasoning.";

    let prompt = "How can I solve 8x + 7 = -23";

    let response: ChatResponse = llm
        .with_system_prompt(system_prompt)    
        .with_json_schema(json_schema)
        .with_max_retries(0)
        .invoke(prompt)
        .await?;

    match response.choices {
        Some(candidates) => {
            candidates.iter()
                .filter_map(|candidate| candidate
                    .message.as_ref()?
                    .content.as_ref()
                ).for_each(|content| println!("{}", content));
        }
        None => println!("No response choices available"),
    };
    
    Ok(())
}