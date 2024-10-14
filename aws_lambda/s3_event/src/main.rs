use aws_lambda_events::event::s3::S3Event;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use aws_sdk_s3::Client;
use aws_sdk_s3::types::{ByteStream, PutObjectInput};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyData {
    id: u32,
    name: String,
    value: f64,
}

async fn function_handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    // Get the S3 event details
    let record = event.payload.records.first().ok_or("No S3 event record found")?;
    let bucket_name = record.s3.bucket.name.as_ref().ok_or("No bucket name found")?;
    let object_key = record.s3.object.key.as_ref().ok_or("No object key found")?;

    // Create an AWS SDK client
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // Download the object from S3
    let get_object_output = client.get_object().bucket(bucket_name).key(object_key).send().await?;
    let body = get_object_output.body.collect().await?;

    // Deserialize the data from JSON
    let data: Vec<MyData> = serde_json::from_slice(&body)?;

    // Modify the data as needed
    let modified_data = data.iter().map(|d| MyData {
        id: d.id,
        name: format!("Modified {}", d.name),
        value: d.value * 2.0,
    }).collect::<Vec<_>>();

    // Serialize the modified data to JSON
    let json_data = serde_json::to_vec(&modified_data)?;

    // Upload the modified data to S3
    let put_object_input = PutObjectInput::builder()
        .bucket(bucket_name)
        .key(format!("modified/{}", object_key))
        .body(ByteStream::from(json_data))
        .build();
    client.put_object(put_object_input).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
