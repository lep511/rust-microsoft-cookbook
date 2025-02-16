#[allow(dead_code)]
pub mod generate_data;
pub mod compatible;
use log::{info, error};
use generate_data::generate_random_data;
use tokio::time::{sleep, Duration};
use aws_sdk_s3tables as s3tables;
use aws_sdk_s3tables::Client;
use aws_sdk_athena as athena;
use aws_sdk_athena::Client as AthenaClient;
use aws_sdk_athena::types::QueryExecutionState;
use aws_sdk_s3tables::types::{
    OpenTableFormat, TableMetadata, IcebergMetadata, 
    SchemaField, IcebergSchema,
};
use compatible::chat::ChatCompatible;
use compatible::libs::ChatResponse;
use env_logger::Env;

const MAX_BYTES: usize = 262144; // 256KB

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ CREATE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn create_namespace(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {
    let name_space = "flight";

    let namespace = client.create_namespace()
                .table_bucket_arn(table_bucket_arn)
                .namespace(name_space)
                .send().await?;

    info!("Namespace created: {}", name_space);
    
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

    let table_name = "flight_data";
    let name_space = "flight";

    let fields = Some(vec![
        SchemaField::builder()
            .name("year")
            .r#type("int")
            .required(true)
            .build()?,
        SchemaField::builder()
            .name("month")
            .r#type("int")
            .required(true)
            .build()?,
        SchemaField::builder()
            .name("day_of_month")
            .r#type("int")
            .required(true)
            .build()?,
        SchemaField::builder()
            .name("day_of_week")
            .r#type("int")
            .build()?,
        SchemaField::builder()
            .name("deptime")
            .r#type("float")
            .build()?,
        SchemaField::builder()
            .name("crs_deptime")
            .r#type("int")
            .build()?,
        SchemaField::builder()
            .name("arr_time")
            .r#type("float")
            .build()?,
        SchemaField::builder()
            .name("crs_arr_time")
            .r#type("int")
            .build()?,
        SchemaField::builder()
            .name("unique_carrier")
            .r#type("string")
            .build()?,
        SchemaField::builder()
            .name("flight_num")
            .r#type("int")
            .build()?,
        SchemaField::builder()
            .name("taxi_in")
            .r#type("float")
            .build()?,
        SchemaField::builder()
            .name("taxi_out")
            .r#type("float")
            .build()?,
        SchemaField::builder()
            .name("cancelled")
            .r#type("boolean")
            .build()?,
        SchemaField::builder()
            .name("cancellation_code")
            .r#type("string")
            .build()?,
        SchemaField::builder()
            .name("diverted")
            .r#type("boolean")
            .build()?,
        SchemaField::builder()
            .name("carrier_delay")
            .r#type("float")
            .build()?,
        SchemaField::builder()
            .name("weather_delay")
            .r#type("float")
            .build()?,
        SchemaField::builder()
            .name("nas_delay")
            .r#type("float")
            .build()?,
        SchemaField::builder()
            .name("security_delay")
            .r#type("float")
            .build()?,
        SchemaField::builder()
            .name("flight_date")
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

    let name_space = "flight";

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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ CHECK ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn check_table(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {

    let table_name = "flight_data";
    let name_space = "flight";

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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ INSERT ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn insert_with_athena(
    client: &AthenaClient, 
    table_bucket_arn: &str
) -> Result<(), athena::Error> {
    let config = aws_config::load_from_env().await;
    let client = athena::Client::new(&config);

    let mut table_bucket = "no_table";
    let name_space = "flight";
    let table_name = "flight_data";
    
    // ################## Generate Data #########################
    let values_gen = generate_random_data(1000);
    // ##########################################################
    
    let values = values_gen.join(",");
    if values.len() > MAX_BYTES {
        panic!("Query exceeds maximum allowed size 256 kb");
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

    // Set query execution ID
    let query_execution_id = result.query_execution_id().unwrap();
    info!("Query execution ID: {}", query_execution_id);

    // Polling for query execution completion
    loop {
        let response = client
            .get_query_execution()
            .query_execution_id(query_execution_id)
            .send()
            .await?;
        
        if let Some(query_execution) = response.query_execution {
            if let Some(status) = query_execution.status {
                if let Some(state) = status.state {
                    match state {
                        QueryExecutionState::Succeeded => {
                            if let Some(query) = query_execution.query {
                                info!("Query execution succeeded.");
                                // println!("The SQL query is: {}", query);
                            } else {
                                error!("Query not found in execution details.");
                            }
                            break;
                        }
                        QueryExecutionState::Failed | QueryExecutionState::Cancelled => {
                            if let Some(query) = query_execution.query {
                                error!("Query execution failed or was cancelled.");
                                info!("The SQL query is: {}", query);
                            } else {
                                error!("Query not found in execution details.");
                            }
                            break;
                        }
                        _ => {
                            info!("Query is still running. Waiting...");
                            sleep(Duration::from_secs(7)).await;
                        }
                    }
                }
            }
        } else {
            error!("Query execution not found.");
            break;
        }
    }

    Ok(())
}

pub async fn insert_with_athena_handler(
    client: &AthenaClient, 
    table_bucket_arn: &str
) -> Result<(), s3tables::Error> {

    match insert_with_athena(client, table_bucket_arn).await {
        Ok(_) => (),
        Err(e) => error!("Error: {}", e),
    }

    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ QUERY ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn query_handler(
    client: &AthenaClient,
    table_bucket_arn: &str,
    query_fparam: &str,
    query_lparam: &str,
) -> Result<(), athena::Error> {
    let config = aws_config::load_from_env().await;
    let client = athena::Client::new(&config);

    let name_space = "flight";
    let table_name = "flight_data";
    let mut table_bucket = "no_table";
    let mut query_success = false;

    if let Some(table) = table_bucket_arn.split('/').last() {
        table_bucket = table;
    }

    let query = format!("{} FROM \"s3tablescatalog/{}\".\"{}\".\"{}\" {};",
        query_fparam,
        table_bucket,
        name_space,
        table_name,
        query_lparam,
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

    // Set query execution ID
    let query_execution_id = result.query_execution_id().unwrap();
    info!("Query execution ID: {}", query_execution_id);

    // Polling for query execution completion
    loop {
        let response = client
            .get_query_execution()
            .query_execution_id(query_execution_id)
            .send()
            .await?;

        if let Some(query_execution) = response.query_execution {
            if let Some(status) = query_execution.status {
                if let Some(state) = status.state {
                    match state {
                        QueryExecutionState::Succeeded => {
                            if let Some(query) = query_execution.query {
                                info!("Query execution succeeded.");
                                info!("File csv: {}", query_execution.result_configuration.unwrap().output_location.unwrap());
                            } else {
                                error!("Query not found in execution details.");
                            }
                            query_success = true;
                            break;
                        }
                        QueryExecutionState::Failed | QueryExecutionState::Cancelled => {
                            if let Some(query) = query_execution.query {
                                error!("Query execution failed or was cancelled.");
                                info!("The SQL query is: {}", query);
                            } else {
                                error!("Query not found in execution details.");
                            }
                            break;
                        }
                        _ => {
                            info!("Query is still running. Waiting...");
                            sleep(Duration::from_secs(7)).await;
                        }
                    }
                }
            }
        } else {
            error!("Query execution not found.");
            break;
        }
    }

    if query_success {
        let results = client.get_query_results()
            .query_execution_id(query_execution_id)
            .send()
            .await?;

        let mut table_data: Vec<Vec<String>> = Vec::new();
        
        if let Some(result_set) = results.result_set() {
            for row in result_set.rows() {
                // Each row can have multiple cells (columns).
                let row_data: Vec<String> = row.data()
                    .iter()
                    .map(|datum| datum.var_char_value().unwrap_or("").to_string())
                    .collect();
                println!("{:?}", row_data);
                table_data.push(row_data);
            }
        }

        let base_url = "https://api.x.ai/v1/chat/completions";
        let model = "grok-2-latest";
        let llm = ChatCompatible::new(base_url, model);
    
        let prompt = format!(
            "According to the information in this table, which are the airlines with the most \
            cancelled flights? Do not provide information about the origin of the data. \
            Table: \n {:?}",
            table_data,
        );
       
        let response = llm
            .with_max_retries(0)
            .invoke(&prompt)
            .await;

        match response {
            Ok(response) => {
                match response.choices {
                    Some(candidates) => {
                        for candidate in candidates {
                            #[allow(irrefutable_let_patterns)]
                            if let Some(message) = candidate.message {
                                if let Some(content) = message.content {
                                    println!("{}", content);
                                }
                            }
                        }
                    }
                    None => println!("No response choices available"),
                };
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        } 
    }

    Ok(())
}

pub async fn query_with_athena(
    client: &AthenaClient,
    table_bucket_arn: &str,
) -> Result<(), s3tables::Error> {

    let query_fparam = "SELECT CASE \
            WHEN unique_carrier = 'UA' THEN 'United Airlines' \
            WHEN unique_carrier = 'WN' THEN 'Southwest Airlines' \
            WHEN unique_carrier = 'AS' THEN 'Alaska Airlines' \
            WHEN unique_carrier = 'AA' THEN 'American Airlines' \
            WHEN unique_carrier = 'DL' THEN 'Delta Air Lines' \
            ELSE unique_carrier  \
        END AS airline_name, \
        COUNT(*) AS total_cancelled";

    let query_lparam = "WHERE cancelled \
        GROUP BY 1
        ORDER BY total_cancelled DESC";

    match query_handler(client, table_bucket_arn, query_fparam, query_lparam).await {
        Ok(_) => (),
        Err(e) => error!("Error: {}", e),
    }

    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ DELETE ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn delete_table(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {

    let table_name = "flight_data";
    let name_space = "flight";

    let table = client.delete_table()
                .table_bucket_arn(table_bucket_arn)
                .namespace(name_space)
                .name(table_name)
                .send().await?;

    info!("Table deleted: {}", table_name);

    Ok(())
}

pub async fn delete_namespace(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {

    let name_space = "flight";
    let namespace = client.delete_namespace()
                .table_bucket_arn(table_bucket_arn)
                .namespace(name_space)
                .send().await?;

    info!("Namespace deleted: {}", name_space);

    Ok(())
}

#[allow(dead_code)]
pub async fn delete_table_bucket(client: &Client, table_bucket_arn: &str) -> Result<(), s3tables::Error> {

    let table = client.delete_table_bucket()
                .table_bucket_arn(table_bucket_arn)
                .send().await?;

    info!("Table bucket deleted: {}", table_bucket_arn);

    Ok(())
}

#[::tokio::main]
async fn main() -> Result<(), s3tables::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let athena_client = AthenaClient::new(&config);
    let table_bucket_arn = "arn:aws:s3tables:us-east-1:491085411627:bucket/my-s3table-49585733";

    // create_namespace(&client, table_bucket_arn).await?;
    // list_namespaces(&client, table_bucket_arn).await?;

    // create_table(&client, table_bucket_arn).await?;
    // list_tables(&client, table_bucket_arn).await?;
    // check_table(&client, table_bucket_arn).await?;

    // insert_with_athena_handler(&athena_client, table_bucket_arn).await?;

    query_with_athena(&athena_client, table_bucket_arn).await?;

    // delete_table(&client, table_bucket_arn).await?;
    // delete_namespace(&client, table_bucket_arn).await?;
    
    // delete_table_bucket(&client, table_bucket_arn).await?;

    Ok(())
}
