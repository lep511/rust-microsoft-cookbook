#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use schemars::schema::RootSchema;
use serde_json::{Value, json};

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

/// Transforms a JSON schema into a simplified representation
///
/// # Arguments
///
/// * `schema` - A RootSchema instance containing the JSON schema to transform
/// * `sub_struct` - A boolean flag indicating whether to wrap the schema in an array structure
///
/// # Returns
///
/// Returns a Result containing either:
/// * Ok(Value) - A serde_json::Value containing the transformed schema 
/// * Err(Box<dyn Error>) - An error if:
///   - The schema cannot be serialized to JSON
///   - The serialized JSON is not an object
///
pub fn generate_schema(
    schema: RootSchema, 
    sub_struct: bool
) -> Result<Value, Box<dyn std::error::Error>> {
    let response_json = match serde_json::to_value(schema) {
        Ok(value) => value,
        Err(e) => return Err(Box::new(e)),
    };

    let mut response = match response_json.as_object() {
        Some(obj) => obj.clone(),
        None => return Err("Serialized JSON is not an object".into()),
    };

    response.remove("$schema");
    response.remove("title");
    response.remove("definitions");

    if sub_struct {
        response.remove("required");
        return Ok(json!({
            "type": "array",
            "items": response
        }))
    }

    Ok(Value::Object(response))
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