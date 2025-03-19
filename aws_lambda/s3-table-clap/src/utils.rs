use aws_sdk_s3tables::{Client, Error};
use aws_sdk_s3tables::operation::get_namespace::GetNamespaceOutput;
use aws_sdk_s3tables::operation::get_table::GetTableOutput;
use aws_sdk_s3tables::operation::get_table_bucket::GetTableBucketOutput;
use aws_sdk_s3tables::operation::list_namespaces::ListNamespacesOutput;
use aws_sdk_s3tables::operation::list_tables::ListTablesOutput;
use serde::{Deserialize, Serialize};
use csv::{ReaderBuilder, Reader};
use std::fs::{self, File};
use log::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct TableTemplate {
    pub table_name: String,
    pub namespace: String,
    pub fields: Vec<FieldTemplate>,
}

#[derive(Debug, Serialize, Deserialize)]
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