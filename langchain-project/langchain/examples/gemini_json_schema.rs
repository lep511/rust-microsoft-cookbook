#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use serde::{Deserialize, Serialize};
use langchain::gemini::utils::generate_schema;
use schemars::JsonSchema;

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct Recipe {
    pub title: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
    pub notes: String,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let prompt = "From the image detail the recipe to bake this item, include item names and quantities for the recipe";

    // Generate schema for Ingredient
    let schema_ingedient = schemars::schema_for!(Ingredient);
    let json_schema_ing = generate_schema(schema_ingedient, true)?;

    // Generate schema for Recipe
    let schema_recipe = schemars::schema_for!(Recipe);
    let mut json_schema_rec = generate_schema(schema_recipe, false)?;
    json_schema_rec["properties"]["ingredients"] = json_schema_ing;

    let file_path = Some("tests/files/croissants.jpg");
    let upload_data = None;
    let display_name = "croissants.jpg";
    let mime_type = "auto";

    let response = llm
        .with_json_schema(json_schema_rec)
        .media_upload(
            file_path,
            upload_data,
            display_name,
            mime_type,
        )
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