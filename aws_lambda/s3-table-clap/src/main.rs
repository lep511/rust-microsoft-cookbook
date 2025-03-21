use log::{info, error};
use aws_sdk_s3tables::Client;
use aws_sdk_athena::Client as AthenaClient;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::io::{self, Write};
use serde::Deserialize;
use envy::from_env;

pub mod xai;
pub mod table_manager;
use table_manager::create_table_from_yaml;
pub mod athena;
use athena::{
    insert_with_athena, query_with_athena, 
    query_with_llm, 
};
pub mod utils;
use utils::{
    delete_table, delete_namespace, delete_table_bucket,
    get_table, list_namespaces, list_tables, get_table_bucket,
    list_table_buckets,
};
pub mod error;

#[derive(Deserialize, Debug)]
struct Config {
    table_bucket_arn: Option<String>,
    athena_bucket: Option<String>,
    xai_api_key: Option<String>,
}

// Define CLI arguments using clap
#[derive(Parser)]
#[command(
    author = "Esteban Perez <estebanpbuday@gmail.com>",
    version,
    about = "AWS S3 Tables and Athena Management CLI",
    long_about = "A command-line tool for managing AWS S3 Tables and Athena, allowing creation of tables, data insertion, querying, and administration tasks.",
    color = clap::ColorChoice::Auto,
    after_help = "Environment variables:
  TABLE_BUCKET_ARN - Required: S3 bucket ARN for Table buckets.
  ATHENA_BUCKET    - Optional: S3 bucket name for Athena query results (default: None)
  XAI_API_KEY      - Optional: API key for XAI service (default: None)"
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
    #[command(after_help = "Examples:\n  s3tool create templates/my-template.yaml")]
    Create {
        /// Location of the yaml template file
        #[arg(required = true, value_name = "TEMPLATE_PATH")]
        #[arg(value_hint = clap::ValueHint::FilePath)]
        template: String,
    },
    
    /// Insert data from a file into the table
    /// 
    /// Reads data from a file and inserts it into the table using Athena.
    /// Supports CSV files with configurable delimiters and header options.
    /// 
    #[command(after_help = "Examples:\n  s3tool insert my-template.yaml data.csv\n  s3tool insert my-template.yaml data.csv --delimiter=\"|\" --header=false")]
    Insert {
        /// Location of the yaml template file
        #[arg(required = true, value_name = "TEMPLATE_PATH")]
        #[arg(value_hint = clap::ValueHint::FilePath)]
        template: String,

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

    /// List tables in a specific namespace
    ListTables {
        /// Namespace of the table
        namespace: String,
    },

    /// List all namespaces and their tables
    /// 
    /// Displays all namespaces in the table bucket and lists
    /// all tables within each namespace.
    #[command(visible_alias = "ls")]
    List,
    
    /// Run a query against table data
    /// 
    /// Executes an Athena query against the table data.
    Query,

    /// Delete a table from a namespace
    DeleteTable {
        /// The namespace containing the table
        #[arg(long, short, value_name = "NAMESPACE")]
        namespace: String,
        
        /// Name of the table to delete
        #[arg(long, short = 't', value_name = "TABLE_NAME")]
        table: String,
    },
    
    /// Delete namespace
    DeleteNamespace {
        /// Namespace to delete
        #[arg(long, short, value_name = "NAMESPACE")]
        namespace: String,
    },
    
    /// Delete table bucket S3
    DeleteTableBucket,
    
    /// Query with XAI LLM
    /// 
    /// Execute a natural language query using a Large Language Model.
    /// The query is translated into SQL and executed against the table.
    Llm {
        /// Natural language query to execute
        #[arg(required = true, value_name = "QUERY")]
        query_text: String,
    },
}

#[::tokio::main]
async fn main() {
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .filter_module("s3table-clap", log::LevelFilter::Info)
        .init();
    
    // Parse command line arguments using clap
    let cli = Cli::parse();
    
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let athena_client = AthenaClient::new(&config);
    
    let env_var = match from_env::<Config>() {
        Ok(config) => config,
        Err(error) => {
            info!("Error loading environment variables. {}", error);
            return;
        }
    };

    let table_bucket_arn = match env_var.table_bucket_arn {
        Some(val) => val,
        None => {
            error!("TABLE_BUCKET_ARN environment variable not set");
            return;
        }
    };

    let arn_buckets = match list_table_buckets(&client).await {
        Ok(buckets) => buckets,
        Err(e) => {
            error!("Error listing table buckets. {}", e);
            return;
        }
    };

    // Check if table bucket exists
    match get_table_bucket(&client, &table_bucket_arn).await {
        Ok(tableb) => {
            info!("Table bucket name: {}", tableb.name.bold());
        }
        Err(e) => {
            error!("Error getting table bucket. {}", e);
            return;
        }
    }
    
    // Handle commands using match
    match &cli.command {
        Commands::Create { template } => {
            if !template.ends_with(".yaml") || !template.ends_with(".yml") {
                error!("Template file must be a yaml file");
                return;
            }

            match create_table_from_yaml(
                &client, 
                &table_bucket_arn, 
                template,
            ).await {
                Ok(_) => info!("Table created successfully\n"),
                Err(e) => error!("Error creating table: {}\n", e),
            }
        },
        Commands::Insert { template, file, delimiter, header, limit } => {
            // Check if template finish with yaml
            if !template.ends_with(".yaml") || !template.ends_with(".yml") {
                error!("Template file must be a yaml file");
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

            match insert_with_athena(
                &athena_client, 
                &table_bucket_arn,
                &athena_bucket_fmt,
                template,
                &file,
                delimiter_fmt,
                header_fmt,
                limit_row,
            ).await {
                Ok(_) => (),
                Err(e) => error!("Error inserting data: {}\n", e),
            }
        },
        Commands::ListTables { namespace } => {
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

            for table in tables.tables() {
                println!("Table: {:?}", table.name);
                println!("Created at: {:?}", table.created_at);
                println!("Table modified at {}", table.modified_at());
                println!("--------------------------");
            }
        },
        Commands::List => {
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
            }
        },
        Commands::Query => {
            let athena_bucket = match env_var.athena_bucket {
                Some(val) => val,
                None => {
                    error!("ATHENA_BUCKET environment variable not set.");
                    return;
                }
            };
            let athena_bucket_fmt = format!("s3://{}/", athena_bucket);

            let xai_api_key = match env_var.xai_api_key {
                Some(val) => val,
                None => {
                    error!("XAI_API_KEY environment variable not set.");
                    return;
                }
            };

            match query_with_athena(
                &athena_client, 
                &table_bucket_arn,
                &athena_bucket_fmt,
                &xai_api_key,
            ).await {
                Ok(_) => info!("Query executed successfully\n"),
                Err(e) => error!("Error executing query: {}\n", e),
            }
        },
        Commands::DeleteTable { namespace, table } => {
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
            match delete_namespace(&client, &table_bucket_arn, &namespace).await {
                Ok(_) => info!("Namespace deleted successfully"),
                Err(e) => error!("Error deleting namespace: {}", e),
            }
        },
        Commands::DeleteTableBucket => {
            match delete_table_bucket(&client, &table_bucket_arn).await {
                Ok(_) => info!("Table bucket deleted successfully"),
                Err(e) => error!("Error deleting table bucket: {}", e),
            }
        },
        Commands::Llm { query_text } => {
            let athena_bucket = match env_var.athena_bucket {
                Some(val) => val,
                None => {
                    error!("ATHENA_BUCKET environment variable not set.");
                    return;
                }
            };
            let athena_bucket_fmt = format!("s3://{}/", athena_bucket);

            let xai_api_key = match env_var.xai_api_key {
                Some(val) => val,
                None => {
                    error!("XAI_API_KEY environment variable not set.");
                    return;
                }
            };

            match query_with_llm(
                &athena_client, 
                &table_bucket_arn,
                &athena_bucket_fmt, 
                &query_text,
                &xai_api_key,
            ).await {
                Ok(_) => info!("Query executed successfully"),
                Err(e) => error!("Error executing query: {}", e),
            }
        }
    }
}