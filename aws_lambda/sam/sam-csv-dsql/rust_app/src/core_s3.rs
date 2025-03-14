use lambda_runtime::Error;
use aws_sdk_s3::Client;
use aws_config::{load_defaults, BehaviorVersion, Region};
use aws_sdk_dsql::auth_token::{AuthTokenGenerator, Config};
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use uuid::Uuid;
use crate::libs::FlightData;
use tokio::io::AsyncBufReadExt;
use std::collections::HashMap;
use tracing::{error, warn, info};

pub const BATCH_SIZE: usize = 1500;
pub const MAX_RECORDS: usize = 20000;

pub async fn process_beta_files(
    client: &Client,
    cluster_endpoint: &str,
    region: &str,
    bucket_name: &str, 
    object_key: &str,
) -> Result<HashMap<String, i32>, Error> {
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
    
    // Download the object from S3
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

    let mut total_records = 0;
    let mut records = Vec::new();

    let stream = get_object_output.body;
    let buf_reader = stream.into_async_read();
    let mut lines = buf_reader.lines();

    // Skip header
    let _ = lines.next_line().await?;

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        let line = line.split(",").collect::<Vec<&str>>();
        let record = match FlightData::from_vec(&line) {
            Ok(record) => record,
            Err(error) => {
                warn!("Error parsing record: {}", error);
                continue;
            }
        };

        records.push(record);

        if records.len() >= BATCH_SIZE {
            // Save record to DSQL database
            save_to_dsql(&records, &pool).await?;
            records.clear();
        }
      
        total_records += 1;
        
        if total_records >= MAX_RECORDS {
            break;
        }
    }

    info!("Total records process: {}", total_records);

    // Close the connection pool
    pool.close().await;

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
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Error creating table: {}", e);
        Error::from(e)
    })?;

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