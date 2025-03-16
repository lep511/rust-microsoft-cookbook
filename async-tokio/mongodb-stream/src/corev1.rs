use aws_sdk_s3::{Client, Error as S3Error};
use aws_smithy_runtime_api::client::result::SdkError;
use mongodb::{Collection, Client as MongoClient};
use mongodb::error::Error as MongoError;
use crate::libs::FlightData;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use tokio::task;
use tokio::io::{AsyncBufReadExt, BufReader};
use thiserror::Error;
use tracing::error;
use rayon::prelude::*;
use std::io;
use std::env;

pub const BUFFER_CAPACITY: usize = 10_000; // Process ~10K lines at a time
pub const BATCH_SIZE: usize = 2000;
// pub const MAX_RECORDS: usize = 1000;

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
    
    #[error("Generic error: {0}")]
    Generic(String),
}

impl AppError {
    pub fn generic<T: ToString>(error: T) -> Self {
        AppError::Generic(error.to_string())
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

pub async fn process_file(
    s3_client: &Client,
    bucket_name: &str,
    object_key: &str,
) -> Result<(), AppError> {
    let get_object_output = s3_client
        .get_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await
        .map_err(|e| {
            error!("Error getting object: {}", e);
            e
        })?;

    let reader = BufReader::new(get_object_output.body.into_async_read());
    let mut lines = reader.lines();

    // Skip header
    let _ = lines.next_line().await?;

    // Process in chunks
    let mut buffer = Vec::with_capacity(BUFFER_CAPACITY);
    let mut combined_tasks = Vec::new();

    // Set up atomic counter for total records
    let total_records = Arc::new(AtomicUsize::new(0));
            
    while let Some(line) = lines.next_line().await? {

        buffer.push(line);
        
        if buffer.len() >= BUFFER_CAPACITY {
            let total_records_clone = Arc::clone(&total_records);
            let chunk = std::mem::take(&mut buffer);
            buffer = Vec::with_capacity(BUFFER_CAPACITY);

            // Process chunk and save to database in a single task
            let combined_handle = tokio::spawn(async move {
                // Process the chunk for stats in a blocking task
                let processed_chunk = task::spawn_blocking(move || {     
                     // Return the chunk for database processing
                     chunk
                    }).await.unwrap_or_else(|e| {
                        error!("Error processing chunk for stats: {}", e);
                        Vec::new()
                    });
                    
                    // Now process and save the records to the database
                    let records: Vec<FlightData> = processed_chunk
                        .iter()
                        .filter_map(|line| {
                            let fields: Vec<&str> = line.split(',').collect();
                            FlightData::from_vec(&fields).ok()
                        })
                        .collect();
                    
                    if !records.is_empty() {
                        if let Err(e) = save_to_mongodb(&records).await {
                            error!("Error saving to MongoDB: {}", e);
                        }
                    }
                });
                
                combined_tasks.push(combined_handle);
            }
        }     
    
        // Process remaining lines
        if !buffer.is_empty() {
            let total_records_clone = Arc::clone(&total_records);
            let remaining_buffer = buffer;

            // Process remaining chunk and save to database
            let combined_handle = tokio::spawn(async move {
                // Process the chunk for stats in a blocking task
                let processed_chunk = task::spawn_blocking(move || {
                    // Process each line in parallel to collect stats
                    remaining_buffer.par_iter().for_each(|line| {
                        let fields: Vec<&str> = line.split(',').collect();
                        
                    });
                                    
                    // Return the chunk for database processing
                    remaining_buffer
            }).await.unwrap_or_else(|e| {
                error!("Error processing chunk for stats: {}", e);
                Vec::new()
            });
            
            // Now process and save the records to the database
            let records: Vec<FlightData> = processed_chunk
                .iter()
                .filter_map(|line| {
                    let fields: Vec<&str> = line.split(',').collect();
                    FlightData::from_vec(&fields).ok()
                })
                .collect();
            
            if !records.is_empty() {
                if let Err(e) = save_to_mongodb(&records).await {
                    error!("Error saving to MongoDB: {}", e);
                }
            }
        });
        
        combined_tasks.push(combined_handle);
    }
    
    // Wait for all combined tasks to complete
    for task in combined_tasks {
        if let Err(e) = task.await {
            error!("Error in combined task: {}", e);
        }
    }

    Ok(())
}

async fn save_to_mongodb(
    records: &Vec<FlightData>,
) -> Result<(), AppError> {
    // Connect to MongoDB
    let uri = env::var("MONGODB_SRV")
        .expect("MONGODB_SRV environment variable not set.");

    let mongo_client = MongoClient::with_uri_str(uri).await?;

    let collection: Collection<FlightData> = mongo_client
        .database("flights_data")
        .collection("flights");
    
    // let insert_many_result = collection.insert_many(records).await?;
    
    // Process in batches
    for chunk in records.chunks(BATCH_SIZE) {
        println!("Saving batch of {} documents to MongoDB", chunk.len());
        
        // Convert chunk to insertable documents
        collection.insert_many(chunk).await?;
    }

    println!("Successfully saved {} airport records to MongoDB", records.len());
    Ok(())
}