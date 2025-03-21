use aws_sdk_s3tables::Error;
use aws_sdk_athena::{
    Client as AthenaClient, Error as AthenaError,
};
use aws_sdk_athena::types::error::InternalServerException;
use aws_sdk_athena::types::{
    QueryExecutionState, ResultConfiguration,
};
use crate::xai::chat::ChatCompatible;
use crate::utils::{
    TableTemplate, FieldTemplate, read_csv_file, 
    read_yaml_file, format_field,
};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use log::{error, warn, info};

const MAX_BYTES: usize = 250_000; // ~256KB

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ INSERT ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn insert_with_athena(
    client: &AthenaClient, 
    table_bucket_arn: &str,
    athena_bucket: &str,
    template_path: &str,
    file_path: &str,
    delimiter: u8,
    has_headers: bool,
    limit: u32,
) -> Result<(), AthenaError> {
    let mut table_bucket = "no_table";
    
    // Get the table-bucket name
    if let Some(table) = table_bucket_arn.split('/').last() {
        table_bucket = table;
    }

    // Load the YAML template           
    let table_template: TableTemplate = match read_yaml_file(template_path) {
        Ok(template) => template,
        Err(err) => {
            let message = format!("Error reading template file. {}", err);
            return Err(AthenaError::InternalServerException(
                InternalServerException::builder()
                    .message(&message)
                    .build(),
            )); 
        }
    };

    let namespace = &table_template.namespace;
    let table_name = &table_template.table_name;
    let fields: Vec<FieldTemplate> = table_template.fields.clone();

    // Convert Vec<FieldTemplate> to HashMap<u32, FieldTemplate>
    let field_map: HashMap<usize, FieldTemplate> = fields
        .into_iter()              
        .enumerate() // Add indices to each element: (0, field1), (1, field2), ...
        .map(|(i, field)| (i as usize + 1, field))  // Convert to (1, field1), (2, field2), ... 
        .collect();  

    // Get the data from the csv file
    let query_values: Vec<String> = match generate_insert_query(
        &field_map,
        file_path,
        delimiter,
        has_headers,
        limit,
    ) {
        Ok(values) => values,
        Err(err) => {
            let message = format!("Error generating insert query. {}", err);
            return Err(AthenaError::InternalServerException(
                InternalServerException::builder()
                    .message(&message)
                    .build(),
            )); 
        }
    };

    let mut query_num = 1;

    info!("{} requests for processing async in Athena.", query_values.len());
    info!("Wait for 5 seconds... press Ctrl + C to cancel the operation");
    sleep(Duration::from_secs(5)).await;

    for query_value in query_values.into_iter() {
        let query = format!("INSERT INTO \"s3tablescatalog/{}\".\"{}\".\"{}\" \n \
            {};",
            table_bucket,
            namespace,
            table_name,
            query_value,
        );

        let result = client.start_query_execution()
            .query_string(query)
            // Add output location configuration
            .result_configuration(
                ResultConfiguration::builder()
                    .output_location(athena_bucket)
                    .build()
            )
            .send()
            .await?;

        // Set query execution ID
        let query_id = result.query_execution_id().unwrap();
        info!("Query number {} - execution ID: {}", query_num, query_id);
        query_num += 1;
    }

    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ GENERATE INSERT QUERY ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub fn generate_insert_query(
    fields_map: &HashMap<usize, FieldTemplate>,
    file_path: &str,
    delimiter: u8,
    has_headers: bool,
    limit: u32,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut rdr = read_csv_file(
        file_path, 
        delimiter, 
        has_headers,
    )?;

    let mut vec_query: Vec<String> = Vec::new();
    let mut query = String::new();

    // Track if we've processed at least one row
    let mut first_row = true;

    let mut n_row = 0;
    let mut row_error = 0;

    for result in rdr.records() {
        let record = match result {
            Ok(record) => record,
            Err(err) => {
                let message = format!("Error reading CSV record. {}", err);
                row_error += 1;
                if row_error > 10 {
                    return Err(message.into());
                } else {
                    warn!("{}", message);
                    continue;
                }
            }
        };

        // If this isn't the first row, add a UNION ALL
        if !first_row {
            query.push_str("\nUNION ALL\n");
        } else {
            first_row = false;
        }

        query.push_str("SELECT ");

        for (serie, field) in record.iter().enumerate() {           
            // Add a comma if this isn't the first field
            if serie > 0 {
                query.push_str(", ");
            }

            let n_field = serie + 1;
            if let Some(field_template) = fields_map.get(&n_field) {

                let field_fmt: String = match format_field(
                    field, 
                    &field_template.field_type,
                ) {
                    Ok(field_fmt) => field_fmt,
                    Err(err) => {
                        let message = format!("Error formatting field. {}", err);
                        warn!("{}", message);
                        "NULL".to_string()
                    }
                };

                query.push_str(&field_fmt);
            }
        }

        if query.len() > MAX_BYTES {
            vec_query.push(query.clone());
            query.clear();
            first_row = true;
        }

        n_row += 1;
        
        if n_row == limit {
            break;
        }
    }

    vec_query.push(query.clone());
    info!("Processed {} rows.", n_row);
    
    Ok(vec_query)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ QUERY ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn query_handler(
    client: &AthenaClient,
    table_bucket_arn: &str,
    athena_bucket: &str,
    query_fparam: &str,
    query_lparam: &str,
) -> Result<Vec<Vec<String>>, AthenaError> {
    let namespace = "flights";
    let table_name = "flight_data";
    let mut table_bucket = "no_table";
    let mut query_success = false;

    if let Some(table) = table_bucket_arn.split('/').last() {
        table_bucket = table;
    }

    let query = format!("{} FROM \"s3tablescatalog/{}\".\"{}\".\"{}\" {};",
        query_fparam,
        table_bucket,
        namespace,
        table_name,
        query_lparam,
    );

    let result = client.start_query_execution()
        .query_string(query)
        // Add output location configuration
        .result_configuration(
            ResultConfiguration::builder()
                .output_location(athena_bucket)
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
                            if let Some(_query) = query_execution.query {
                                info!("Query execution succeeded.");
                                info!("File csv: {}", query_execution.result_configuration.unwrap().output_location.unwrap());
                            } else {
                                error!("Query not found in execution details.");
                            }
                            query_success = true;
                            break;
                        }
                        QueryExecutionState::Failed | QueryExecutionState::Cancelled => {
                            if let Some(_query) = query_execution.query {
                                error!("Query execution failed or was cancelled.");
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
                // println!("{:?}", row_data);
                table_data.push(row_data);
            }
        }
        Ok(table_data)
    } else {
        error!("Query execution failed or was cancelled.");
        Ok(Vec::new())
    }
}

pub async fn query_with_athena(
    client: &AthenaClient,
    table_bucket_arn: &str,
    athena_bucket: &str,
    xai_api_key: &str,
) -> Result<(), Error> {

    let query_fparam = "SELECT CASE \
            WHEN unique_carrier = 'UA' THEN 'United Airlines' \
            WHEN unique_carrier = 'WN' THEN 'Southwest Airlines' \
            WHEN unique_carrier = 'AS' THEN 'Alaska Airlines' \
            WHEN unique_carrier = 'AA' THEN 'American Airlines' \
            WHEN unique_carrier = 'DL' THEN 'Delta Air Lines' \
            WHEN unique_carrier = 'B6' THEN 'JetBlue Airways' \
            WHEN unique_carrier = 'HA' THEN 'Hawaiian Airlines' \
            WHEN unique_carrier = 'NK' THEN 'Spirit Airlines' \
            WHEN unique_carrier = 'VX' THEN 'Virgin America' \
            WHEN unique_carrier = 'F9' THEN 'Frontier Airlines' \
            ELSE unique_carrier  \
        END AS airline_name, \
        COUNT(*) AS total_cancelled";

    let query_lparam = "WHERE cancelled \
        GROUP BY 1
        ORDER BY total_cancelled DESC";

    let table_data = match query_handler(
        client, 
        table_bucket_arn, 
        athena_bucket,
        query_fparam, 
        query_lparam
    ).await {
        Ok(table_data) => table_data,
        Err(e) => {
            error!("Error: {}", e);
            return Ok(());
        }
    };

    let base_url = "https://api.x.ai/v1/chat/completions";
    let model = "grok-2-latest";
    let llm = ChatCompatible::new(base_url, model);

    let prompt = format!(
        "According to the information in this table, which are the airlines with the least \
        cancelled flights? Do not provide information about the origin of the data or \
        reference to the table.\n\n \
        Table: \n {:?}",
        table_data,
    );
   
    let response = llm
        .with_max_retries(3)
        .with_api_key(xai_api_key)
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
                None => warn!("No response choices available"),
            };
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    } 

    Ok(())
}

pub async fn query_with_llm(
    client: &AthenaClient,
    table_bucket_arn: &str,
    athena_bucket: &str,
    query_text: &str,
    xai_api_key: &str,
) -> Result<(), Error> {

    let base_url = "https://api.x.ai/v1/chat/completions";
    let model = "grok-2-latest";
    let llm = ChatCompatible::new(base_url, model);

    let prompt = format!(
        "Write a query in AWS Athena to search this table {}. Do not use any language other than English in SQL code. Response only with SQL QUERY. \
        my_table: \
        \"year\",\"month\",\"day_of_month\",\"day_of_week\",\"deptime\",\"crs_deptime\",\"arr_time\",\"crs_arr_time\",\"unique_carrier\",\"flight_num\",\"taxi_in\",\"taxi_out\",\"cancelled\",\"cancellation_code\",\"diverted\",\"carrier_delay\",\"weather_delay\",\"nas_delay\",\"security_delay\",\"flight_date\" \
        \"2000\",\"1\",\"22\",\"6\",\"1490.0\",\"1490\",\"1650.0\",\"1650\",\"AA\",\"387\",\"3.0\",\"10.0\",\"false\",\"A\",\"false\",\"21.0\",\"699.0\",\"1162.0\",\"326.0\",\"2000-01-22 01:04:59.000000\"\n \
        \"2019\",\"4\",\"19\",\"3\",\"441.0\",\"440\",\"624.0\",\"620\",\"WN\",\"2858\",\"1.0\",\"6.0\",\"false\",\"C\",\"true\",\"1098.0\",\"422.0\",\"463.0\",\"653.0\",\"2019-04-19 16:41:32.000000\"\n \
        \"2007\",\"6\",\"1\",\"7\",\"2128.0\",\"2125\",\"2262.0\",\"2260\",\"WN\",\"775\",\"9.0\",\"1.0\",\"true\",\"A\",\"false\",\"1100.0\",\"560.0\",\"300.0\",\"391.0\",\"2007-06-01 17:39:39.000000\"\n \
        \"2004\",\"1\",\"13\",\"5\",\"699.0\",\"695\",\"839.0\",\"835\",\"WN\",\"3452\",\"7.0\",\"14.0\",\"false\",\"B\",\"true\",\"440.0\",\"266.0\",\"41.0\",\"895.0\",\"2004-01-13 19:43:27.000000\"\n \
        \"2018\",\"3\",\"28\",\"7\",\"1895.0\",\"1895\",\"2088.0\",\"2085\",\"VX\",\"2557\",\"5.0\",\"10.0\",\"false\",\"D\",\"false\",\"596.0\",,\"240.0\",\"814.0\",\"2018-03-28 14:04:38.000000\" \
        ",
        query_text
    );
   
    let response = llm
        .with_max_retries(0)
        .with_api_key(xai_api_key)
        .invoke(&prompt)
        .await;

    let mut query = String::new();

    match response {
        Ok(response) => {
            match response.choices {
                Some(candidates) => {
                    for candidate in candidates {
                        #[allow(irrefutable_let_patterns)]
                        if let Some(message) = candidate.message {
                            if let Some(content) = message.content {
                                // println!("{}", content);
                                query.push_str(&content);
                            }
                        }
                    }
                }
                None => warn!("No response choices available"),
            };
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    } 

    let cleaned = query
        .replace("```sql", "")
        .replace("```", "")
        .replace("sql", "")
        .replace("'false'", "false")
        .replace("'true'", "true")
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join(" ");

    let cleaned = cleaned.split_whitespace().collect::<Vec<&str>>().join(" ");
    info!("Query: {}", cleaned);

    let parts: Vec<&str> = cleaned.split("FROM my_table").collect();
    if parts.len() != 2 {
        error!("Invalid query format");
        return Ok(());
    }

    let (query_fparam, query_lparam) = (parts[0], parts[1]);

    let table_data = match query_handler(
        client, 
        table_bucket_arn,
        athena_bucket,
        query_fparam, 
        query_lparam,
    ).await {
        Ok(result) => result,
        Err(e) => {
            error!("Error: {}", e);
            return Ok(());
        }
    };

    let llm = ChatCompatible::new(base_url, model);

    let prompt = format!(
        "According to the information in this table, answer this query: {}\n \
        Do not provide information about the origin of the data or \
        reference to the table. If there is no data in the table it responds \
        that you cannot answer the query.\n\n \
        Table: \n {:?}",
        query_text,
        table_data,
    );

    let response = llm
        .with_max_retries(3)
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
                None => warn!("No response choices available"),
            };
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    } 

    Ok(())
}