use aws_sdk_athena::Client as AthenaClient;
use aws_sdk_athena::types::{
    QueryExecutionState, ResultConfiguration,
};
use crate::openai::chat::ChatOpenAI;
use crate::openai::libs::ChatResponse;
use crate::utils::{
    TableTemplate, FieldTemplate, ProcessFileResult,
    read_yaml_file, format_field, read_file, QueryData,
};
use crate::error::MainError;
use csv_async::AsyncReaderBuilder;
use serde_json::json;
use chrono::{Utc, SecondsFormat};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::File as TokioFile;
use tokio::time::{sleep, Duration};
use futures_util::TryStreamExt;
use tokio_util::compat::TokioAsyncReadCompatExt;
use log::{error, warn, info};

const MAX_BYTES: usize = 250_000; // ~256KB

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ INSERT WITH ATHENA ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn insert_with_athena(
    template_path: impl AsRef<Path>,
    file_path: &str,
    delimiter: u8,
    has_headers: bool,
    limit: u32,
) -> Result<ProcessFileResult, MainError> {   
    // Load the YAML template           
    let table_template: TableTemplate = match read_yaml_file(template_path).await {
        Ok(template) => template,
        Err(err) => {
            let message = format!("Error reading template file. {}", err);
            return Err(MainError::GenericError { 
                message,
            });
        }
    };
    let fields: Vec<FieldTemplate> = table_template.fields.clone();

    // Convert Vec<FieldTemplate> to HashMap<u32, FieldTemplate>
    let field_map: HashMap<usize, FieldTemplate> = fields
        .into_iter()              
        .enumerate() // Add indices to each element: (0, field1), (1, field2), ...
        .map(|(i, field)| (i as usize + 1, field))  // Convert to (1, field1), (2, field2), ... 
        .collect();  

    // Get the data from the csv file
    let process_fields: ProcessFileResult = match generate_insert_query(
        &field_map,
        file_path,
        delimiter,
        has_headers,
        limit,
    ).await {
        Ok(values) => values,
        Err(err) => {
            let message = format!("Error generating insert query. {}", err);
            return Err(MainError::GenericError { 
                message,
            });
        }
    };

    Ok(process_fields)
}

pub async fn process_insert_items(
    client: &AthenaClient, 
    table_bucket_arn: &str,
    template_path: impl AsRef<Path>,
    athena_bucket: &str,
    query_values: &Vec<String>,
) -> Result<(), MainError> {
    // Get the table-bucket name
    let table_bucket = table_bucket_arn.split('/').last().unwrap_or("no_table");
    
    // Load the YAML template           
    let table_template: TableTemplate = match read_yaml_file(template_path).await {
        Ok(template) => template,
        Err(err) => {
            let message = format!("Error reading template file. {}", err);
            return Err(MainError::GenericError { 
                message,
            });
        }
    };

    let namespace = &table_template.namespace;
    let table_name = &table_template.table_name;
    
    let mut query_num = 1;
    for query_value in query_values.into_iter() {
        let query = format!("INSERT INTO \"s3tablescatalog/{}\".\"{}\".\"{}\" \n \
            {};",
            table_bucket,
            namespace,
            table_name,
            query_value,
        );

        let result = match client.start_query_execution()
            .query_string(query)
            // Add output location configuration
            .result_configuration(
                ResultConfiguration::builder()
                    .output_location(athena_bucket)
                    .build()
            )
            .send()
            .await {
                Ok(result) => result,
                Err(err) => {
                    let message = format!("Error executing query. {}", err);
                    return Err(MainError::GenericError {
                        message,
                    });
                }
            };

        // Set query execution ID
        let query_id = result.query_execution_id().unwrap();
        info!("Query number {} - execution ID: {}", query_num, query_id);
        query_num += 1;
    }
    
    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ GENERATE INSERT QUERY ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn generate_insert_query(
    fields_map: &HashMap<usize, FieldTemplate>,
    file_path: &str,
    delimiter: u8,
    has_headers: bool,
    limit: u32,
) -> Result<ProcessFileResult, MainError> {
    // Open the file asynchronously
    let file = TokioFile::open(file_path).await?;

    // Convert to a type that implements futures::AsyncRead
    let compat_file = file.compat();

    // Create the CSV reader with the compatible file
    let mut reader = AsyncReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(has_headers)
        .create_reader(compat_file);

    let mut vec_query: Vec<String> = vec![];
    let mut rows_errors: Vec<String> = vec![];
    let mut query = String::new();
    let mut n_row = 0;
    let mut n_field = 0;

    // Track if we've processed at least one row
    let mut first_row = true;
 
    // Process the CSV one record at a time
    let mut records = reader.records();
    loop {
        match records.try_next().await {
            Ok(record) => {
                if record == None {
                    break;
                }
                let mut serie = 0;
                // If this isn't the first row, add a UNION ALL
                if !first_row {
                    query.push_str("\nUNION ALL\n");
                } else {
                    first_row = false;
                }

                query.push_str("SELECT ");

                record.iter().for_each(|row| {          
                    row.iter().for_each(|field| {
                        // Add a comma if this isn't the first field
                        if serie > 0 {
                            query.push_str(", ");
                        }

                        n_field = serie + 1;
                        if let Some(field_template) = fields_map.get(&n_field) {
                            let field_fmt: String = match format_field(
                                field, 
                                &field_template.field_type,
                            ) {
                                Ok(field_fmt) => field_fmt,
                                Err(err) => {
                                    let now = Utc::now();
                                    // Format the timestamp in RFC 3339 format with milliseconds and "Z" suffix.
                                    let timestamp = now.to_rfc3339_opts(SecondsFormat::Millis, true);
                                    let message = format!(
                                        "[{}] formatting field. {} - {}", 
                                        timestamp, 
                                        &field_template.field_type,
                                        err,
                                    );
                                    rows_errors.push(message);  
                                    "NULL".to_string()
                                }
                            };

                            query.push_str(&field_fmt);
                            serie += 1;
                        }
                    })
                });

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
            Err(err) => {
                let now = Utc::now();
                // Format the timestamp in RFC 3339 format with milliseconds and "Z" suffix.
                let timestamp = now.to_rfc3339_opts(SecondsFormat::Millis, true);
                let message = format!(
                    "[{}] Error reading CSV record - {}", 
                    timestamp, 
                    err,
                );
                rows_errors.push(message);
                continue;         
            }
        }
    }

    vec_query.push(query.clone());

    let response = ProcessFileResult {
        fields: vec_query,
        errors: rows_errors,
        n_columns: n_field,
    };
    
    Ok(response)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ QUERY HANDLER ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn query_handler(
    client: &AthenaClient,
    table_bucket_arn: &str,
    athena_bucket: &str,
    query: &str,
) -> Result<Vec<Vec<String>>, MainError> {
    let namespace = "flights";
    let table_name = "flight_data";
    let mut table_bucket = "no_table";
    let mut query_success = false;

    if let Some(table) = table_bucket_arn.split('/').last() {
        table_bucket = table;
    }

    let result = match client.start_query_execution()
        .query_string(query)
        // Add output location configuration
        .result_configuration(
            ResultConfiguration::builder()
                .output_location(athena_bucket)
                .build()
        )
        .send()
        .await {
            Ok(result) => result,
            Err(err) => {
                let message = format!("Error executing query. {}", err);
                return Err(MainError::GenericError {
                    message,
                });
            }
        };

    // Set query execution ID
    let query_execution_id = result.query_execution_id().unwrap();
    info!("Query execution ID: {}", query_execution_id);

    // Polling for query execution completion
    loop {
        let response = match client
            .get_query_execution()
            .query_execution_id(query_execution_id)
            .send()
            .await {
                Ok(result) => result,
                Err(err) => {
                    let message = format!("Error executing query. {}", err);
                    return Err(MainError::GenericError {
                        message,
                    });
                }
            };

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
        let results = match client.get_query_results()
            .query_execution_id(query_execution_id)
            .send()
            .await {
                Ok(result) => result,
                Err(err) => {
                    let message = format!("Error executing query. {}", err);
                    return Err(MainError::GenericError {
                        message,
                    });
                }
            };

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
    openai_api_key: &str,
) -> Result<(), MainError> {

    let query = "SELECT CASE \
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

    let table_data = match query_handler(
        client, 
        table_bucket_arn, 
        athena_bucket,
        query, 
    ).await {
        Ok(table_data) => table_data,
        Err(e) => {
            error!("Error: {}", e);
            return Ok(());
        }
    };

    let llm = ChatOpenAI::new("open_ai_model");

    let prompt = format!(
        "According to the information in this table, which are the airlines with the least \
        cancelled flights? Do not provide information about the origin of the data or \
        reference to the table.\n\n \
        Table: \n {:?}",
        table_data,
    );
   
    let response = llm
        .with_max_retries(3)
        .with_api_key(openai_api_key)
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

// ToDo ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ QUERY WITH LLM ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn query_with_llm(
    client: &AthenaClient,
    table_bucket_arn: &str,
    template_path: impl AsRef<Path>,
    athena_bucket: &str,
    query_text: &str,
    openai_api_key: &str,
    open_ai_model: &str,
) -> Result<(), MainError> {
    let llm = ChatOpenAI::new(open_ai_model);

    let yaml_contents = read_file(template_path).await?;

    let prompt = format!(
        "Write a SQL query in AWS Athena to find: {}.\n \
        Use the following YAML template YAML of a Parquet Iceberg table.\n\n \
        {} \
        \n\n \
        # Output Format \
        \n \
        Provide a SQL query suitable for executing in AWS Athena.
        \n \
        # Notes \
        \n \
        - Make sure to replace placeholder values with actual values from YAML when implementing.\n \
        - The query must target the specified Athena table using the correct namespace and table name.\n",
        query_text,
        yaml_contents,
    );

    let sql_function = json!({
        "type":"function",
        "function":{
            "name":"execute_sql_query",
            "strict":true,
            "parameters":{
                "type":"object",
                "required":[
                    "query",
                    "tablename",
                    "namespace"
                ],
                "properties":{
                    "query":{
                        "type":"string",
                        "description":"The SQL query to be executed"
                    },
                    "tablename":{
                        "type":"string",
                        "description":"The name of the table"
                    },
                    "namespace":{
                        "type":"string",
                        "description":"The name of the namespace"
                    }
                },
                "additionalProperties":false
            },
            "description":"Executes a SQL query against a database"
        }
    });

    let tool_choice = json!({"type": "function", "function": {"name": "execute_sql_query"}});
   
    let response: ChatResponse = llm
        .with_tools(vec![sql_function])
        .with_tool_choice(tool_choice)
        .with_max_retries(0)
        .with_api_key(openai_api_key)
        .invoke(&prompt)
        .await?;

    let mut _function_name = String::new();
    let mut function_args = String::new();

    match response.choices {
        Some(candidates) => {
            candidates.iter()
                .filter_map(|candidate| {
                    candidate.message.as_ref().and_then(|msg| 
                        if let Some(tool_calls) = &msg.tool_calls {
                            Some(tool_calls.iter().for_each(|call| {
                                if let Some(func) = call.get("function") {
                                    if let Some(name) = func.get("name") {
                                        _function_name = name.as_str().unwrap_or_default().to_string();
                                    }
                                    if let Some(args) = func.get("arguments") {
                                        function_args = args.as_str().unwrap_or_default().to_string();
                                    }
                                }
                            }))
                        } else {
                            msg.content.as_ref().map(|content| println!("{}", content))
                        }
                    )
                })
                .count();
        }
        None => println!("No response choices available"),
    };

    // Parse the JSON string
    let query_data: QueryData = serde_json::from_str(&function_args)?;
    let query = query_data.query.clone();
    let namespace = query_data.namespace.clone();
    let table_name = query_data.tablename.clone();
    let table_bucket = table_bucket_arn.split('/').last().unwrap_or("no_table");

    let source = format!("\"s3tablescatalog/{}\".\"{}\".\"{}\"",
        table_bucket,
        namespace,
        table_name,
    );

    let query_fmt = replace_from_line(&query, &source);

    let table_data = query_handler(
        client, 
        table_bucket_arn,
        athena_bucket,
        &query_fmt, 
    ).await?;

    let llm = ChatOpenAI::new(open_ai_model);

    let prompt = format!(
        "According to the information in this table, answer this query: {}\n \
        Do not provide information about the origin of the data or \
        reference to the table. If there is no data in the table it responds \
        that you cannot answer the query.\n\n \
        Table: \n {:?}",
        query_text,
        table_data,
    );

    let response: ChatResponse = llm
        .with_max_retries(3)
        .invoke(&prompt)
        .await?;

    match response.choices {
        Some(candidates) => {
            candidates.iter()
                .filter_map(|candidate| candidate
                    .message.as_ref()?
                    .content.as_ref()
                ).for_each(|content| println!("{}", content));
        }
        None => println!("Sorry, we were unable to connect to the OpenAI \
                    API. Please try again later."),
    };

    Ok(())
}

pub fn replace_from_line(sql_query: &str, new_input: &str) -> String {
    sql_query
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("FROM") {
                format!("FROM {}", new_input)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}