use aws_config::{load_defaults, BehaviorVersion};
use tracing::{info, error};

mod libs;
mod core;
use core::process_file;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();



    // Extract bucket name and object key - see http_handler.rs file
    // let bucket_name = &s3_event.detail.bucket.name;
    // let object_key = &s3_event.detail.object.key;

    let bucket_name = "data-lake-bucket-raw-49583";
    let object_key = "base_datos_2009.csv";

    let config = load_defaults(BehaviorVersion::latest()).await;
    let s3_client = aws_sdk_s3::Client::new(&config);
    
    match process_file(
        &s3_client,
        bucket_name,
        object_key,
    ).await {
        Ok(_) => info!("File processed successfully"),
        Err(e) => error!("Error processing file: {}", e),
    }
    
    Ok(())
}