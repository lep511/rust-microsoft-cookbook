use aws_sdk_s3::{Client, Error as S3Error};
use aws_smithy_runtime_api::client::result::SdkError;
use futures::stream::StreamExt;
use mongodb::{
    bson::doc,
    options::{ClientOptions, InsertManyOptions},
    Collection, Client as MongoClient,
};
use mongodb::error::Error as MongoError;
use crate::libs::FlightData;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::task;
use tokio::io::{AsyncBufReadExt, BufReader};
use thiserror::Error;
use tracing::{error, info, instrument};
use rayon::prelude::*;
use std::io;
use std::env;

// Constants
pub const BUFFER_CAPACITY: usize = 10_000; // Process ~10K lines at a time
pub const BATCH_SIZE: usize = 2000;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("S3 error: {0}")]
    S3Error(#[from] S3Error),
    
    #[error("AWS SDK error: {0}")]
    SdkError(String),
    
    #[error("MongoDB error: {0}")]
    MongoError(#[from] MongoError),
        
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("CSV parsing error: {0}")]
    CsvParseError(String),
    
    #[error("Generic error: {0}")]
    Generic(String),
}

impl AppError {
    pub fn generic<T: ToString>(error: T) -> Self {
        AppError::Generic(error.to_string())
    }
    
    pub fn csv_parse<T: ToString>(error: T) -> Self {
        AppError::CsvParseError(error.to_string())
    }
}

// Implement From for SdkError with any operation error and response
impl<E, R> From<SdkError<E, R>> for AppError 
where
    E: std::fmt::Display,
    R: std::fmt::Debug,
{
    fn from(err: SdkError<E, R>) -> Self {
        AppError::SdkError(err.to_string())
    }
}

// MongoDB connection pool
struct MongoPool {
    client: MongoClient,
}

impl MongoPool {
    async fn new() -> Result<Self, AppError> {
        let uri = env::var("MONGODB_SRV")
            .map_err(|_| AppError::generic("MONGODB_SRV environment variable not set"))?;
        
        let mut client_options = ClientOptions::parse(uri).await?;
        
        // Set connection pool options
        client_options.max_pool_size = Some(10);
        client_options.min_pool_size = Some(5);
        
        let client = MongoClient::with_options(client_options)?;
        
        // Test the connection
        client.database("admin").run_command(doc! {"ping": 1}, None).await?;
        info!("Connected to MongoDB");
        
        Ok(Self { client })
    }
    
    fn get_collection(&self) -> Collection<FlightData> {
        self.client
            .database("flights_data")
            .collection("flights")
    }
}

#[instrument(skip(s3_client, mongo_pool), fields(bucket = %bucket_name, key = %object_key))]
pub async fn process_file(
    s3_client: &Client,
    mongo_pool: Arc<MongoPool>,
    bucket_name: &str,
    object_key: &str,
) -> Result<usize, AppError> {
    info!("Starting to process file {}/{}", bucket_name, object_key);
    
    let get_object_output = s3_client
        .get_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await
        .map_err(|e| {
            error!("Error getting object from S3: {}", e);
            e
        })?;

    let reader = BufReader::new(get_object_output.body.into_async_read());
    let mut lines = reader.lines();

    // Skip header
    if let Some(header) = lines.next_line().await? {
        info!("Skipped header: {}", header);
    } else {
        return Err(AppError::generic("File appears to be empty"));
    }

    // Process in chunks
    let mut buffer = Vec::with_capacity(BUFFER_CAPACITY);
    let mut combined_tasks = Vec::new();

    // Set up atomic counter for total records
    let total_records = Arc::new(AtomicUsize::new(0));
    
    // Process lines in chunks
    while let Some(line) = lines.next_line().await? {
        buffer.push(line);
        
        if buffer.len() >= BUFFER_CAPACITY {
            let chunk = std::mem::take(&mut buffer);
            buffer = Vec::with_capacity(BUFFER_CAPACITY);
            let total_records_clone = Arc::clone(&total_records);
            let mongo_pool_clone = Arc::clone(&mongo_pool);

            // Process chunk and save to database in a single task
            let combined_handle = tokio::spawn(async move {
                let processed_records = process_chunk(chunk, total_records_clone).await;
                if let Ok(records) = processed_records {
                    if !records.is_empty() {
                        if let Err(e) = save_to_mongodb(mongo_pool_clone, &records).await {
                            error!("Error saving to MongoDB: {}", e);
                        }
                    }
                }
            });
            
            combined_tasks.push(combined_handle);
        }
    }
    
    // Process remaining lines
    if !buffer.is_empty() {
        let remaining_buffer = buffer;
        let total_records_clone = Arc::clone(&total_records);
        let mongo_pool_clone = Arc::clone(&mongo_pool);

        // Process remaining chunk and save to database
        let combined_handle = tokio::spawn(async move {
            let processed_records = process_chunk(remaining_buffer, total_records_clone).await;
            if let Ok(records) = processed_records {
                if !records.is_empty() {
                    if let Err(e) = save_to_mongodb(mongo_pool_clone, &records).await {
                        error!("Error saving to MongoDB: {}", e);
                    }
                }
            }
        });
        
        combined_tasks.push(combined_handle);
    }
    
    // Wait for all combined tasks to complete
    futures::future::join_all(combined_tasks).await;
    
    let final_count = total_records.load(Ordering::SeqCst);
    info!("Successfully processed {} records from {}/{}", final_count, bucket_name, object_key);
    
    Ok(final_count)
}

async fn process_chunk(
    chunk: Vec<String>,
    total_records: Arc<AtomicUsize>,
) -> Result<Vec<FlightData>, AppError> {
    // Process the chunk for stats in a blocking task
    let processed_chunk = task::spawn_blocking(move || {
        // Process each line in parallel to collect stats and parse records
        let records: Vec<FlightData> = chunk.par_iter()
            .filter_map(|line| {
                let fields: Vec<&str> = line.split(',').collect();
                match FlightData::from_vec(&fields) {
                    Ok(data) => Some(data),
                    Err(e) => {
                        error!("Error parsing line: {}", e);
                        None
                    }
                }
            })
            .collect();
        
        records
    }).await.map_err(|e| AppError::generic(format!("Error in task: {}", e)))?;
    
    // Update total records count
    total_records.fetch_add(processed_chunk.len(), Ordering::SeqCst);
    
    Ok(processed_chunk)
}

#[instrument(skip(mongo_pool, records), fields(record_count = records.len()))]
async fn save_to_mongodb(
    mongo_pool: Arc<MongoPool>,
    records: &[FlightData],
) -> Result<(), AppError> {
    if records.is_empty() {
        return Ok(());
    }
    
    let collection = mongo_pool.get_collection();
    
    // Process in batches with ordered: false for better performance
    let options = InsertManyOptions::builder().ordered(false).build();
    
    // Use chunk size that's appropriate for MongoDB (avoid too large BSON documents)
    for chunk in records.chunks(BATCH_SIZE) {
        info!("Saving batch of {} documents to MongoDB", chunk.len());
        
        match collection.insert_many(chunk, options.clone()).await {
            Ok(result) => {
                info!("Successfully inserted {} documents", result.inserted_ids.len());
            },
            Err(e) => {
                // If it's a bulk write error, some documents might have been inserted
                if let mongodb::error::ErrorKind::BulkWrite(bulk_error) = e.kind.as_ref() {
                    info!(
                        "Partial success: {} documents inserted, {} failed",
                        bulk_error.write_errors.len(),
                        chunk.len() - bulk_error.write_errors.len()
                    );
                } else {
                    return Err(e.into());
                }
            }
        }
    }

    info!("Successfully saved {} records to MongoDB", records.len());
    Ok(())
}