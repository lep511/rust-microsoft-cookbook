#[allow(dead_code)]
use langchain::openai::chat::ChatOpenAI;
use langchain::openai::libs::ChatResponse;
use langchain::openai::utils::generate_schema;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use env_logger::Env;

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
struct EmailSchema {
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatOpenAI::new("gpt-4o-mini")?;
    
    let system_prompt = "You extract email addresses into JSON data.";

    let prompt = "Feeling stuck? Send a message to help@mycompany.com.";

    let start = Instant::now();

    let name_schema = "email_schema";
    let is_strict = true;
    let is_additional_properties = false;
    let is_sub_struct = false;

    // Generate schema for EmailSchema
    let schema_data = schemars::schema_for!(EmailSchema);
    let json_schema = generate_schema(
        schema_data,
        name_schema,
        is_strict,
        is_additional_properties,
        is_sub_struct,
    )?;

    let response: ChatResponse = llm
        .with_system_prompt(system_prompt)
        .with_json_schema(json_schema)
        .invoke(prompt)
        .await?;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]\n", elapsed);

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