use langchain::anthropic::chat::ChatAnthropic;
use serde_json::json;
use std::fs;

pub async fn article_summarization() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatAnthropic::new("claude-3-5-haiku-20241022")?;

    let file_path = "tests/files/anthropic_web_scraping.txt";

    let content = fs::read_to_string(file_path)?;

    let tool_data = json!({
        "name":"print_summary",
        "description":"Prints a summary of the article.",
        "input_schema":{
            "type":"object",
            "properties":{
                "author":{
                    "type":"string",
                    "description":"Name of the article author"
                },
                "topics":{
                    "type":"array",
                    "items":{
                        "type":"string"
                    },
                    "description":"Array of topics, e.g. [\"tech\", \"politics\"]. Should be as specific as possible, and can overlap."
                },
                "summary":{
                    "type":"string",
                    "description":"Summary of the article. One or two paragraphs max."
                },
                "coherence":{
                    "type":"integer",
                    "description":"Coherence of the article's key points, 0-100 (inclusive)"
                },
                "persuasion":{
                    "type":"number",
                    "description":"Article's persuasion score, 0.0-1.0 (inclusive)"
                }
            },
            "required":[
                "author",
                "topics",
                "summary",
                "coherence",
                "persuasion",
                "counterpoint"
            ]
        }
    });

    let tools = vec![tool_data];
    let tool_choice = Some(json!({"type": "tool", "name": "print_summary"}));
    let prompt = format!(
        "<article>\n{}\n</article>\n\nUse the 'print_summary' tool.",
        content,
    );

    let response = llm
        .with_tools(
            tools, 
            tool_choice,
        )
        .with_max_tokens(4096)
        .with_retry(0)
        .invoke(&prompt)
        .await?;

    if let Some(candidates) = &response.content {
        for candidate in candidates {
            if candidate.content_type == "tool_use" {
                println!("Result: {:?}", candidate.input);
            }
        }
    };

    Ok(())
}

pub async fn entity_recognition() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatAnthropic::new("claude-3-5-haiku-20241022")?;

    let tool_data = json!({
        "name":"print_entities",
        "description":"Prints extract named entities.",
        "input_schema":{
            "type":"object",
            "properties":{
                "entities":{
                    "type":"array",
                    "items":{
                        "type":"object",
                        "properties":{
                            "name":{
                                "type":"string",
                                "description":"The extracted entity name."
                            },
                            "type":{
                                "type":"string",
                                "description":"The entity type (e.g., PERSON, ORGANIZATION, LOCATION)."
                            },
                            "context":{
                                "type":"string",
                                "description":"The context in which the entity appears in the text."
                            }
                        },
                        "required":[
                            "name",
                            "type",
                            "context"
                        ]
                    }
                }
            },
            "required":[
                "entities"
            ]
        }
    });

    let tools = vec![tool_data];
    let tool_choice = Some(json!({"type": "tool", "name": "print_entities"}));
    let text = "John works at Google in New York. He met with Sarah, the CEO of Acme Inc., last week in San Francisco.";
    let prompt = format!(
        "<document>\n{}\n</document>\n\nUse the 'print_entities' tool.",
        text,
    );

    let response = llm
        .with_tools(
            tools, 
            tool_choice,
        )
        .with_max_tokens(4096)
        .with_retry(0)
        .invoke(&prompt)
        .await?;

    if let Some(candidates) = &response.content {
        for candidate in candidates {
            if candidate.content_type == "tool_use" {
                println!("Result: {:?}", candidate.input);
            }
        }
    };

    Ok(())
}

pub async fn unknown_keys() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatAnthropic::new("claude-3-5-haiku-20241022")?;

    let tool_data = json!({
        "name":"print_all_characteristics",
        "description":"Prints all characteristics which are provided.",
        "input_schema":{
            "type":"object",
            "additionalProperties": true
        }
    });

    let tools = vec![tool_data];
    let tool_choice = Some(json!({"type": "tool", "name": "print_all_characteristics"}));
    
    let prompt = "Given a description of a character, your task is to extract all the \
                characteristics of the character and print them using the print_all_characteristics tool. \
                \
                The print_all_characteristics tool takes an arbitrary number of inputs where the key is the \
                characteristic name and the value is the characteristic value (age: 28 or eye_color: green). \
                \
                <description> \
                The man is tall, with a beard and a scar on his left cheek. He has a deep voice and \
                wears a black leather jacket. \
                </description> \
                \
                Now use the print_all_characteristics tool.";

    let response = llm
        .with_tools(
            tools, 
            tool_choice,
        )
        .with_max_tokens(4096)
        .with_retry(0)
        .invoke(prompt)
        .await?;

    if let Some(candidates) = &response.content {
        for candidate in candidates {
            if candidate.content_type == "tool_use" {
                println!("Result: {:?}", candidate.input);
            }
        }
    };

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // article_summarization().await?
    // entity_recognition().await?;
    unknown_keys().await?;
    
    Ok(())
}