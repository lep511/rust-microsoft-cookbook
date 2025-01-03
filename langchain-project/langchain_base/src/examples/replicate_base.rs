#[allow(dead_code)]
use crate::replicate::ReplicateModels;
use std::time::Instant;
use serde_json::json;

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ReplicateModels::new("models/meta/meta-llama-3-70b-instruct/predictions")?;
    let input_data = json!({
        "input": {
            "top_k":0,
            "top_p":0.9,
            "prompt":"Work through this problem step by step:\\n\\nQ: Sarah has 7 llamas. Her friend gives her 3 more trucks of llamas. Each truck has 5 llamas. How many llamas does Sarah have in total?",
            "max_tokens":512,
            "min_tokens":0,
            "temperature":0.6,
            "system_prompt":"You are a helpful assistant",
            "length_penalty":1,
            "stop_sequences":"<|end_of_text|>,<|eot_id|>",
            "prompt_template":"<|begin_of_text|><|start_header_id|>system<|end_header_id|>\\n\\nYou are a helpful assistant<|eot_id|><|start_header_id|>user<|end_header_id|>\\n\\n{prompt}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\\n\\n",
            "presence_penalty":1.15,
            "log_performance_metrics":false
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