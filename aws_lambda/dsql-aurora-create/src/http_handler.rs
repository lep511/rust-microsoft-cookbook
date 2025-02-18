use lambda_http::{Body, Error, Request, RequestExt, Response};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_dsql::auth_token::{AuthTokenGenerator, Config};
use rand::Rng;
use sqlx::Row;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use uuid::Uuid;
use std::env;

async fn example(cluster_endpoint: String, region: String) -> anyhow::Result<()> {
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

    // Create owners table
    // To avoid Optimistic concurrency control (OCC) conflicts
    // Have this table created already.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS owner (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			name VARCHAR(255),
			city VARCHAR(255),
			telephone VARCHAR(255)
		)").execute(&pool).await?;    
    
    // Insert some data
    let id = Uuid::new_v4();
    let telephone = rand::thread_rng()
        .gen_range(123456..987654)
        .to_string();
    let result = sqlx::query("INSERT INTO owner (id, name, city, telephone) VALUES ($1, $2, $3, $4)")
        .bind(id)
        .bind("John Doe")
        .bind("Anytown")
        .bind(telephone.as_str())
        .execute(&pool)
        .await?;
    assert_eq!(result.rows_affected(), 1);

    // Read data back
    let rows = sqlx::query("SELECT * FROM owner WHERE id=$1").bind(id).fetch_all(&pool).await?;
    println!("{:?}", rows);

    assert_eq!(rows.len(), 1);
    let row = &rows[0];
    assert_eq!(row.try_get::<&str, _>("name")?, "John Doe");
    assert_eq!(row.try_get::<&str, _>("city")?, "Anytown");
    assert_eq!(row.try_get::<&str, _>("telephone")?, telephone);

    // sqlx::query("DELETE FROM owner WHERE name='John Doe'")
    //     .execute(&pool).await?;

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

    let cluster_endpoint = env::var("CLUSTER_ENDPOINT").expect("CLUSTER_ENDPOINT required");
    let region = env::var("REGION").expect("REGION of aws required");
    let _ = example(cluster_endpoint, region).await?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;

    Ok(resp)
}