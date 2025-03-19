use log::{info, error};
use aws_sdk_s3tables::Client;
use aws_sdk_athena::Client as AthenaClient;
use clap::{Parser, Subcommand};
use colored::Colorize;
use env_logger::Env;
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
    template_path: Option<String>,
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
  TEMPLATE_PATH    - Optional: Path to table template YAML (default: templates/table_template.yaml)
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
    /// Uses the YAML template specified in TEMPLATE_PATH to create
    /// a new table in the configured S3 bucket.
    /// If no template is specified, it uses the default template:
    /// templates/table_template.yaml
    Create,
    
    /// Insert data from a file into the table
    /// 
    /// Reads data from a file and inserts it into the table using Athena.
    /// Supports CSV files with configurable delimiters and header options.
    /// 
    /// Examples:
    ///   s3tool insert data.csv
    ///   s3tool insert data.csv --delimiter="|" --header=false
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
        table_name: String,
    },
    
    /// Delete namespace
    DeleteNamespace {
        /// Namespace to delete
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
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
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

    for tbucket in arn_buckets.table_buckets {
        println!("Table bucket name: {:?}", tbucket.name);
        println!("Table bucket ARN: {:?}", tbucket.arn);
        println!("--------------------------------------");
    }

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
        Commands::Create => {
            let template_path = match env_var.template_path {
                Some(val) => val,
                None => {
                    info!("TEMPLATE_PATH environment variable not set. Using default: templates/table_template.yaml");
                    "templates/table_template.yaml".to_string()
                }
            };

            match create_table_from_yaml(
                &client, 
                &table_bucket_arn, 
                &template_path,
            ).await {
                Ok(_) => info!("Table created successfully\n"),
                Err(e) => error!("Error creating table: {}\n", e),
            }
        },
        Commands::Insert { file, delimiter, header } => {
            let template_path = match env_var.template_path {
                Some(val) => val,
                None => {
                    info!("TEMPLATE_PATH environment variable not set. Using default: templates/table_template.yaml");
                    "templates/table_template.yaml".to_string()
                }
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
                &template_path,
                &file,
                delimiter_fmt,
                header_fmt,
            ).await {
                Ok(_) => info!("Data inserted successfully\n"),
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
                println!("Namespace: {:?}\n", namespace_str);

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
                    println!("Table modified at {}", table.modified_at());
                }

                println!("--------------------------\n");
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
        Commands::DeleteTable { namespace, table_name } => {
            // Check if table exist
            match get_table(
                &client,
                &table_bucket_arn,
                &namespace,
                &table_name,
            ).await {
                Ok(_) => (),
                Err(e) => {
                    error!("Error getting table: {}", e);
                    return;
                }
            }
            println!("You are about to delete table: {}", table_name.green().bold());
            println!("{} This action cannot be undone!", "WARNING:".red().bold());
            println!("{}", "\nTo confirm deletion, please re-enter the table name:".bold());

            // Get confirmation from user
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            
            if input == table_name {
                // Delete table
                match delete_table(
                    &client, 
                    &table_bucket_arn,
                    &namespace,
                    &table_name,
                ).await {
                    Ok(_) => info!("Table {} deleted successfully\n", table_name),
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