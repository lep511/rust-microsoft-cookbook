#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let prompt = "From the image detail the recipe to bake this item, include item names and quantities for the recipe";
    let file_path = "tests/files/croissants.jpg";

    let json_schema = json!({
        "type":"object",
        "properties":{
            "title":{
                "type":"string"
            },
            "ingredients":{
                "type":"array",
                "items":{
                    "type":"object",
                    "properties":{
                        "name":{
                            "type":"string"
                        },
                        "quantity":{
                            "type":"string"
                        }
                    }
                }
            },
            "instructions":{
                "type":"array",
                "items":{
                    "type":"string"
                }
            },
            "notes":{
                "type":"string"
            }
        },
        "required":[
            "title",
            "ingredients",
            "instructions"
        ]
    });

    let response = llm
        .with_json_schema(json_schema)
        .media_upload(file_path, "auto")
        .await?
        .invoke(prompt)
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