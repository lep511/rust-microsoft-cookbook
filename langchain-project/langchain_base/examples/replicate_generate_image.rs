#[allow(dead_code)]
use langchain_base::replicate::ReplicateModels;
use std::time::Instant;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ReplicateModels::new("models/black-forest-labs/flux-1.1-pro-ultra/predictions")?;
    let input_data = json!({
        "input": {
            "raw": false,
            "prompt": "a majestic snow-capped mountain peak bathed in a warm glow of the setting sun",
            "aspect_ratio": "3:2",
            "output_format": "jpg",
            "safety_tolerance": 2,
            "image_prompt_strength": 0.1
            }
    });

    let start = Instant::now();
    let response = llm.clone().invoke(input_data).await?;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]", elapsed);

    println!("\n#### Example Replicate Image Generated ####");
    // match &response.output {
    //     Some(output) => {
    //         println!("Output: {}", output);
    //         println!("{:?}", response);
    //         if output.contains("https://replicate") {
    //             let file_name = llm.get_file(&output).await?;
    //             println!("File saved: {}", file_name);
    //         }
    //     }
    //     None => {
    //         println!("{:?}", response);
    //     }
    // }
    println!("{:?}", response);
    Ok(())
}