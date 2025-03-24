# S3Tables Management Tool

This tool allows you to manage S3 tables using AWS services like Athena and S3. It supports various operations such as creating, inserting, querying, and deleting tables and namespaces.

## Requirements

- Rust
- AWS CLI configured with appropriate credentials

## Usage

Run the tool using the following command:

```sh
cargo run -- <option>
```

### Options

* **create**: Create a namespace, list namespaces, create a table, list tables, and check the table.

* **insert**: Insert data using Athena.

* **query**: Query data using Athena.

* **llm**: Query with llm. Example: 

    ```sh
    cargo run -- llm "the five airlines with the most cancelled flights"
    ```

* **delete**: Delete the table and the namespace.

* **delete-table**: Delete the table only.

* **help**: Show all the options

### Example

```sh
cargo run -- query
```

This command will execute the `query_with_athena` function, which queries the data using Athena.

### Functions

* `create_namespace(client, table_bucket_arn)`: Create a namespace in the specified S3 bucket.

* `list_namespaces(client, table_bucket_arn)`: List all namespaces in the specified S3 bucket.

* `create_table(client, table_bucket_arn)`: Create a table in the specified namespace.

* `list_tables(client, table_bucket_arn)`: List all tables in the specified namespace.

* `check_table(client, table_bucket_arn)`: Check the status of the table.

* `insert_with_athena_handler(athena_client, table_bucket_arn)`: Insert data into the table using Athena.

* `query_with_athena(athena_client, table_bucket_arn)`: Query data from the table using Athena.

* `delete_table(client, table_bucket_arn)`: Delete the specified table.

* `delete_namespace(client, table_bucket_arn)`: Delete the specified namespace.

* `delete_table_bucket(client, table_bucket_arn)`: Delete the table only.

-------

Windows Command Prompt

```
setx OPENAI_API_KEY "REPLACE_WITH_YOUR_KEY_VALUE_HERE"
```

PowerShell
```
[System.Environment]::SetEnvironmentVariable('OPENAI_API_KEY', 'REPLACE_WITH_YOUR_KEY_VALUE_HERE', 'User')
```

### Logs
The tool uses env_logger to log information. The log level can be set using the RUST_LOG environment variable. By default, it is set to info.

### Contributions
Feel free to contribute to this project by creating issues or submitting pull requests.

### License
This project is licensed under the MIT License.