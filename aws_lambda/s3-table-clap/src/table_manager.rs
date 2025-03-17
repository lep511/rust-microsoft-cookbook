use serde::{Deserialize, Serialize};
use std::fs;
use aws_sdk_s3tables::{Client, Error};
use aws_sdk_s3tables::types::{
    OpenTableFormat, TableMetadata, IcebergMetadata, 
    SchemaField, IcebergSchema,
};
use crate::utils::{create_namespace, get_namespace};
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

pub async fn create_table_from_yaml(
    client: &Client, 
    table_bucket_arn: &str,
    template_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // Load the YAML template
    let template_content = match fs::read_to_string(template_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Error reading template file");
            return Err(Box::new(e));
        }
    };
            
    let table_template: TableTemplate = match serde_yaml::from_str(&template_content) {
        Ok(template) => template,
        Err(e) => {
            error!("Error parsing YAML template");
            return Err(Box::new(e));
        }
    };

    let namespace = &table_template.namespace;
    let table_name = &table_template.table_name;

    // Check if namespace exists
    match get_namespace(&client, table_bucket_arn, namespace).await {
        Ok(_) => (),
        Err(_) => {
            match create_namespace(&client, table_bucket_arn, namespace).await {
                Ok(_) => info!("Namespace created successfully"),
                Err(e) => {
                    error!("Error creating namespace: {}", e);
                    return Err(Box::new(e));
                }
            }
        }
    }
    
    // Convert template fields to SchemaField objects
    let mut schema_fields = Vec::new();

    for field in table_template.fields {
        let schema_field = match SchemaField::builder()
            .name(&field.name)
            .r#type(&field.field_type)
            .required(field.required.unwrap_or(false))
            .build() {
                Ok(field) => field,
                Err(e) => {
                    return Err(Box::new(e));
                }
            };

        schema_fields.push(schema_field);
    }
    
    // Create Iceberg schema
    let iceberg_schema = IcebergSchema::builder()
        .set_fields(Some(schema_fields))
        .build()?;

    let iceberg_metadata = IcebergMetadata::builder()
        .schema(iceberg_schema)
        .build();

    let table_metadata = TableMetadata::Iceberg(iceberg_metadata);

    let _response = match create_s3_table(
        client,
        table_bucket_arn,
        table_name,
        namespace,
        table_metadata,
    ).await {
        Ok(response) => response,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
  
    Ok(())
}

pub async fn create_s3_table(
    client: &Client, 
    table_bucket_arn: &str, 
    table_name: &str, 
    namespace: &str,
    table_metadata: TableMetadata,
) -> Result<(), Error> {
    // Create the table
    client.create_table()
        .table_bucket_arn(table_bucket_arn)
        .namespace(namespace)
        .name(table_name)
        .format(OpenTableFormat::Iceberg)
        .metadata(table_metadata)
        .send().await?;

    // Get table details for printing
    let get_table = client.get_table()
        .table_bucket_arn(table_bucket_arn)
        .namespace(namespace)
        .name(table_name)
        .send().await?;

    info!("Table created at: {}", get_table.created_at());
    info!("Table created by: {}", get_table.created_by());
    info!("Table format: {}", get_table.format());

    Ok(())
}