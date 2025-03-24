use aws_sdk_s3tables::{Client, Error};
use aws_sdk_s3tables::operation::get_namespace::GetNamespaceOutput;
use aws_sdk_s3tables::operation::get_table::GetTableOutput;
use aws_sdk_s3tables::operation::get_table_bucket::GetTableBucketOutput;
use aws_sdk_s3tables::operation::list_namespaces::ListNamespacesOutput;
use aws_sdk_s3tables::operation::list_tables::ListTablesOutput;
use aws_sdk_s3tables::operation::list_table_buckets::ListTableBucketsOutput;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::fs::File as TokioFile;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc, TimeZone};
use std::path::Path;
use std::io::Write;
use log::{error, info};

pub struct ProcessFileResult {
    pub fields: Vec<String>,
    pub errors: Vec<String>,
    pub n_columns: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableTemplate {
    pub table_name: String,
    pub namespace: String,
    pub fields: Vec<FieldTemplate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldTemplate {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryData {
    pub tablename: String,
    pub namespace: String,
    pub query: String,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ READ YAML FILE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn read_yaml_file(
    yaml_file_path: impl AsRef<Path>,
) -> Result<TableTemplate, Box<dyn std::error::Error + Send + Sync>> {
    // Read the file asynchronously using tokio's file system utilities
    let template_content = tokio::fs::read_to_string(yaml_file_path).await?;
    
    // Deserialize the YAML content
    // Note: serde_yaml::from_str is CPU-bound and doesn't have an async version
    // We can use tokio::task::spawn_blocking for CPU-intensive operations
    let table_template = tokio::task::spawn_blocking(move || {
        serde_yaml::from_str::<TableTemplate>(&template_content)
    }).await??; // Double ? to handle both JoinError and serde_yaml::Error

    Ok(table_template)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ PAUSE KEYPRESS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn pause_for_keypress() -> io::Result<()> {
    // Print the prompt
    print!("Press any key to continue...");
    std::io::stdout().flush()?;
    
    // Read a single byte from stdin
    let mut stdin = io::stdin();
    let mut buffer = [0u8; 1];
    stdin.read_exact(&mut buffer).await?;
    
    println!(); // Print a newline after keypress
    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ GET USER CONFIRMATION ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn get_user_confirmation(prompt: &str) -> io::Result<String> {
    // Get confirmation from user asynchronously
    let mut stdout = io::stdout();
    let prompt_fmt = format!("{}: ", prompt);
    stdout.write_all(prompt_fmt.as_bytes()).await?;
    stdout.flush().await?;

    // Set up a buffered reader for stdin
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    
    // Read line asynchronously
    let mut input = String::new();
    reader.read_line(&mut input).await?;
    
    // Trim whitespace and return
    Ok(input.trim().to_string())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ SAVE LOG FILE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn save_log_file(
    content: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let timestamp_millis = now.timestamp_millis();
    let log_file_name = format!("log_{}.txt", timestamp_millis);
    
    // Create a file for writing
    let mut file = TokioFile::create(&log_file_name).await?;

    // Write the content to the file
    file.write_all(content.as_bytes()).await?;
    
    // Ensure all data is written to disk
    file.flush().await?;

    Ok(log_file_name)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ CREATE NAMESPACE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn create_namespace(
    client: &Client, 
    table_bucket_arn: &str,
    namespace: &str,
) -> Result<(), Error> {
    let _namespace = match client.create_namespace()
                .table_bucket_arn(table_bucket_arn)
                .namespace(namespace)
                .send().await {
                    Ok(namespace) => namespace,
                    Err(e) => {
                        error!("Error creating namespace: {}", e);
                        return Err(e.into());
                    }
                };

    info!("Namespace created: {}", namespace);
    
    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ GET TABLE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn get_table(
    client: &Client, 
    table_bucket_arn: &str,
    namespace: &str,
    table_name: &str,
) -> Result<GetTableOutput, Error> {
    let table = client.get_table()
                .table_bucket_arn(table_bucket_arn)
                .namespace(namespace)
                .name(table_name)
                .send().await?;

    Ok(table)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ GET NAMESPACE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn get_namespace(
    client: &Client,
    table_bucket_arn: &str,
    namespace: &str,
) -> Result<GetNamespaceOutput, Error> {
    let response = match client.get_namespace()
                .table_bucket_arn(table_bucket_arn)
                .namespace(namespace)
                .send().await {
                    Ok(namespace) => namespace,
                    Err(e) => {
                        return Err(e.into());
                    }
                };

    Ok(response)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ GET TABLE BUCKET ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn get_table_bucket(
    client: &Client,
    table_bucket_arn: &str
) -> Result<GetTableBucketOutput, Error> {
    let response = client.get_table_bucket()
                .table_bucket_arn(table_bucket_arn)
                .send().await?;

    Ok(response)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ LIST NAMESPACE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn list_namespaces(
    client: &Client, 
    table_bucket_arn: &str
) -> Result<ListNamespacesOutput, Error> {
    let namespaces = client.list_namespaces()
                .table_bucket_arn(table_bucket_arn)
                .send().await?;

    Ok(namespaces)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ LIST TABLES ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn list_tables(
    client: &Client, 
    table_bucket_arn: &str,
    namespace: &str,
) -> Result<ListTablesOutput, Error> {
    let tables = client.list_tables()
                .table_bucket_arn(table_bucket_arn)
                .namespace(namespace)
                .send().await?;

    Ok(tables)
}

pub async fn list_table_buckets(
    client: &Client,
) -> Result<ListTableBucketsOutput, Error> {
    let table_buckets = client.list_table_buckets()
                .send().await?;

    Ok(table_buckets)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ DELETE TABLE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn delete_table(
    client: &Client, 
    table_bucket_arn: &str,
    namespace: &str,
    table_name: &str,
) -> Result<(), Error> {
    let _response = client.delete_table()
                .table_bucket_arn(table_bucket_arn)
                .namespace(namespace)
                .name(table_name)
                .send().await?;

    info!("Table deleted: {}", table_name);

    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ DELETE NAMESPACE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn delete_namespace(
    client: &Client, 
    table_bucket_arn: &str,
    namespace: &str,
) -> Result<(), Error> {
    let _response = client.delete_namespace()
                .table_bucket_arn(table_bucket_arn)
                .namespace(namespace)
                .send().await?;

    info!("Namespace deleted: {}", namespace);

    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ DELETE TABLE BUCKET S3 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn delete_table_bucket(
    client: &Client, 
    table_bucket_arn: &str
) -> Result<(), Error> {
    let _response = client.delete_table_bucket()
                .table_bucket_arn(table_bucket_arn)
                .send().await?;

    info!("Table bucket deleted: {}", table_bucket_arn);

    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ READ FILE ASYNC ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Asynchronously reads a text file from the specified path.
///
/// # Arguments
///
/// * `path` - A path-like value that references the file to read.
///            Can be any type that implements `AsRef<Path>`, such as `&str`, `String`, `PathBuf`, etc.
///
/// # Returns
///
/// * `io::Result<String>` - A Result containing either the file contents as a String
///   or an IO error if the file couldn't be opened or read
///
/// # Errors
///
/// This function will return an error if:
/// * The file at the specified path does not exist
/// * The process lacks permissions to read the file
/// * The file content is not valid UTF-8
/// * Any other I/O error occurs during reading
///
/// # Example
///
/// ```
/// let contents = read_file("example.txt").await?;
/// println!("File content: {}", contents);
/// ```
/// 
pub async fn read_file(path: impl AsRef<Path>) -> io::Result<String> {
    // Open the file asynchronously, returning any IO errors
    let mut file = TokioFile::open(path).await?;
    
    // Create an empty String to hold the file contents
    let mut contents = String::new();
    
    // Read the entire file contents into the string asynchronously
    // The read_to_string method will return an error if the file doesn't contain valid UTF-8
    file.read_to_string(&mut contents).await?;
    
    Ok(contents)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ PARSE DATE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn parse_date(
    date_string: &str
) -> Result<String, Box<dyn std::error::Error>> {
    // List of common datetime formats with time zones
    let formats = [
        "%Y-%m-%d %H:%M:%S",           // 2023-01-15 14:30:15
        "%Y-%m-%d %H:%M:%S.%f",        // 2023-01-15 14:30:15.123
        "%Y/%m/%d %H:%M:%S",           // 2023/01/15 14:30:15
        "%Y-%m-%dT%H:%M:%S",           // 2023-01-15T14:30:15
        "%Y-%m-%dT%H:%M:%S%z",         // 2023-01-15T14:30:15+00:00
        "%Y-%m-%dT%H:%M:%S.%f%z",      // 2023-01-15T14:30:15.123+00:00
        "%a, %d %b %Y %H:%M:%S %z",    // Tue, 15 Jan 2023 14:30:15 +0000
        "%A, %d-%b-%y %H:%M:%S %z",    // Friday, 21-Mar-25 14:47:21 UTC
        "%A, %d-%b-%y %H:%M:%S %Z",    // Friday, 21-Mar-25 14:47:21 UTC
        "%m/%d/%Y @ %I:%M%p",          // UTC: 03/21/2025 @ 2:47pm
        "%m/%d/%Y @ %I:%M:%S %p",      // UTC with seconds (optional support)
    ];

    // Try parsing datetime with timezone
    for format in formats {
        if let Ok(dt) = DateTime::parse_from_str(date_string, format) {
            let dt_utc = dt.with_timezone(&Utc);
            return Ok(dt_utc.format("%Y-%m-%d %H:%M:%S%.3f").to_string());
        }
    }

    // Try parsing naive datetime (assuming UTC)
    let naive_formats = [
        "%Y-%m-%dT%H:%M:%S%z",          // ISO 8601, RFC 3339: 2025-03-21T14:47:21+00:00
        "%Y-%m-%dT%H:%M:%S.%f%z",       // With microseconds
        "%a, %d %b %Y %H:%M:%S %z",     // RFC 822/2822: Fri, 21 Mar 2025 14:47:21 +0000
        "%A, %d-%b-%y %H:%M:%S %Z",     // RFC 2822 variant: Friday, 21-Mar-25 14:47:21 UTC
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M:%S.%f",
        "%Y/%m/%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M:%SZ",
        "%a, %d %b %Y %H:%M:%S %z",
        "%m/%d/%Y @ %I:%M%p",
        "%m/%d/%Y @ %I:%M:%S %p",
    ];

    for format in naive_formats {
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_string, format) {
            let dt = Utc.from_utc_datetime(&naive_dt);
            return Ok(dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string());
        }
    }

    // Try parsing as a Unix timestamp (seconds or milliseconds)
    if let Ok(timestamp) = date_string.parse::<i64>() {
        let (seconds, nanos) = if timestamp.abs() > 9_999_999_999 {
            // Assume milliseconds
            let seconds = timestamp / 1000;
            let millis_remainder = (timestamp % 1000).abs();
            (seconds, (millis_remainder * 1_000_000) as u32)
        } else {
            // Assume seconds
            (timestamp, 0)
        };

        if let chrono::LocalResult::Single(dt) = Utc.timestamp_opt(seconds, nanos) {
            return Ok(dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string());
        }
    }

    // If parsing fails, return an error
    let message = format!("{}", date_string);
    Err(message.into())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ FORMAT FIELD ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub fn format_field(field: &str, field_type: &str) -> Result<String, Box<dyn std::error::Error>> {
    let field = field.trim();

    if field.is_empty() {
        return Ok("NULL".to_string());
    } else if field.to_lowercase() == "null" {
        return Ok("NULL".to_string());
    } else if field.to_lowercase() == "none" {
        return Ok("NULL".to_string());
    } else if field.to_lowercase() == "nil" {
        return Ok("NULL".to_string());
    } else if field.to_lowercase() == "na" {
        return Ok("NULL".to_string());
    } else if field.to_lowercase() == "n/a" {
        return Ok("NULL".to_string());
    } else if field.to_lowercase() == "nan" {
        return Ok("NULL".to_string());
    }

    match field_type {
        "string" => {
            let field_fmt = field.replace("'", "''"); // Escape single quotes for SQL
            let quoted_field = format!("'{}'", field_fmt); // Wrap in quotes for SQL
            Ok(quoted_field)
        },
        "int" => {
            if field.parse::<i32>().is_ok() {
                Ok(field.to_string())
            } else {
                return Err("Invalid integer value".into());
            }
        },
        "float" => {
            if field.parse::<f32>().is_ok() {
                Ok(field.to_string())
            } else {
                return Err("Invalid float value".into());
            }
        },
        "double" => {
            if field.parse::<f64>().is_ok() {
                Ok(field.to_string())
            } else {
                return Err("Invalid double value".into());
            }
        },
        "decimal" => {
            if field.parse::<f64>().is_ok() {
                Ok(field.to_string())
            } else {
                return Err("Invalid decimal value".into());
            }
        },
        "boolean" => {
            if field.to_lowercase() == "true" || field.to_lowercase() == "false" {
                Ok(field.to_lowercase())
            } else if field.to_lowercase() == "yes" {
                Ok("true".to_string())
            } else if field.to_lowercase() == "no" {
                Ok("false".to_string())
            } else if field == "1" {
                Ok("true".to_string())
            } else if field == "0" {
                Ok("false".to_string())
            } else {
                return Err("Invalid boolean value".into());
            }
        },
        // Although Iceberg supports microsecond precision for the timestamp data type
        // Athena supports only millisecond precision for timestamps
        // https://docs.aws.amazon.com/athena/latest/ug/querying-iceberg.html
        "timestamp" => {
            let date_fmt = match parse_date(field) {
                Ok(date_fmt) => date_fmt,
                Err(e) => {
                    let message = format!("Invalid timestamp value: {}", e);
                    return Err(message.into());
                }
            };
            let result_date = format!("TIMESTAMP '{}'", date_fmt);
            return Ok(result_date);
        },
        _ => {
            let field_fmt = format!("'{}'", field);
            Ok(field_fmt)
        }
    }
}
