pub mod generate_data;

use generate_data::generate_random_data;
use aws_sdk_s3tables as s3tables;
use aws_sdk_s3tables::Client;
use aws_sdk_athena as athena;
use aws_sdk_athena::Client as AthenaClient;
use aws_sdk_s3tables::types::{
    OpenTableFormat, TableMetadata, IcebergMetadata, 
    SchemaField, IcebergSchema,
};

const MAX_BYTES: usize = 262144; // 256KB

pub async fn create_namespace(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {
    let name_space = "new_space";

    let namespace = client.create_namespace()
                .table_bucket_arn(table_bucket_arn)
                .namespace(name_space)
                .send().await?;

    println!("Namespace created: {}", name_space);
    
    Ok(())
}

pub async fn list_namespaces(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {

    let namespaces = client.list_namespaces()
                .table_bucket_arn(table_bucket_arn)
                .send().await?;

    for namespace in namespaces.namespaces() {
        println!("Namespace: {:?}", namespace.namespace);
        println!("Created at: {:?}", namespace.created_at);
        println!("--------------------------");
    }

    Ok(())
}

pub async fn create_table(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {

    let table_name = "my_new_table";
    let name_space = "new_space";

    let fields = Some(vec! [
        SchemaField::builder()
            .name("id")
            .r#type("int")
            .required(true)
            .build()?,
        SchemaField::builder()
            .name("type")
            .r#type("string")
            .build()?,
        SchemaField::builder()
            .name("value")
            .r#type("int")
            .build()?,
        SchemaField::builder()
            .name("date")
            .r#type("timestamp")
            .build()?,
    ]);

    let iceberg_schema = IcebergSchema::builder()
        .set_fields(fields)
        .build()?;

    let iceberg_metadata = IcebergMetadata::builder()
        .schema(iceberg_schema)
        .build();

    let table_metadata = TableMetadata::Iceberg(iceberg_metadata);
    
    let table = client.create_table()
                .table_bucket_arn(table_bucket_arn)
                .namespace(name_space)
                .name(table_name)
                .format(OpenTableFormat::Iceberg)
                .metadata(table_metadata)
                .send().await?;

    let get_table = client.get_table()
        .table_bucket_arn(table_bucket_arn)
        .namespace(name_space)
        .name(table_name)
        .send().await?;

    println!("Table created at: {}", get_table.created_at());
    println!("Table created by: {}", get_table.created_by());
    println!("Table format: {}", get_table.format());

    Ok(())
}

pub async fn list_tables(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {

    let name_space = "new_space";

    let tables = client.list_tables()
                .table_bucket_arn(table_bucket_arn)
                .namespace(name_space)
                .send().await?;

    for table in tables.tables() {
        println!("Table: {:?}", table.name);
        println!("Created at: {:?}", table.created_at);
        println!("Table modified at {}", table.modified_at());
        println!("--------------------------");
    }

    Ok(())
}

pub async fn check_table(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {

    let table_name = "my_new_table";
    let name_space = "new_space";

    let table = client.get_table()
                .table_bucket_arn(table_bucket_arn)
                .namespace(name_space)
                .name(table_name)
                .send().await?;

    println!("Table created at: {}", table.created_at());
    println!("Table modified at {}", table.modified_at());
    println!("Table created by: {}", table.created_by());
    println!("Table format: {}", table.format());

    Ok(())
}

pub async fn delete_table(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {

    let table_name = "my_new_table";
    let name_space = "new_space";

    let table = client.delete_table()
                .table_bucket_arn(table_bucket_arn)
                .namespace(name_space)
                .name(table_name)
                .send().await?;

    println!("Table deleted: {}", table_name);

    Ok(())
}

pub async fn delete_namespace(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {

    let name_space = "new_space";
    let namespace = client.delete_namespace()
                .table_bucket_arn(table_bucket_arn)
                .namespace(name_space)
                .send().await?;

    println!("Namespace deleted: {}", name_space);

    Ok(())
}

pub async fn delete_table_bucket(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {

    let table = client.delete_table_bucket()
                .table_bucket_arn(table_bucket_arn)
                .send().await?;

    println!("Table bucket deleted: {}", table_bucket_arn);

    Ok(())
}

pub async fn insert_with_athena(
    client: &AthenaClient, 
    table_bucket_arn: &str
) -> Result<(), athena::Error> {
    let config = aws_config::load_from_env().await;
    let client = athena::Client::new(&config);

    let mut table_bucket = "no_table";
    let name_space = "new_space";
    let table_name = "my_new_table";
    // let values = "(1111, 'ABC', 100, TIMESTAMP '2025-01-15 08:23:45.123456'),(2222, 'XYZ', 200, TIMESTAMP '2025-01-15 08:23:45.123456');";
    let values_gen = generate_random_data(3000);
    let values = values_gen.join(",");
    if values.len() > MAX_BYTES {
        panic!("Values too big");
    }

    if let Some(table) = table_bucket_arn.split('/').last() {
        table_bucket = table;
    }

    let query = format!("INSERT INTO \"s3tablescatalog/{}\".\"{}\".\"{}\" \
        VALUES \
        {};",
        table_bucket,
        name_space,
        table_name,
        values,
    );

    let result = client.start_query_execution()
        .query_string(query)
        // Add output location configuration
        .result_configuration(
            athena::types::ResultConfiguration::builder()
                .output_location("s3://data-lake-bucket-raw-49583/")
                .build()
        )
        .send()
        .await?;

    // Check status
    let execution_id = result.query_execution_id().unwrap();
    println!("Query execution ID: {}", execution_id);

    Ok(())
}

pub async fn insert_with_athena_handler(
    client: &AthenaClient, 
    table_bucket_arn: &str
) -> Result<(), s3tables::Error> {

    match insert_with_athena(client, table_bucket_arn).await {
        Ok(_) => println!("Query in progress"),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}


#[::tokio::main]
async fn main() -> Result<(), s3tables::Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let athena_client = AthenaClient::new(&config);
    let table_bucket_arn = "arn:aws:s3tables:us-east-1:491085411627:bucket/my-s3table-49585733";

    // create_namespace(&client, table_bucket_arn).await?;
    // list_namespaces(&client, table_bucket_arn).await?;

    // create_table(&client, table_bucket_arn).await?;
    // list_tables(&client, table_bucket_arn).await?;
    // check_table(&client, table_bucket_arn).await?;

    insert_with_athena_handler(&athena_client, table_bucket_arn).await?;

    // delete_table(&client, table_bucket_arn).await?;
    // delete_namespace(&client, table_bucket_arn).await?;
    
    // delete_table_bucket(&client, table_bucket_arn).await?;

    Ok(())
}
