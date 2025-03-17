use aws_sdk_s3tables::{Client, Error};
use aws_sdk_s3tables::operation::get_namespace::GetNamespaceOutput;
use log::{error, info};

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
                        error!("Error getting namespace: {}", e);
                        return Err(e.into());
                    }
                };

    Ok(response)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ CHECK TABLE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn check_table(
    client: &Client, 
    table_bucket_arn: &str,
    namespace: &str,
    table_name: &str,
) -> Result<(), Error> {
    let table = client.get_table()
                .table_bucket_arn(table_bucket_arn)
                .namespace(namespace)
                .name(table_name)
                .send().await?;

    println!("Table created at: {}", table.created_at());
    println!("Table modified at {}", table.modified_at());
    println!("Table format: {}", table.format());

    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ LIST NAMESPACE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn list_namespaces(
    client: &Client, 
    table_bucket_arn: &str
) -> Result<(), Error> {
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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ LIST TABLES ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn list_tables(
    client: &Client, 
    table_bucket_arn: &str,
    namespace: &str,
) -> Result<(), Error> {
    let tables = client.list_tables()
                .table_bucket_arn(table_bucket_arn)
                .namespace(namespace)
                .send().await?;

    for table in tables.tables() {
        println!("Table: {:?}", table.name);
        println!("Created at: {:?}", table.created_at);
        println!("Table modified at {}", table.modified_at());
        println!("--------------------------");
    }

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