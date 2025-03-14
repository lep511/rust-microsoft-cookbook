use lambda_runtime::{Error, LambdaEvent};
use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use aws_config::{load_defaults, BehaviorVersion};
use tokio::time::Instant;
use serde::{Serialize, Deserialize};
use crate::core_s3::{process_small_files, process_large_files};
use tracing::{info, error};

// Define types that match the S3 event structure
#[derive(Deserialize, Serialize, Debug)]
pub struct S3EventDetail {
    pub version: String,
    pub bucket: S3Bucket,
    pub object: S3Object,
    #[serde(rename = "request-id")]
    pub request_id: String,
    pub requester: String,
    #[serde(rename = "source-ip-address")]
    pub source_ip_address: String,
    pub reason: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct S3Bucket {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct S3Object {
    pub key: String,
    pub size: u64,
    pub etag: String,
}

pub(crate)async fn function_handler(
    event: LambdaEvent<EventBridgeEvent<S3EventDetail>>
) -> Result<(), Error> {
    // Extract some useful information from the request
    let s3_event = event.payload;
    info!("Payload: {:?}", s3_event);

    // Extract bucket name and object key
    let bucket_name = &s3_event.detail.bucket.name;
    let object_key = &s3_event.detail.object.key;
    
    info!("Bucket name: {:?}", bucket_name);
    info!("Object key: {:?}", object_key);

    let config = load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);

    // Get object size first
    let head_object = client
        .head_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await?;

    let content_length: usize = head_object
        .content_length()
        .map_or_else(
            || {
                error!("Content length not found");
                0
            },
            |len| len as usize,
        );
    
    let content_length_mb = content_length as f64 / 1_048_576.0;
    info!("File size: {:.2} MB", content_length_mb);
    
    let start = Instant::now();

    // Choose processing strategy based on file size
    if content_length < 100_000_000 { // Less than 100MB
        let response = match process_small_files(
            &client, 
            bucket_name, 
            object_key,
            content_length,
        ).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error processing small files: {:?}", e);
                return Err(e.into());
            }
        };

        let response_string = response.iter()
            .map(|(airport, count)| format!("{}: {} flights", airport, count))
            .collect::<Vec<String>>()
            .join(", ");

        info!("Departure counts: {}", response_string);
    
    } else {
        let response = match process_large_files(
            &client,
            bucket_name,
            object_key,
        ).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error processing large files: {:?}", e);
                return Err(e.into());
            }
        };
        
        let response_string = response.iter()
            .map(|(airport, count)| format!("{}: {} flights", airport, count))
            .collect::<Vec<String>>()
            .join(", ");

        info!("Departure counts: {}", response_string);
    }

    let elapsed_seconds = start.elapsed().as_secs();
    info!("Operation took: {} whole seconds", elapsed_seconds);
    
    Ok(())
}