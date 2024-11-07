use std::collections::HashMap;
use std::error::Error;
use reqwest::blocking::Client;
use bson::{doc, Document};
use bson::de::from_bson;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InstanceData {
    instance_size: String,
    maximum_bandwidth_mbps: u32,
    maximum_throughput_mbs: f32,
    maximum_iops: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://aws-well-architected-labs.s3.us-west-2.amazonaws.com/Cost/Labs/iops/ec2-ebs-optimization-maximums.json";
    let client = Client::new();
    let response = client.get(url).send()?;
    let body = response.bytes()?;

    // Convert JSON to BSON
    let bson_data: Vec<Document> = bson::from_slice(&body)?;

    // Process the BSON data
    for bson_instance in bson_data {
        let instance: InstanceData = from_bson(bson::Bson::Document(bson_instance)).expect("Failed to deserialize BSON document");
        println!(
            "Instance size: {}, Max Bandwidth: {} Mbps, Max Throughput: {} MB/s, Max IOPS: {}",
            instance.instance_size, instance.maximum_bandwidth_mbps, instance.maximum_throughput_mbs, instance.maximum_iops
        );
    }

    // You can also convert the Vec<InstanceData> to a HashMap for easier lookup
    let instance_map: HashMap<&str, InstanceData> = bson_data
        .iter()
        .map(|doc| {
            let instance: InstanceData = from_bson(bson::Bson::Document(doc.clone())).expect("Failed to deserialize BSON document");
            (instance.instance_size.as_str(), instance)
        })
        .collect::<Result<_, _>>()?;

    println!("\nInstance map: {:?}", instance_map);

    Ok(())
}