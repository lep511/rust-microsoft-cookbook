use langchain::anthropic::chat::ChatAnthropic;
use env_logger::Env;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatAnthropic::new("claude-3-5-sonnet-20241022")?;

    let tool_data = json!({
        "name":"record_summary",
        "description":"Record summary of an image using well-structured JSON.",
        "input_schema":{
            "type":"object",
            "properties":{
                "key_colors":{
                    "type":"array",
                    "items":{
                        "type":"object",
                        "properties":{
                            "r":{
                                "type":"number",
                                "description":"red value [0.0, 1.0]"
                            },
                            "g":{
                                "type":"number",
                                "description":"green value [0.0, 1.0]"
                            },
                            "b":{
                                "type":"number",
                                "description":"blue value [0.0, 1.0]"
                            },
                            "name":{
                                "type":"string",
                                "description":"Human-readable color name in snake_case, e.g. \"olive_green\" or \"turquoise\""
                            }
                        },
                        "required":[
                            "r",
                            "g",
                            "b",
                            "name"
                        ]
                    },
                    "description":"Key colors in the image. Limit to less then four."
                },
                "description":{
                    "type":"string",
                    "description":"Image description. One to two sentences max."
                },
                "estimated_year":{
                    "type":"integer",
                    "description":"Estimated year that the images was taken, if is it a photo. Only set this if the image appears to be non-fictional. Rough estimates are okay!"
                }
            },
            "required":[
                "key_colors",
                "description"
            ]
        }
    });

    let tools = vec![tool_data];
    let tool_choice = Some(json!({"type": "tool", "name": "record_summary"}));
    let file_path = "tests/files/image01.jpg";
    let mime_type = "image/jpeg";
    let prompt = "Describe this image.";

    let response = llm
        .with_image_file(file_path, mime_type)
        .with_tools(tools, tool_choice)
        .invoke(prompt)
        .await?;

    if let Some(contents) = &response.content {
        println!("Content: {:?}", contents);
    };
    
    Ok(())
}