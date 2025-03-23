use aws_sdk_s3tables::{Client, Error};
use aws_sdk_s3tables::types::{
    OpenTableFormat, TableMetadata, IcebergMetadata, 
    SchemaField, IcebergSchema,
};
use crate::utils::{
    TableTemplate, create_namespace,
    read_yaml_file, get_namespace,
};
use std::path::Path;
use crate::error::MainError;
use log::info;

pub async fn create_table_from_yaml(
    client: &Client, 
    table_bucket_arn: &str,
    template_path: &Path,
) -> Result<(), MainError> {
    // Load the YAML template           
    let table_template: TableTemplate = match read_yaml_file(template_path).await {
        Ok(template) => template,
        Err(e) => {
            let message = format!("Error reading template file: {}", e);
            return Err(MainError::GenericError { 
                message,
            });
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
                    let message = format!("Error creating namespace: {}", e);
                    return Err(MainError::GenericError { 
                        message,
                    });
                }
            }
        }
    }
    
    // Convert template fields to SchemaField objects
    let mut schema_fields = Vec::new();

    for field in table_template.fields {
        let schema_field = SchemaField::builder()
            .name(&field.name)
            .r#type(&field.field_type)
            .required(field.required.unwrap_or(false))
            .build()?;

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

    let _response = create_s3_table(
        client,
        table_bucket_arn,
        table_name,
        namespace,
        table_metadata,
    ).await?;
  
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
    info!("Table format: {}", get_table.format());

    Ok(())
}