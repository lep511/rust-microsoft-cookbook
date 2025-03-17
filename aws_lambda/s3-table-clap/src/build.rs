use serde::{Deserialize, Serialize};
use std::fs;
use aws_sdk_s3tables::Client;
use aws_sdk_s3tables::types::{
    OpenTableFormat, TableMetadata, IcebergMetadata, 
    SchemaField, IcebergSchema,
};

#[derive(Debug, Serialize, Deserialize)]
struct TableTemplate {
    table_name: String,
    namespace: String,
    fields: Vec<FieldTemplate>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FieldTemplate {
    name: String,
    #[serde(rename = "type")]
    field_type: String,
    required: Option<bool>,
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
            return Err(Box::new(e));
        }
    };
    
    let table_template: TableTemplate = match serde_yaml::from_str(&template_content) {
        Ok(template) => template,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    
    // Convert template fields to SchemaField objects
    let mut schema_fields = Vec::new();
    for field in table_template.fields {
        let mut field_builder = SchemaField::builder()
            .name(&field.name)
            .r#type(&field.field_type);
            
        if let Some(required) = field.required {
            field_builder = field_builder.required(required);
        }
        
        let schema_field = field_builder.build()?;
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

    println!("Table metadata: {:?}", table_metadata);
    
    // Create the table
    match client.create_table()
        .table_bucket_arn(table_bucket_arn)
        .namespace(&table_template.namespace)
        .name(&table_template.table_name)
        .format(OpenTableFormat::Iceberg)
        .metadata(table_metadata)
        .send().await {
            Ok(_) => println!("Table created successfully"),
            Err(e) => {
                return Err(Box::new(e));
            }
        }

    // Get table details for printing
    let get_table = client.get_table()
        .table_bucket_arn(table_bucket_arn)
        .namespace(&table_template.namespace)
        .name(&table_template.table_name)
        .send().await?;

    println!("Table created at: {}", get_table.created_at());
    println!("Table created by: {}", get_table.created_by());
    println!("Table format: {}", get_table.format());

    Ok(())
}