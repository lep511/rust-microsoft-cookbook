use aws_config::{BehaviorVersion, Region};
use aws_sdk_dsql::auth_token::{AuthTokenGenerator, Config};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
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
        "CREATE TABLE IF NOT EXISTS flights (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            airport_code VARCHAR(255),
            count INTEGER
		)").execute(&pool).await?;    
    
    // Read data back
    let rows = sqlx::query("SELECT airport_code, count FROM flights WHERE airport_code=$1").bind("PHX").fetch_all(&pool).await?;
    println!("{:?}", rows);

    pool.close().await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cluster_endpoint = env::var("CLUSTER_ENDPOINT").expect("CLUSTER_ENDPOINT required");
    let region = env::var("REGION").expect("REGION of aws required");
    Ok(example(cluster_endpoint, region).await?)
}

#[cfg(test)]
mod tests {

    use super::*;
    use tokio::test;

    #[test]
    async fn test_crud() {
        let cluster_endpoint = env::var("CLUSTER_ENDPOINT").expect("CLUSTER_ENDPOINT required");
        let region = env::var("REGION").expect("REGION of aws required");
        let result = example(cluster_endpoint, region).await;
        assert!(result.is_ok());
        println!("Successfully completed test run.");
    }
}