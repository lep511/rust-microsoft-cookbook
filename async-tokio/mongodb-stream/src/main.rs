use aws_config::{load_defaults, BehaviorVersion};
use std::sync::Arc;
use tracing::{info, error};

mod libs;
use libs::{FlightData, MongoPool};
mod core;
use core::process_s3_csv_file;
mod error;
use error::AppError;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    
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
    let mongo_pool = MongoPool::new().await?;
    let arc_mongo_pool = Arc::new(mongo_pool);
    let error_file_path = Some("/xerror-csv");
    
    match process_s3_csv_file(
        &s3_client,
        arc_mongo_pool,
        bucket_name,
        object_key,
        error_file_path,
    ).await {
        Ok(_) => info!("File processed successfully"),
        Err(e) => error!("Error processing file: {}", e),
    }
    
    Ok(())
}