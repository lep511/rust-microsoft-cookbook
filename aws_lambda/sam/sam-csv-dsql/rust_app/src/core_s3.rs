use lambda_runtime::Error;
use aws_sdk_s3::Client;
use aws_config::{load_defaults, BehaviorVersion, Region};
use aws_sdk_dsql::auth_token::{AuthTokenGenerator, Config};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use uuid::Uuid;
use crate::libs::FlightData;
use std::io::{Cursor, BufRead};
use std::sync::{Arc, Mutex};
use tokio::task;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use tracing::{error, info};
use rayon::prelude::*;

pub const BUFFER_CAPACITY: usize = 100_000; // Process ~100K lines at a time

/// Processes smaller flight data files by downloading the entire file and analyzing it in memory.
///
/// This function retrieves a flight data file from S3, downloads it completely, and processes
/// its contents in a parallel manner to count airport occurrences.
///
/// # Arguments
///
/// * `client` - The AWS S3 client used to retrieve the file
/// * `cluster_endpoint` - The endpoint for the AWS DSQL cluster
/// * `region` - The AWS region where the DSQL cluster is located
/// * `bucket_name` - The name of the S3 bucket containing the file
/// * `object_key` - The key (path) of the object in the S3 bucket
/// * `content_length` - The expected size of the file in bytes, used for buffer pre-allocation
///
/// # Returns
///
/// A `Result` containing either:
/// * `HashMap<String, u32>` - A mapping of airport codes to their occurrence counts
/// * `Error` - An error that occurred during retrieval or processing
///
/// # Processing Details
///
/// The function:
/// 1. Downloads the entire file from S3
/// 2. Processes the file in a separate blocking thread pool to avoid blocking the async runtime
/// 3. Parses each line into flight data records using parallel processing via Rayon
/// 4. Counts occurrences of each airport code (found in field 17)
/// 5. Uses thread-local maps to reduce contention during parallel processing
/// 6. Merges results into a single HashMap that's returned to the caller
///
/// # Errors
///
/// Returns an error if:
/// * The S3 object cannot be retrieved
/// * The downloaded data cannot be read
/// * The blocking task fails to complete
///
pub async fn process_small_files(
    client: &Client, 
    cluster_endpoint: &str,
    region: &str,
    bucket_name: &str, 
    object_key: &str,
    content_length: usize,
) -> Result<HashMap<String, u32>, Error> {
    // For smaller files, download the whole file and process in memory

    // Set up atomic counter for total records
    let total_records = Arc::new(AtomicUsize::new(0));
    
    // Create a thread-safe HashMap to store airport counts
    let airport_counts = Arc::new(Mutex::new(HashMap::<String, u32>::new()));
    
    let get_object_output = client
        .get_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await
        .map_err(|e| {
            error!("Error getting object: {}", e);
            e
        })?;
    
    let mut bytes = Vec::with_capacity(content_length);
    get_object_output.body.into_async_read().read_to_end(&mut bytes).await?;

    // Process file in a blocking task using rayon
    let total_records_clone = Arc::clone(&total_records);
    let airport_counts_clone = Arc::clone(&airport_counts);

    task::spawn_blocking(move || {
        // Split into lines and skip header
        let cursor = Cursor::new(&bytes);
        let mut lines: Vec<String> = BufRead::lines(cursor)
            .filter_map(Result::ok)
            .collect();
            
        if !lines.is_empty() {
            lines.remove(0); // Remove header
        }
        
        // Use a local thread-local HashMap for each thread to avoid contention
        let thread_local_maps: Vec<HashMap<String, u32>> = lines
            .par_iter()
            .map(|line| {
                let fields: Vec<&str> = line.split(',').collect();
                let mut local_map = HashMap::new();
                
                // Parse record
                let record = match FlightData::from_vec(&fields) {
                    Ok(record) => record,
                    Err(_) => return local_map,
                };
                
                // Increment counters
                if let Some(airport_code) = fields.get(17) {
                    if !airport_code.is_empty() {
                        let count = local_map.entry(airport_code.to_string()).or_insert(0);
                        *count += 1;
                    }
                }
                
                total_records_clone.fetch_add(1, Ordering::Relaxed);
                local_map
            })
            .collect();
        
        // Merge all thread-local maps into the shared HashMap
        let mut final_map = airport_counts_clone.lock().unwrap();
        for local_map in thread_local_maps {
            for (key, value) in local_map {
                *final_map.entry(key).or_insert(0) += value;
            }
        }
    }).await?;

    // Return a clone of the HashMap
    let result = Arc::try_unwrap(airport_counts)
        .unwrap_or_else(|arc| (*arc.lock().unwrap()).clone().into())
        .into_inner()
        .unwrap_or_default();

    // Save results to AWS DSQL database
    save_to_dsql(&result, cluster_endpoint, region).await?;
    
    Ok(result)
}

/// Processes large S3 files containing flight data, streaming and processing the content in chunks.
/// Then saves the processed airport counts to an AWS DSQL database.
///
/// This function retrieves an object from an S3 bucket and processes it line by line to count
/// airport occurrences. It implements several optimizations for handling large files:
/// - Streams data instead of loading the entire file into memory
/// - Processes data in configurable-sized chunks
/// - Uses parallel processing with thread-local maps to reduce lock contention
/// - Merges results efficiently from multiple threads
/// - Stores the results in an AWS DSQL Postgres database
///
/// # Arguments
///
/// * `client` - An AWS S3 client used to retrieve the object
/// * `cluster_endpoint` - The endpoint for the AWS DSQL cluster
/// * `region` - The AWS region where the DSQL cluster is located
/// * `bucket_name` - The name of the S3 bucket containing the target file
/// * `object_key` - The key (path) of the object within the bucket
///
/// # Returns
///
/// * `Result<HashMap<String, u32>, Error>` - A map of airport codes to their occurrence counts,
///   or an error if the file couldn't be processed or data couldn't be saved
///
/// # Errors
///
/// This function will return an error if:
/// - The S3 object cannot be retrieved
/// - There are issues reading lines from the file
/// - Any of the spawned tasks fail
/// - Connection to the DSQL database fails
/// - Database operations fail
pub async fn process_large_files(
    client: &Client, 
    cluster_endpoint: &str,
    region: &str,
    bucket_name: &str, 
    object_key: &str,
) -> Result<HashMap<String, u32>, Error> {
    // For larger files, stream and process in chunks

    // Set up atomic counter for total records
    let total_records = Arc::new(AtomicUsize::new(0));
    
    // Create a thread-safe HashMap to store airport counts
    let airport_counts = Arc::new(Mutex::new(HashMap::<String, u32>::new()));

    let get_object_output = client
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
    let mut tasks = Vec::new();
    
    while let Some(line) = lines.next_line().await? {
        buffer.push(line);
        
        if buffer.len() >= BUFFER_CAPACITY {
            let airport_counts_clone = Arc::clone(&airport_counts);
            let total_records_clone = Arc::clone(&total_records);
            let chunk = std::mem::take(&mut buffer);
            buffer = Vec::with_capacity(BUFFER_CAPACITY);
            
            // Process chunk in parallel
            let handle = task::spawn_blocking(move || {
                // Use a local thread-local HashMap for each thread to avoid contention
                let thread_local_maps: Vec<HashMap<String, u32>> = chunk
                    .par_iter()
                    .map(|line| {
                        let fields: Vec<&str> = line.split(',').collect();
                        let mut local_map = HashMap::new();
                        
                        // Parse record
                        let record = match FlightData::from_vec(&fields) {
                            Ok(record) => record,
                            Err(_) => return local_map,
                        };
                                                
                        // Increment counters
                        if let Some(airport_code) = fields.get(17) {
                            if !airport_code.is_empty() {
                                let count = local_map.entry(airport_code.to_string()).or_insert(0);
                                *count += 1;
                            }
                        }
                        
                        total_records_clone.fetch_add(1, Ordering::Relaxed);
                        local_map
                    })
                    .collect();
                
                // Merge all thread-local maps into the shared HashMap
                let mut final_map = airport_counts_clone.lock().unwrap();
                for local_map in thread_local_maps {
                    for (key, value) in local_map {
                        *final_map.entry(key).or_insert(0) += value;
                    }
                }
            });
            
            tasks.push(handle);
        }
    }
    
    // Process remaining lines
    if !buffer.is_empty() {
        let airport_counts_clone = Arc::clone(&airport_counts);
        let total_records_clone = Arc::clone(&total_records);
        
        let handle = task::spawn_blocking(move || {
            // Use a local thread-local HashMap for each thread to avoid contention
            let thread_local_maps: Vec<HashMap<String, u32>> = buffer
                .par_iter()
                .map(|line| {
                    let fields: Vec<&str> = line.split(',').collect();
                    let mut local_map = HashMap::new();
                    
                    // Parse record
                    let record = match FlightData::from_vec(&fields) {
                        Ok(record) => record,
                        Err(_) => return local_map,
                    };
                                        
                    // Increment counters
                    if let Some(airport_code) = fields.get(17) {
                        if !airport_code.is_empty() {
                            let count = local_map.entry(airport_code.to_string()).or_insert(0);
                            *count += 1;
                        }
                    }
                    
                    total_records_clone.fetch_add(1, Ordering::Relaxed);
                    local_map
                })
                .collect();
            
            // Merge all thread-local maps into the shared HashMap
            let mut final_map = airport_counts_clone.lock().unwrap();
            for local_map in thread_local_maps {
                for (key, value) in local_map {
                    *final_map.entry(key).or_insert(0) += value;
                }
            }
        });
        
        tasks.push(handle);
    }
    
    // Wait for all processing tasks to complete
    for task in tasks {
        task.await?;
    }

    // Get the final HashMap
    let result = Arc::try_unwrap(airport_counts)
        .unwrap_or_else(|arc| (*arc.lock().unwrap()).clone().into())
        .into_inner()
        .unwrap_or_default();
    
    // Save results to AWS DSQL database
    save_to_dsql(&result, cluster_endpoint, region).await?;
    
    Ok(result)
}

/// Saves the processed airport counts to an AWS DSQL database.
///
/// # Arguments
///
/// * `airport_counts` - A HashMap containing airport codes and their occurrence counts
/// * `cluster_endpoint` - The endpoint for the AWS DSQL cluster
/// * `region` - The AWS region where the DSQL cluster is located
///
/// # Returns
///
/// * `Result<(), Error>` - Ok if the data was successfully saved, or an error otherwise
async fn save_to_dsql(
    airport_counts: &HashMap<String, u32>,
    cluster_endpoint: &str,
    region: &str,
) -> Result<(), Error> {
    // Generate auth token
    let sdk_config = load_defaults(BehaviorVersion::latest()).await;
    let signer = AuthTokenGenerator::new(
        Config::builder()
            .hostname(cluster_endpoint)
            .region(Region::new(region.to_string()))
            .build()
            .unwrap(),
    );
    let password_token = signer.db_connect_admin_auth_token(&sdk_config).await
        .map_err(|e| {
            error!("Error generating auth token: {}", e);
            Error::from(e)
        })?;

    // Setup connections
    let connection_options = PgConnectOptions::new()
        .host(cluster_endpoint)
        .port(5432)
        .database("postgres")
        .username("admin")
        .password(password_token.as_str())
        .ssl_mode(sqlx::postgres::PgSslMode::VerifyFull);
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(connection_options.clone())
        .await
        .map_err(|e| {
            error!("Error connecting to database: {}", e);
            Error::from(e)
        })?;

    // Create flights table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS flights (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            airport_code VARCHAR(255),
            count INTEGER
        )"
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        error!("Error creating table: {}", e);
        Error::from(e)
    })?;

    // Insert data for each airport code
    for (airport_code, count) in airport_counts {
        let id = Uuid::new_v4();
        
        sqlx::query("INSERT INTO flights (id, airport_code, count) VALUES ($1, $2, $3)")
            .bind(id)
            .bind(airport_code)
            .bind(*count as i32)  // Convert u32 to i32 for PostgreSQL
            .execute(&pool)
            .await
            .map_err(|e| {
                error!("Error inserting data: {}", e);
                Error::from(e)
            })?;
    }

    info!("Successfully saved {} airport records to DSQL", airport_counts.len());
    
    // Close the connection pool
    pool.close().await;
    
    Ok(())
}