#[allow(dead_code)]
use langchain::openai::chat::ChatOpenAI;
use langchain::openai::libs::ChatResponse;
use serde_json::json;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatOpenAI::new("gpt-4o-mini")?;
    
    let system_prompt = "You extract email addresses into JSON data.";

    let prompt = "Feeling stuck? Send a message to help@mycompany.com.";

    let start = Instant::now();

    let json_schema = json!({
        "name":"email_schema",
        "schema":{
            "type":"object",
            "properties":{
                "email":{
                    "description":"The email address that appears in the input",
                    "type":"string"
                }
            },
            "additionalProperties": false
        }
    });

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