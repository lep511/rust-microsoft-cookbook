use aws_sdk_s3tables::{Client, Error};
use aws_sdk_s3tables::operation::get_namespace::GetNamespaceOutput;
use aws_sdk_s3tables::operation::get_table::GetTableOutput;
use aws_sdk_s3tables::operation::get_table_bucket::GetTableBucketOutput;
use aws_sdk_s3tables::operation::list_namespaces::ListNamespacesOutput;
use aws_sdk_s3tables::operation::list_tables::ListTablesOutput;
use aws_sdk_s3tables::operation::list_table_buckets::ListTableBucketsOutput;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, TimeZone, Utc};
use csv::{ReaderBuilder, Reader};
use std::fs::{self, File};
use log::{error, info};

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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ READ CSV FILE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub fn read_csv_file(
    csv_file_path: &str,
    delimiter: u8,
    has_headers: bool,
) -> Result<Reader<File>, Box<dyn std::error::Error>> {
    let file = File::open(csv_file_path)?;

    let reader = ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(has_headers) 
        .from_reader(file);

    Ok(reader)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ READ YAML FILE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub fn read_yaml_file(
    yaml_file_path: &str,
) -> Result<TableTemplate, Box<dyn std::error::Error>> {
    let template_content = match fs::read_to_string(yaml_file_path) {
        Ok(content) => content,
        Err(e) => return Err(Box::new(e)),
    };
            
    let table_template: TableTemplate = match serde_yaml::from_str(&template_content) {
        Ok(template) => template,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(table_template)
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
            // ISO 8601 - 2025-03-21T13:23:16+00:00	
            if field.contains("-") && field.contains(":") && field.len() >= 16 {
                let dt_utc = match DateTime::parse_from_rfc3339(field) {
                    Ok(dt) => dt,
                    Err(e) => return Err(e.into()),
                };

                let field_fmt = format!(
                    "TIMESTAMP '{}'", 
                    dt_utc.format("%Y-%m-%d %H:%M:%S%.3f")
                );
                Ok(field_fmt)
            // UTC - 03/21/2025 @ 1:23pm
            } else if field.contains("/") && field.contains(":") && field.len() >= 16 {
                let dt_utc = match DateTime::parse_from_str(field, "%m/%d/%Y %l:%M%p") {
                    Ok(dt) => dt,
                    Err(e) => return Err(e.into()),
                };

                let field_fmt = format!(
                    "TIMESTAMP '{}'", 
                    dt_utc.format("%Y-%m-%d %H:%M:%S%.3f")
                );
                Ok(field_fmt)
            // epoch time 1742563396
            } else if field.parse::<i64>().is_ok() {
                let field_num = match field.parse::<i64>() {
                    Ok(num) => num,
                    Err(e) => return Err(e.into()),
                };
                
                // The issue is here - timestamp_opt() doesn't return Result but SingleResult
                let dt_utc = match Utc.timestamp_opt(field_num, 0) {
                    chrono::offset::LocalResult::Single(dt) => dt,
                    _ => return Err("Invalid timestamp value".into()), //Err(anyhow::anyhow!("Invalid timestamp")),
                };
                
                let field_fmt = format!(
                    "TIMESTAMP '{}'", 
                    dt_utc.format("%Y-%m-%d %H:%M:%S%.3f")
                );
                Ok(field_fmt)
            } else {
                return Err("Invalid timestamp value".into());
            }
        },
        _ => {
            let field_fmt = format!("'{}'", field);
            Ok(field_fmt)
        }
    }
}
