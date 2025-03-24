#[allow(dead_code)]
use langchain::openai::chat::ChatOpenAI;
use langchain::openai::libs::ChatResponse;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use env_logger::Env;

#[derive(Serialize, Deserialize, Debug)]
struct QueryData {
    tablename: String,
    namespace: String,
    query: String,
}

#[tokio::main]
async fn main() {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatOpenAI::new("o3-mini");

    let yaml_contents = match read_file("tests/files/template.yaml").await {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    let query = "team with the most goals";

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
        query,
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

    let response: ChatResponse = match llm
        .with_tools(vec![sql_function])
        .with_tool_choice(tool_choice)
        .invoke(&prompt)
        .await {
            Ok(response) => response,
            Err(e) => {
                println!("Error invoking LLM: {}", e);
                return;
            }
        };

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
    let query_data: QueryData = match serde_json::from_str(&function_args) {
        Ok(data) => data,
        Err(e) => {
            println!("Error parsing JSON: {}", e);
            return;
        }
    };

    // Access the extracted data
    println!("Table name: {}", query_data.tablename);
    println!("Namespace: {:?}", query_data.namespace);
    println!("Query: {}", query_data.query);
    
}

/// Asynchronously reads a text file from the specified path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file to be read
///
/// # Returns
///
/// * `io::Result<String>` - A Result containing either the file contents as a String
///   or an IO error if the file couldn't be opened or read
///
/// # Errors
///
/// This function will return an error if:
/// * The file at the specified path does not exist
/// * The process lacks permissions to read the file
/// * The file content is not valid UTF-8
/// * Any other I/O error occurs during reading
///
/// # Example
///
/// ```
/// let contents = read_file("example.txt").await?;
/// println!("File content: {}", contents);
/// ```
async fn read_file(path: &str) -> io::Result<String> {
    // Open the file asynchronously, returning any IO errors
    let mut file = File::open(path).await?;
    
    // Create an empty String to hold the file contents
    let mut contents = String::new();
    
    // Read the entire file contents into the string asynchronously
    // The read_to_string method will return an error if the file doesn't contain valid UTF-8
    file.read_to_string(&mut contents).await?;
    
    Ok(contents)
}