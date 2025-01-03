#[allow(dead_code)]
use crate::replicate::ReplicateModels;
use std::time::Instant;
use serde_json::json;

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    // let llm = ReplicateModels::new("predictions")?;
    // let input_data = json!({
    //     "version": "b063023ee937f28e922982abdbf97b041ffe34ad3b35a53d33e1d74bb19b36c4",
    //     "input": {
    //         "prompt": "I forgot how to kill a process in Linux, can you help?",
    //         "assistant": "Sure! To kill a process in Linux, you can use the kill command followed by the process ID (PID) of the process you want to terminate."
    //     }
    // });

    let llm = ReplicateModels::new("predictions")?;
    let input_data = json!({
        "version": "a06276a89f1a902d5fc225a9ca32b6e8e6292b7f3b136518878da97c458e2bad",
        "input": {
            "texts": "[\"In the water, fish are swimming.\", \"Fish swim in the water.\", \"A book lies open on the table.\"]",
            "batch_size": 32,
            "normalize_embeddings": true
        }
    });

    let start = Instant::now();
    let response = llm.invoke(input_data).await?;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[Task took {:.2} seconds]", elapsed);

    println!("\n#### Example Replicate Models ####");
    match &response.output {
        Some(output) => {
            println!("Output: {}", output);
            println!("{:?}", response);            
        }
        None => {
            println!("{:?}", response);
        }
    }    
    Ok(())
}