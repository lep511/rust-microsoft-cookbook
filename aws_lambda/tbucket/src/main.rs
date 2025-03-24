use log::{info, warn, error};
use aws_sdk_s3tables::Client;
use aws_sdk_athena::Client as AthenaClient;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::io::{self, Write};
use std::path::Path;
use serde::Deserialize;
use envy::from_env;

pub mod openai;
pub mod table_manager;
use table_manager::create_table_from_yaml;
pub mod athena;
use athena::{
    insert_with_athena, query_with_athena, 
    query_with_llm, process_insert_items, 
};
pub mod utils;
use utils::{
    delete_table, delete_namespace, delete_table_bucket, get_table,
    pause_for_keypress, list_namespaces, list_tables, save_log_file,
    ProcessFileResult, list_table_buckets, get_user_confirmation,
};
pub mod error;

const DEFAULT_TEMPLATE: &str = "s3table_template.yaml";
const DEFAULT_OPENAI_MODEL: &str = "o3-mini";
const APPROVED_VALUES: [&str; 8] = ["yes", "y", "ok", "OK", "Ok", "Yes", "Y", "YES"];

#[derive(Deserialize, Debug)]
struct Config {
    table_bucket_arn: Option<String>,
    template_path: Option<String>,
    athena_bucket: Option<String>,
    openai_api_key: Option<String>,
    openai_model: Option<String>,
}

// Define CLI arguments using clap
#[derive(Parser)]
#[command(
    author = "Esteban Perez <estebanpbuday@gmail.com>",
    version,
    about = "AWS S3 Tables management CLI",
    long_about = "A command-line tool for managing AWS S3 Tables, allowing creation of tables, data insertion, querying, and administration tasks.",
    color = clap::ColorChoice::Auto,
    after_help = "Environment variables:
  TABLE_BUCKET_ARN - Required: S3 bucket ARN for Table buckets.
  TEMPLATE_PATH    - Optional: Path to the template file (default: s3table_template.yaml)
  ATHENA_BUCKET    - Optional: S3 bucket name for Athena query results (default: None)
  OPENAI_API_KEY   - Optional: API key for OpenAI API service (default: None)
  OPENAI_MODEL     - Optional: Model name for OpenAI API service (default: o3-mini)"
)]

#[command(author, version, about, long_about = None)]
struct Cli {   
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new namespace and table from a template
    /// 
    /// Uses the YAML template file to create
    /// a new table in the configured S3 bucket.
    /// 
    #[command(after_help = "Examples:\n  echo \"Table bucket template here.\" > s3table_template.yaml\ns3tool create")]
    Create,
    
    /// Insert data from a file into the table
    /// 
    /// Reads data from a file and inserts it into the table using Athena.
    /// Supports CSV files with configurable delimiters and header options.
    /// 
    #[command(after_help = "Examples:\n  s3tool insert data.csv\n  s3tool insert data.csv --delimiter=\"|\" --header=false")]
    Insert {
        /// Path to the file containing data to insert
        #[arg(required = true, value_name = "FILE_PATH")]
        #[arg(value_hint = clap::ValueHint::FilePath)]
        file: String,

        /// Character used to separate values in the CSV file
        #[arg(short, long, value_name = "CHAR", default_value = ",")]
        delimiter: Option<String>,

        /// Whether the CSV file contains a header row
        #[arg(short = 'x', long, default_value = "true")]
        header: Option<bool>,

        /// Maximum number of rows to process from the input file
        #[arg(short, long)] 
        limit: Option<u32>,
    },

    /// List all table buckets in the region
    #[command(visible_alias = "ls")]
    List,

    /// List all namespaces and their tables
    /// 
    /// Displays all namespaces in the table bucket and lists
    /// all tables within each namespace.
    #[command(visible_alias = "ln")]
    ListNamespaces,

    /// List tables in a specific namespace
    #[command(visible_alias = "lt")]
    ListTables {
        /// Namespace of the table
        namespace: String,
    },
    
    /// Run a query against table data
    /// 
    /// Executes an Athena query against the table data.
    Query,

    /// Query with OpenAI LLM
    /// 
    /// Execute a natural language query using a Large Language Model.
    /// The query is translated into SQL and executed against the table.
    Llm {
        /// Natural language query to execute
        #[arg(required = true, value_name = "QUERY")]
        query_text: String,
    },

    /// Delete a table from a namespace
    #[command(visible_alias = "dt")]
    DeleteTable {
        /// The namespace containing the table
        #[arg(long, short, value_name = "NAMESPACE")]
        namespace: String,
        
        /// Name of the table to delete
        #[arg(long, short = 't', value_name = "TABLE_NAME")]
        table: String,
    },
    
    /// Delete namespace
    #[command(visible_alias = "dn")]
    DeleteNamespace {
        /// Namespace to delete
        #[arg(long, short, value_name = "NAMESPACE")]
        namespace: String,
    },
    
    /// Delete table bucket S3
    #[command(visible_alias = "db")]
    DeleteTableBucket,
}

#[::tokio::main]
async fn main() {
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .filter_module("tbucket", log::LevelFilter::Info)
        .init();
    
    // Parse command line arguments using clap
    let cli = Cli::parse();
    
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let athena_client = AthenaClient::new(&config);
    
    let env_var = match from_env::<Config>() {
        Ok(config) => config,
        Err(error) => {
            error!("Error loading environment variables. {}", error);
            return;
        }
    };

    let table_bucket_arn: String = match env_var.table_bucket_arn {
        Some(table_bucket) => table_bucket,
        None => "table_bucket_not_set".to_string()
    };

    let template = match env_var.template_path {
        Some(path) => path,
        None => DEFAULT_TEMPLATE.to_string(),
    };

    let open_ai_model = match env_var.openai_model {
        Some(model) => model,
        None => DEFAULT_OPENAI_MODEL.to_string(),
    };

    let template_path = Path::new(&template);
    
    // Handle commands using match
    match &cli.command {
        Commands::Create => {
            if table_bucket_arn == "table_bucket_not_set" {
                error!("TABLE_BUCKET_ARN environment variable not set.");
                return;
            } else if !template_path.exists() {
                error!("Template file not found");
                return;
            }
            
            match create_table_from_yaml(
                &client, 
                &table_bucket_arn, 
                template_path,
            ).await {
                Ok(_) => info!("Table created successfully\n"),
                Err(e) => error!("Error creating table: {}\n", e),
            }
        },
        Commands::Insert { file, delimiter, header, limit } => {
            if table_bucket_arn == "table_bucket_not_set" {
                error!("TABLE_BUCKET_ARN environment variable not set.");
                return;
            } else if !template_path.exists() {
                error!("Template file not found");
                return;
            }

            let limit_row: u32 = match limit {
                Some(val) => *val,
                None => 0, // Default value
            };
            
            let athena_bucket = match env_var.athena_bucket {
                Some(val) => val,
                None => {
                    error!("ATHENA_BUCKET environment variable not set.");
                    return;
                }
            };
            let athena_bucket_fmt = format!("s3://{}/", athena_bucket);
            
            let delimiter_fmt: u8 = match delimiter {
                Some(d) => d.as_bytes()[0],
                None => b',', // Default value
            };

            let header_fmt: bool = match header {
                Some(h) => *h,
                None => true, // Default value
            };

            let process_fields: ProcessFileResult = match insert_with_athena(
                template_path,
                &file,
                delimiter_fmt,
                header_fmt,
                limit_row,
            ).await {
                Ok(fields) => fields,
                Err(e) => {
                    error!("Error inserting data: {}\n", e);
                    return;
                }
            };

            info!("{} requests for processing async in Athena.", process_fields.fields.len());
            if process_fields.errors.len() > 1 {
                info!("{} errors found.", process_fields.errors.len());
                let mut errors_string = String::new();
                for error in &process_fields.errors {
                    errors_string.push_str(&format!("{}\n", error));
                }
                let file_log_eror = match save_log_file(&errors_string).await {
                    Ok(file) => file,
                    Err(err) => {
                        let message = format!("Error saving log file. {}", err);
                        warn!("{}", message);
                        "no_file_log".to_string()
                    }
                };
                if file_log_eror != "no_file_log" {
                    info!("Error log file has been saved: {}", file_log_eror);
                }
            } else {
                info!("No errors found processing the file {}", &file);
            }

            let prompt = "Would you like to continue (y/N)";
            match get_user_confirmation(prompt).await {
                Ok(input) => {                
                    if !APPROVED_VALUES.contains(&input.as_str()) {
                        info!("Operation cancelled by user.\n");
                        return;
                    }
                }
                Err(e) => {
                    error!("Error reading input: {}", e);
                    return;
                }
            }
            match process_insert_items(
                &athena_client,
                &table_bucket_arn,
                template_path,
                &athena_bucket_fmt,
                &process_fields.fields,
            ).await {
                Ok(_) => info!("Data inserted successfully\n"),
                Err(e) => error!("Error inserting data: {}\n", e),
            }
        },
        Commands::List => {
            let arn_buckets = match list_table_buckets(&client).await {
                Ok(buckets) => buckets,
                Err(e) => {
                    error!("Error listing table buckets. {}", e);
                    return;
                }
            };
            if arn_buckets.table_buckets().is_empty() {
                println!("No table buckets found");
                return;
            }
            let mut n_count = 0;
            for table_b in arn_buckets.table_buckets() {
                println!("Table Bucket: {}", table_b.name.green());
                println!("ARN: {:?}", table_b.arn);
                println!("----------------------------------------------\n");
                n_count += 1;
                if n_count > 10 {
                    println!("Press any key to continue...");
                    n_count = 0;
                    match pause_for_keypress().await {
                        Ok(_) => (),
                        Err(_) => {
                            continue;
                        }
                    }
                }
            }
        }
        Commands::ListNamespaces => {
            if table_bucket_arn == "table_bucket_not_set" {
                error!("TABLE_BUCKET_ARN environment variable not set.");
                return;
            }

            let namespaces = match list_namespaces(
                &client, 
                &table_bucket_arn
            ).await {
                Ok(namespaces) => namespaces,
                Err(e) => {
                    error!("Error listing namespaces: {}\n", e);
                    return;
                }
            };

            // Check if there are no namespaces
            if namespaces.namespaces().is_empty() {
                println!("No namespaces found");
                return;
            }
            
            let mut n_count = 0;
            for namespace in namespaces.namespaces() {
                let namespace_str = &namespace.namespace()[0];
                println!("--| Namespace: {:?} |--\n", namespace_str);

                let tables = match list_tables(
                    &client,
                    &table_bucket_arn,
                    namespace_str,
                ).await {
                    Ok(tables) => tables,
                    Err(e) => {
                        error!("Error listing tables: {}\n", e);
                        continue;
                    }
                };

                for table in tables.tables() {
                    println!("Table: {:?}", table.name);
                    println!("Created at: {:?}", table.created_at);
                    println!("Table modified at {}\n", table.modified_at());
                }

                println!("----------------------------------------------\n");
                n_count += 1;
                if n_count > 10 {
                    println!("Press any key to continue...");
                    n_count = 0;
                    match pause_for_keypress().await {
                        Ok(_) => (),
                        Err(_) => {
                            continue;
                        }
                    }
                }
            }
        },
        Commands::ListTables { namespace } => {
            if table_bucket_arn == "table_bucket_not_set" {
                error!("TABLE_BUCKET_ARN environment variable not set.");
                return;
            } 

            let tables = match list_tables(
                &client, 
                &table_bucket_arn,
                &namespace
            ).await {
                Ok(tables) => tables,
                Err(e) => {
                    error!("Error listing tables: {}\n", e);
                    return;
                }
            };

            let mut n_count = 0;
            for table in tables.tables() {
                println!("Table: {:?}", table.name);
                println!("Created at: {:?}", table.created_at);
                println!("Table modified at {}", table.modified_at());
                println!("--------------------------");
                n_count += 1;
                if n_count > 10 {
                    println!("Press any key to continue...");
                    n_count = 0;
                    match pause_for_keypress().await {
                        Ok(_) => (),
                        Err(_) => {
                            continue;
                        }
                    }
                }
            }
        },
        Commands::Query => {
            if table_bucket_arn == "table_bucket_not_set" {
                error!("TABLE_BUCKET_ARN environment variable not set.");
                return;
            } 

            let athena_bucket = match env_var.athena_bucket {
                Some(val) => val,
                None => {
                    error!("ATHENA_BUCKET environment variable not set.");
                    return;
                }
            };
            let athena_bucket_fmt = format!("s3://{}/", athena_bucket);

            let openai_api_key = match env_var.openai_api_key {
                Some(val) => val,
                None => {
                    error!("OPENAI_API_KEY environment variable not set.");
                    return;
                }
            };

            match query_with_athena(
                &athena_client, 
                &table_bucket_arn,
                &athena_bucket_fmt,
                &openai_api_key,
            ).await {
                Ok(_) => info!("Query executed successfully\n"),
                Err(e) => error!("Error executing query: {}\n", e),
            }
        },
        Commands::DeleteTable { namespace, table } => {
            if table_bucket_arn == "table_bucket_not_set" {
                error!("TABLE_BUCKET_ARN environment variable not set.");
                return;
            } 

            // Check if table exist
            match get_table(
                &client,
                &table_bucket_arn,
                &namespace,
                &table,
            ).await {
                Ok(_) => (),
                Err(e) => {
                    error!("Error getting table: {}", e);
                    return;
                }
            }
            println!("You are about to delete table: {}", table.green().bold());
            println!("{} This action cannot be undone!", "WARNING:".red().bold());
            println!("{}", "\nTo confirm deletion, please re-enter the table name:".bold());

            // Get confirmation from user
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            
            if input == table {
                // Delete table
                match delete_table(
                    &client, 
                    &table_bucket_arn,
                    &namespace,
                    &table,
                ).await {
                    Ok(_) => info!("Table {} deleted successfully\n", table),
                    Err(e) => error!("Error deleting table: {}\n", e),
                }
            } else {
                println!("Table names don't match. {}", "Deletion canceled.\n".bold());
            }
        },
        Commands::DeleteNamespace { namespace } => {
            if table_bucket_arn == "table_bucket_not_set" {
                error!("TABLE_BUCKET_ARN environment variable not set.");
                return;
            } 

            match delete_namespace(&client, &table_bucket_arn, &namespace).await {
                Ok(_) => info!("Namespace deleted successfully"),
                Err(e) => error!("Error deleting namespace: {}", e),
            }
        },
        Commands::DeleteTableBucket => {
            if table_bucket_arn == "table_bucket_not_set" {
                error!("TABLE_BUCKET_ARN environment variable not set.");
                return;
            } 

            match delete_table_bucket(&client, &table_bucket_arn).await {
                Ok(_) => info!("Table bucket deleted successfully"),
                Err(e) => error!("Error deleting table bucket: {}", e),
            }
        },
        Commands::Llm { query_text } => {
            if table_bucket_arn == "table_bucket_not_set" {
                error!("TABLE_BUCKET_ARN environment variable not set.");
                return;
            } 

            let athena_bucket = match env_var.athena_bucket {
                Some(val) => val,
                None => {
                    error!("ATHENA_BUCKET environment variable not set.");
                    return;
                }
            };
            let athena_bucket_fmt = format!("s3://{}/", athena_bucket);

            let openai_api_key = match env_var.openai_api_key {
                Some(val) => val,
                None => {
                    error!("OPENAI_API_KEY environment variable not set.");
                    return;
                }
            };

            match query_with_llm(
                &athena_client, 
                &table_bucket_arn,
                template_path,
                &athena_bucket_fmt, 
                &query_text,
                &openai_api_key,
                &open_ai_model,
            ).await {
                Ok(_) => info!("Query executed successfully"),
                Err(e) => error!("Error executing query: {}", e),
            }
        }
    }
}