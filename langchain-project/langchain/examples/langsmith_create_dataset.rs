use langchain::langsmith::client::LangsmithClient;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let dataset_name = "SampleDatasetData";
    let mut dataset_id = String::from("");

    let client = LangsmithClient::new()?;

    let response: Value = client
        .clone()
        .get_dataset(dataset_name)
        .invoke()
        .await?;

    match serde_json::to_string_pretty(&response) {
        Ok(json) => println!("Pretty-printed JSON:\n{}", json),
        Err(e) => println!("[ERROR] {:?}", e)
    }

    match response[0].get("id") {
        Some(id) => {
            if let Some(id) = id.as_str() {
                println!("\n\nDataset already exists: {}", id);
                dataset_id = id.to_string();
            }
        },
        None => {
            let response: Value = client
                .clone()
                .create_dataset(dataset_name)
                .with_description("A new dataset")
                .invoke()
                .await?;

            match serde_json::to_string_pretty(&response) {
                Ok(json) => println!("Pretty-printed JSON:\n{}", json),
                Err(e) => println!("[ERROR] {:?}", e)
            }

            if let Some(id) = response["id"].as_str() {
                dataset_id = id.to_string();
            }
        }
    }
    
    let input = json!({"text": "You're a wonderful person"});
    let output = json!({"label": "Not toxic"});

    let response: Value = client
        .create_example(
            &dataset_id,
            input,
            output,
        )
        .invoke()
        .await?;

    match serde_json::to_string_pretty(&response) {
        Ok(json) => println!("Pretty-printed JSON:\n{}", json),
        Err(e) => println!("[ERROR] {:?}", e)
    }

    Ok(())
}