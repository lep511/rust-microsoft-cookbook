use lambda_runtime::Error;
use aws_sdk_s3::Client;
use aws_config::{load_defaults, BehaviorVersion, Region};
use aws_sdk_dsql::auth_token::{AuthTokenGenerator, Config};
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use uuid::Uuid;
use crate::libs::FlightData;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::collections::HashMap;
use tracing::{error, warn, info};
use tokio::task;
use rayon::prelude::*;

pub const BUFFER_CAPACITY: usize = 100_000; // Process ~100K lines at a time
pub const BATCH_SIZE: usize = 1500;
pub const MAX_RECORDS: usize = 20000;

pub async fn process_large_files(
    client: &Client,
    cluster_endpoint: &str,
    region: &str,
    bucket_name: &str, 
    object_key: &str,
) -> Result<HashMap<String, i32>, Error> {
    // For larger files, stream and process in chunks

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
    
    info!("Connected to database");

    // Create flights table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS flight_data (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            year INTEGER,
            month INTEGER,
            day INTEGER,
            day_of_week INTEGER,
            scheduled_departure INTEGER,
            actual_departure INTEGER,
            scheduled_arrival INTEGER,
            actual_arrival INTEGER,
            airline_code VARCHAR(255),
            flight_number INTEGER,
            aircraft_registration VARCHAR(255),
            scheduled_flight_time INTEGER,
            actual_flight_time INTEGER,
            air_time INTEGER,
            departure_delay INTEGER,
            arrival_delay INTEGER,
            origin_airport VARCHAR(255),
            destination_airport VARCHAR(255),
            distance INTEGER,
            taxi_out INTEGER,
            taxi_in INTEGER,
            carrier_delay INTEGER,
            weather_delay INTEGER,
            security_delay INTEGER,
            nas_delay INTEGER,
            other_delay INTEGER 
        )"
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        error!("Error creating table: {}", e);
        Error::from(e)
    })?;

    // Set up atomic counter for total records
    let total_records = Arc::new(AtomicUsize::new(0));
    
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
    let mut combined_tasks = Vec::new();
    
    while let Some(line) = lines.next_line().await? {
        buffer.push(line);
        
        if buffer.len() >= BUFFER_CAPACITY {
            let total_records_clone = Arc::clone(&total_records);
            let chunk = std::mem::take(&mut buffer);
            buffer = Vec::with_capacity(BUFFER_CAPACITY);
            let pool_clone = pool.clone();
            
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
                    if let Err(e) = save_to_dsql(&records, &pool_clone).await {
                        error!("Error saving to DSQL: {}", e);
                    }
                }
            });
            
            combined_tasks.push(combined_handle);
        }
    }
    
    // Process remaining lines
    if !buffer.is_empty() {
        let total_records_clone = Arc::clone(&total_records);
        let pool_clone = pool.clone();
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
                if let Err(e) = save_to_dsql(&records, &pool_clone).await {
                    error!("Error saving to DSQL: {}", e);
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

    let mut airport_counts: HashMap<String, i32> = HashMap::new();
    airport_counts.insert("PHX".to_string(), 25);
    
    Ok(airport_counts)
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
    records: &Vec<FlightData>,
    pool: &PgPool,
) -> Result<(), Error> {
    let mut record_count = 0;

    // Insert data for each airport code
    for record in records {
        let id = Uuid::new_v4();
                     
        sqlx::query("INSERT INTO flight_data (id, year, month, day, day_of_week, scheduled_departure, actual_departure, scheduled_arrival, actual_arrival, airline_code, flight_number, aircraft_registration, scheduled_flight_time, actual_flight_time, air_time, departure_delay, arrival_delay, origin_airport, destination_airport, distance, taxi_out, taxi_in, carrier_delay, weather_delay, security_delay, nas_delay, other_delay) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27)")
            .bind(id)
            .bind(record.year)
            .bind(record.month)
            .bind(record.day)
            .bind(record.day_of_week)
            .bind(record.scheduled_departure)
            .bind(record.actual_departure)
            .bind(record.scheduled_arrival)
            .bind(record.actual_arrival)
            .bind(record.airline_code.clone())
            .bind(record.flight_number)
            .bind(record.aircraft_registration.clone())
            .bind(record.scheduled_flight_time)
            .bind(record.actual_flight_time)
            .bind(record.air_time)
            .bind(record.departure_delay)
            .bind(record.arrival_delay)
            .bind(record.origin_airport.clone())
            .bind(record.destination_airport.clone())
            .bind(record.distance)
            .bind(record.taxi_out)
            .bind(record.taxi_in)
            .bind(record.carrier_delay)
            .bind(record.weather_delay)
            .bind(record.security_delay)
            .bind(record.nas_delay)
            .bind(record.other_delay)
            .execute(pool)
            .await
            .map_err(|e| {
                error!("Error inserting data: {}", e);
                Error::from(e)
            })?;

        record_count += 1;
    }

    info!("Successfully saved {} records to DSQL", record_count);
        
    Ok(())
}