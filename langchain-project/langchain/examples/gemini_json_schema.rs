#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use env_logger::Env;
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

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct SimpleRecipe {
    pub recipe_name: String,
    pub grade: Grade,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub enum Grade {
    A,
    B,
    C,
    D,
    F,
}


async fn sample_json_response() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let prompt = "List about 10 cookie recipes, grade them based on popularity";

    // Generate schema for Grade
    let schema_grade = schemars::schema_for!(Grade);
    let json_schema_grade = generate_schema(schema_grade, true)?;
    
    // Generate schema for SimpleRecipe
    let schema_recipe = schemars::schema_for!(SimpleRecipe);
    let mut json_schema_rec = generate_schema(schema_recipe, false)?;
    json_schema_rec["properties"]["grade"] = json_schema_grade;

    let response = llm
        .with_json_schema(json_schema_rec)
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

async fn sample_with_image() -> Result<(), Box<dyn std::error::Error>> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // sample_with_image().await?;
    sample_json_response().await?;

    Ok(())
}