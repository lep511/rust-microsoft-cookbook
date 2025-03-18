use log::{info, error};
use aws_sdk_s3tables::Client;
use aws_sdk_athena::Client as AthenaClient;
use clap::{Parser, Subcommand};
use colored::Colorize;
use env_logger::Env;
use std::io::{self, Write};
use std::env;

pub mod generate_data;
pub mod compatible;
pub mod table_manager;
use table_manager::create_table_from_yaml;
pub mod athena;
use athena::{
    insert_with_athena_handler, query_with_athena, 
    query_with_llm, 
};
pub mod utils;
use utils::{
    delete_table, delete_namespace, delete_table_bucket,
    get_table, list_namespaces, list_tables, get_table_bucket,
};

// Define CLI arguments using clap
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {   
    /// Path to the YAML template file that defines the table structure
    /// - default path: templates/table_template.yaml
    #[arg(short, long)]
    template_path: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create namespace and table
    Create,
    
    /// Insert data into the table
    Insert,

    /// List tables in the namespace
    ListTables {
        /// Namespace of the table
        namespace: String,
    },

    /// List all namespaces
    List,
    
    /// Query data from the table
    Query,

    /// Delete simple table
    DeleteTable {
        /// Namespace of the table
        namespace: String,
        /// Table simple name
        table_name: String,
    },
    
    /// Delete namespace
    DeleteNamespace {
        /// Namespace to delete
        namespace: String,
    },
    
    /// Delete table bucket S3
    DeleteTableBucket,
    
    /// Query with LLM
    Llm {
        /// Query text to pass to LLM
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
    let table_bucket_arn = match env::var("TABLE_BUCKET_ARN") {
        Ok(val) => val,
        Err(e) => {
            error!("Error reading environment variable TABLE_BUCKET_ARN: {}", e);
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

    let template_path = match &cli.template_path {
        Some(path) => path,
        None => {
            info!("No template path provided, using default");
            "templates/table_template.yaml"
        }
    };
    
    // Handle commands using match
    match &cli.command {
        Commands::Create => {
            match create_table_from_yaml(
                &client, 
                &table_bucket_arn, 
                template_path,
            ).await {
                Ok(_) => info!("Table created successfully\n"),
                Err(e) => error!("Error creating table: {}\n", e),
            }
        },
        Commands::Insert => {
            match insert_with_athena_handler(&athena_client, &table_bucket_arn).await {
                Ok(_) => info!("Data inserted successfully\n"),
                Err(e) => error!("Error inserting data: {}\n", e),
            }
        },
        Commands::ListTables { namespace } => {
            match list_tables(&client, &table_bucket_arn, namespace).await {
                Ok(_) => (),
                Err(e) => error!("Error listing tables: {}\n", e),
            }
        },
        Commands::List => {
            match list_namespaces(&client, &table_bucket_arn).await {
                Ok(_) => (),
                Err(e) => error!("Error listing namespaces: {}\n", e),
            }
        },
        Commands::Query => {
            match query_with_athena(&athena_client, &table_bucket_arn).await {
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
            match query_with_llm(&athena_client, &table_bucket_arn, query_text).await {
                Ok(_) => info!("Query executed successfully"),
                Err(e) => error!("Error executing query: {}", e),
            }
        },
    }

}