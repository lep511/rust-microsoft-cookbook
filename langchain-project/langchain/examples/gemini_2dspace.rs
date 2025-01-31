#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use env_logger::Env;
use langchain::gemini::libs::{SafetySetting, HarmCategory, HarmBlock};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Detection {
    box_2d: Vec<i32>,
    label: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let prompt = "Detect the 2d bounding boxes of the cupcakes (with \"label\" as topping description)";

    let system_prompt = "Return bounding boxes as a JSON array with labels. \
            Never return masks or code fencing. Limit to 25 objects. If an object is present  \
            multiple times, name them according to their unique characteristic  \
            (colors, size, position, unique characteristics, etc..)";
    
            let safety_settings = vec![
        SafetySetting {
            category: HarmCategory::HarmCategoryDangerousContent,
            threshold: HarmBlock::BlockOnlyHigh,
        }
    ];

    let file_path = "tests/files/cupcackes.png";

    let response = llm
        .with_safety_settings(safety_settings)
        .media_upload(file_path, "auto")
        .await?
        .with_system_prompt(system_prompt)
        .with_temperature(0.5)
        .invoke(prompt)
        .await?;
    
    let mut response_string = String::from("");

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        response_string.push_str(&text);
                    }
                }
            }
        }
    };

    let json_str = response_string
        .trim_start_matches("```json")
        .trim_end_matches("```")
        .trim();
     
    // Parse JSON into Vec<Detection>
    let detections: Vec<Detection> = serde_json::from_str(json_str)?;

    // Process the detections
    for (index, detection) in detections.iter().enumerate() {
        let [x1, y1, x2, y2] = detection.box_2d.as_slice() else {
            println!("Invalid bounding box format");
            continue;
        };
        
        // Calculate width and height
        let width = x2 - x1;
        let height = y2 - y1;
        let area = width * height;
        
        println!("Detection #{}", index + 1);
        println!("Label: {}", detection.label);
        println!("Position: ({}, {}), ({}, {})", x1, y1, x2, y2);
        println!("Dimensions: {}x{} pixels", width, height);
        println!("Area: {} square pixels", area);
        println!("---");
    }

    Ok(())
}