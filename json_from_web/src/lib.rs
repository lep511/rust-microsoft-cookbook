use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Instance {
    pub instance_size: String,
    pub maximum_bandwidth_mbps: u32,
    pub maximum_throughput_mbs: f64,
    pub maximum_iops: u32,
}

pub async fn get_instance() -> Result<Vec<Instance>, Box<dyn std::error::Error>> {
    let url = "https://aws-well-architected-labs.s3.us-west-2.amazonaws.com/Cost/Labs/iops/ec2-ebs-optimization-maximums.json";
    
    // Send GET request and get the response
    let response = reqwest::get(url).await?;
    
    // Get the response text
    let content = response.text().await?;

    let mut instances: Vec<Instance> = Vec::new();

    // Process each line as a separate JSON object
    for line in content.lines() {
        match serde_json::from_str::<Instance>(&line) {
            Ok(instance) => instances.push(instance),
            Err(e) => eprintln!("Error parsing line: {}", e),
        }
    }

    Ok(instances)
}