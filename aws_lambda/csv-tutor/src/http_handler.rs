use lambda_http::{Body, Error, Request, RequestExt, Response};
use aws_sdk_s3::Client;
use aws_config::BehaviorVersion;
use csv::{Reader, Writer, Error as CsvError};
use serde::{Serialize, Deserialize};
use chrono::Local;
use std::collections::HashMap;
use std::io::Cursor;
use std::{env, io::Write};
use lambda_http::tracing::{info, error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    city: String,
    state: Option<String>,
    population: Option<u32>,
    latitude: Option<f32>,
    longitude: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalRecords {
    total: u32,
    records: HashMap<String, Record>,
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not set");
    let key = "uspope.csv";
    info!("Reading {} from {}", key, bucket_name);

    // Load AWS configuration from environment variables
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    // Download the object from S3
    let get_object_output = client.get_object().bucket(bucket_name).key(key).send().await?;
    let body = get_object_output.body.collect().await?;

    let data_bytes = body.into_bytes();
    let data = data_bytes.as_ref();

    let total_records: TotalRecords = match read_csv_data(data) {
        Ok(records) => records,
        Err(error) => {
            error!("Error reading CSV data: {}", error);
            let resp = Response::builder()
                .status(500)
                .header("content-type", "text/html")
                .body(format!("Error reading CSV data: {}", error).into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    info!("Total records: {}", total_records.total);

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

fn read_csv_data(data: &[u8]) -> Result<TotalRecords, CsvError> {
    let mut rdr: Reader<Cursor<&[u8]>> = csv::Reader::from_reader(Cursor::new(data));
    let mut total = 0;
    let mut records = HashMap::new(); 

    // Get the header from the original CSV and write it to the failed records file.
    let headers = rdr.headers()?.clone();
        
    // Iterate over the raw CSV records.
    for result in rdr.records() {
        let record_raw = match result {
            Ok(record) => record,
            Err(error) => {
                error!("Error reading CSV record: {}", error);
                continue;
            }
        };
            
        // Attempt to deserialize using the headers.
        let record_result: Result<Record, _> = record_raw.deserialize(Some(&headers));
        match record_result {
            Ok(record_struct) => {
                records.insert(record_struct.city.clone(), record_struct);
                total += 1;
            }
            Err(error) => {
                error!(
                    "Error deserializing record (line {}): {}.",
                    total, 
                    error,
                );
            }
        }
    }
        
    Ok(TotalRecords {
        total,
        records,
    })
}

