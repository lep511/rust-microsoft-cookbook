use aws_sdk_s3::Client;
use futures::stream::StreamExt;
use mongodb::{
    bson::doc,
    options::{ClientOptions, InsertManyOptions},
    Collection, Client as MongoClient,
};
use crate::libs::{FlightData, MongoPool};
use crate::error::AppError;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::task;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::{error, info, instrument};
use rayon::prelude::*;
use std::io::{self, Write};
use std::env;
use std::fs::OpenOptions;
use std::path::Path;

// Constants
pub const BUFFER_CAPACITY: usize = 10_000; // Process ~10K lines at a time
pub const BATCH_SIZE: usize = 2000;

/// Process a CSV file from S3 and save valid records to MongoDB
///
/// # Arguments
///
/// * `s3_client` - AWS S3 client
/// * `mongo_pool` - MongoDB connection pool
/// * `bucket_name` - S3 bucket name containing the CSV file
/// * `object_key` - Key of the CSV file in the S3 bucket
/// * `error_file_path` - Optional path to save records that could not be processed
///
/// # Returns
///
/// * Result containing the number of successfully processed records
///
#[instrument(skip(s3_client, mongo_pool), fields(bucket = %bucket_name, key = %object_key))]
pub async fn process_s3_csv_file(
    s3_client: &Client,
    mongo_pool: Arc<MongoPool>,
    bucket_name: &str,
    object_key: &str,
    error_file_path: Option<&str>
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
    
    // Set up error file writer if path is provided
    let error_file = if let Some(path) = error_file_path {
        let file_path = Path::new(path);
        let file_exists = file_path.exists();
        
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path) {
                Ok(file) => {
                    // Create a buffered writer
                    let mut writer = io::BufWriter::new(file);
                    
                    // Write header if file is new
                    if !file_exists {
                        writeln!(writer, "line,error_reason")?;
                    }
                    
                    Some(Arc::new(tokio::sync::Mutex::new(writer)))
                },
                Err(e) => {
                    error!("Failed to open error file: {}", e);
                    None
                }
            }
    } else {
        None
    };

    // Process lines in chunks
    while let Some(line) = lines.next_line().await? {
        buffer.push(line);
        
        if buffer.len() >= BUFFER_CAPACITY {
            let chunk = std::mem::take(&mut buffer);
            buffer = Vec::with_capacity(BUFFER_CAPACITY);
            let total_records_clone = Arc::clone(&total_records);
            let mongo_pool_clone = Arc::clone(&mongo_pool);
            let error_file_clone = error_file.clone();

            // Process chunk and save to database in a single task
            let combined_handle = tokio::spawn(async move {
                let process_result = process_chunk(chunk.clone(), total_records_clone, error_file_clone).await;
                match process_result {
                    Ok(records) => {
                        if !records.is_empty() {
                            if let Err(e) = save_to_mongodb(mongo_pool_clone, &records).await {
                                error!("Error saving to MongoDB: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        error!("Error processing chunk: {}", e);
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
        let error_file_clone = error_file.clone();

        // Process remaining chunk and save to database
        let combined_handle = tokio::spawn(async move {
            let process_result = process_chunk(remaining_buffer.clone(), total_records_clone, error_file_clone).await;
            match process_result {
                Ok(records) => {
                    if !records.is_empty() {
                        if let Err(e) = save_to_mongodb(mongo_pool_clone, &records).await {
                            error!("Error saving to MongoDB: {}", e);
                        }
                    }
                },
                Err(e) => {
                    error!("Error processing remaining chunk: {}", e);
                }
            }
        });
        
        combined_tasks.push(combined_handle);
    }
    
    // Wait for all combined tasks to complete
    futures::future::join_all(combined_tasks).await;
    
    let final_count = total_records.load(Ordering::SeqCst);
    info!("Successfully processed {} records from {}/{}", final_count, bucket_name, object_key);
    
    // Close the error file if it exists
    if let Some(error_writer) = error_file {
        // Get the lock to ensure all writes are complete
        let mut writer = error_writer.lock().await;
        // Flush the writer to ensure all data is written
        if let Err(e) = writer.flush() {
            error!("Failed to flush error file at completion: {}", e);
        }
        // The file will be automatically closed when the writer is dropped
    }
    
    Ok(final_count)
}

/// Process a chunk of CSV lines, parsing them into FlightData objects
/// and recording any errors that occur during parsing
///
/// # Arguments
///
/// * `chunk` - Vector of CSV lines to process
/// * `total_records` - Atomic counter to track total successful records
/// * `error_file` - Optional file writer for recording error information
///
/// # Returns
///
/// * Result containing vector of successfully parsed FlightData objects
///
pub async fn process_chunk(
    chunk: Vec<String>,
    total_records: Arc<AtomicUsize>,
    error_file: Option<Arc<tokio::sync::Mutex<io::BufWriter<std::fs::File>>>>,
) -> Result<Vec<FlightData>, AppError> {
    // Process the chunk for stats in a blocking task
    let process_result = task::spawn_blocking(move || {
        // Using thread-safe collections for parallel processing
        let records = std::sync::Mutex::new(Vec::new());
        let errors = std::sync::Mutex::new(Vec::new());
        
        // Process each line in parallel and collect both successes and errors
        chunk.par_iter().for_each(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            match FlightData::from_vec(&fields) {
                Ok(data) => {
                    // Use thread-safe data structure to collect successful records
                    let mut records_guard = records.lock().unwrap();
                    records_guard.push(data);
                },
                Err(e) => {
                    let error_msg = format!("Parse error: {}", e);
                    // Use thread-safe data structure to collect errors
                    let mut errors_guard = errors.lock().unwrap();
                    errors_guard.push((line.clone(), error_msg));
                }
            }
        });
        
        struct ProcessResult {
            records: Vec<FlightData>,
            errors: Vec<(String, String)>, // (line, error_reason)
        }
        
        ProcessResult {
            records: records.into_inner().unwrap(),
            errors: errors.into_inner().unwrap(),
        }
    }).await.map_err(|e| AppError::generic(format!("Error in task: {}", e)))?;
    
    // Write errors to file if available
    if let Some(error_writer) = error_file {
        if !process_result.errors.is_empty() {
            let mut writer = error_writer.lock().await;
            for (line, reason) in &process_result.errors {
                // Escape line if it contains quotes or commas
                let escaped_line = if line.contains('"') || line.contains(',') {
                    format!("\"{}\"", line.replace('"', "\"\""))
                } else {
                    line.clone()
                };
                
                // Write error record to file
                if let Err(e) = writeln!(writer, "{},{}", escaped_line, reason.replace(',', ";")) {
                    error!("Failed to write to error file: {}", e);
                }
            }
            
            // Flush to ensure writing
            if let Err(e) = writer.flush() {
                error!("Failed to flush error file: {}", e);
            }
        }
    }
    
    // Update total records count
    total_records.fetch_add(process_result.records.len(), Ordering::SeqCst);
    
    info!(
        "Processed chunk: {} successful records, {} errors", 
        process_result.records.len(), 
        process_result.errors.len()
    );
    
    Ok(process_result.records)
}

#[instrument(skip(mongo_pool, records), fields(record_count = records.len()))]
pub async fn save_to_mongodb(
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
        
        match collection.insert_many(chunk).await {
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