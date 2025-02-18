use lambda_http::{Body, Error, Request, RequestExt, Response};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_dsql::auth_token::{AuthTokenGenerator, Config};
use bigdecimal::BigDecimal;
use std::str::FromStr;
use rand::Rng;
use sqlx::Row;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use uuid::Uuid;
use std::env;

pub async fn connect_dsql(cluster_endpoint: String) -> anyhow::Result<()> {
    let region = "us-east-1";

    // Generate auth token
    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let signer = AuthTokenGenerator::new(
        Config::builder()
            .hostname(&cluster_endpoint)
            .region(Region::new(region))
            .build()
            .unwrap(),
    );
    let password_token = signer.db_connect_admin_auth_token(&sdk_config).await.unwrap();
    
    // Setup connections
    let connection_options = PgConnectOptions::new()
        .host(cluster_endpoint.as_str())
        .port(5432)
        .database("postgres")
        .username("admin")
        .password(password_token.as_str())
        .ssl_mode(sqlx::postgres::PgSslMode::VerifyFull);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(connection_options.clone())
        .await?;

    // Create table
    // To avoid Optimistic concurrency control (OCC) conflicts
    // Have this table created already.
    // sqlx::query(
    //     "CREATE TABLE IF NOT EXISTS employees (
	// 		id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    //         first_name VARCHAR(50) NOT NULL,
    //         last_name VARCHAR(50) NOT NULL,
    //         email VARCHAR(100) UNIQUE,
    //         birth_date DATE,
    //         hire_date DATE DEFAULT CURRENT_DATE,
    //         salary DECIMAL(10,2),
    //         department VARCHAR(100),
    //         is_active BOOLEAN DEFAULT true,
    //         notes TEXT,
    //         created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	// 	)").execute(&pool).await?;    
    
    // Insert some data
    let id = Uuid::new_v4();
    let telephone = rand::rng()
        .random_range(123456..987654)
        .to_string();
    
    let salary = BigDecimal::from_str("123.45").unwrap(); 
    
    let result = sqlx::query("INSERT INTO employees (id, first_name, last_name, email, birth_date, hire_date, \
                            salary, department, is_active, notes, created_at) VALUES ($1, $2, $3, $4, $5::DATE, \
                            $6::DATE, $7, $8, $9, $10, $11::DATE)")
        .bind(id)
        .bind("John")
        .bind("Doe")
        .bind("john.doe@example.com")
        .bind("2023-01-01 00:00:00")
        .bind("2023-01-01 00:00:00")
        .bind(salary)
        .bind("IT")
        .bind(true)
        .bind("Some notes")
        .bind("2023-01-01 00:00:00")
        .execute(&pool)
        .await;

    match result {
        Ok(_) => println!("Data inserted successfully"),
        Err(err) => match err {
            sqlx::Error::RowNotFound => println!("Error row not fund"),
            sqlx::Error::Database(err)
                if err.constraint() == Some("username_key") =>
            {
                println!("Username already exists: {}", err.message());
            }
            _ => println!("Error inserting data: {}", err),
        }
    }

    // Read data back
    let rows = sqlx::query("SELECT * FROM employees").fetch_all(&pool).await?;
    println!("{:?}", rows);

    pool.close().await;
    Ok(())
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    let cluster_endpoint = match env::var("CLUSTER_ENDPOINT") {
        Ok(val) => val,
        Err(_e) => {
            let resp = Response::builder()
                .status(404)
                .header("content-type", "text/html")
                .body("CLUSTER_ENDPOINT not set in environment.".into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    match connect_dsql(cluster_endpoint).await {
        Ok(_) => {
            println!("Connected to DSQL");
        },
        Err(e) => {
            let resp = Response::builder()
                .status(500)
                .header("content-type", "text/html")
                .body(format!("Error connecting to DSQL: {}", e).into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    }

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}