use langchain::langsmith::client::LangsmithClient;
use langchain::langsmith::libs::LangsmithResponse;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let dataset_name = "xxSampleDatasetData";
    let mut dataset_id = String::from("");

    let client = LangsmithClient::new()?;

    let response: LangsmithResponse = client
        .clone()
        .get_dataset(dataset_name)
        .invoke()
        .await?;

    println!("Response: {:?}", response);

    if let LangsmithResponse::CreateDataset(result) = response {
        if let Some(id) = result.id {
            dataset_id = id.clone();
        }
    } else {
        let response: LangsmithResponse = client
            .clone()
            .create_dataset(dataset_name)
            .with_description("A new dataset")
            .invoke()
            .await?;

        println!("Response: {:?}", response);

        if let LangsmithResponse::CreateDataset(result) =  response {
            if let Some(id) = result.id {
                dataset_id = id.clone();
            }
        }
    }

    let input = json!({"text": "You're a wonderful person"});
    let output = json!({"label": "Not toxic"});

    let response: LangsmithResponse = client
        .create_example(
            &dataset_id,
            input,
            output,
        )
        .invoke()
        .await?;

    println!("Response: {:?}", response);

    Ok(())
}