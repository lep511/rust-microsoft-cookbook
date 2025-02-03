use lambda_runtime::tracing;
use mongodb::{
    bson::doc,
    Client,
    Collection
};
use mongodb::error::Error as MongoError;
use crate::mongodb::CONNECTION_STRING;
use crate::libs::{MedicalData, MedicalDummie};
use aws_sdk_secretsmanager::{Client as SemClient, Error};
use serde::{Serialize, Deserialize};
use std::{fmt, env};

#[derive(Debug)]
pub enum CustomError {
    MongoError(MongoError),
    CredentialError(String),
}
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::MongoError(err) => write!(f, "MongoDB error: {}", err),
            CustomError::CredentialError(err) => write!(f, "Credential error: {}", err),
        }
    }
}

impl From<MongoError> for CustomError {
    fn from(err: MongoError) -> CustomError {
        CustomError::MongoError(err)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MongoConnection {
    pub mongodb_username: String,
    pub mongodb_password: String,
}

async fn get_secret() -> Result<String, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = SemClient::new(&shared_config);
    
    let secret_name = env::var("SECRET_NAME")
        .expect("SECRET_NAME environment variable not set.");
    
    let secret = client
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;

    match secret.secret_string() {
        Some(secret) => Ok(secret.to_string()),
        None => Ok("not secret found".to_string()),
    }
}

pub async fn mongodb_update(
    user_id: &str, 
    medical_info: &str,
    medical_result: MedicalDummie,
) -> Result<(), CustomError> {
    
    let secret = match get_secret().await {
        Ok(secret) => secret,
        Err(e) => {
            tracing::error!("Error getting credentials: {}", e);
            return Err(CustomError::CredentialError(e.to_string()));
        }
    };

    let credentials:MongoConnection = match serde_json::from_str(&secret) {
        Ok(credentials) => credentials,
        Err(e) => {
            tracing::error!("Error converting secret-string to json: {}", e);
            return Err(CustomError::CredentialError(e.to_string()));
        }
    };

    let user_name = credentials.mongodb_username;
    let db_password = credentials.mongodb_password;
   
    let uri = format!(
        "mongodb+srv://{}:{}@{}",
        user_name,
        db_password,
        CONNECTION_STRING,
    );

    let client = Client::with_uri_str(uri).await?;

    let my_coll: Collection<MedicalData> = client
        .database("medical")
        .collection("medical_data");

    let medical_data = MedicalData {
        user_id: user_id.to_string(),
        medical_info: medical_info.to_string(),
        medical_terms: medical_result.medical_terms,
    };
    
    let res = my_coll.insert_one(medical_data).await?;
    tracing::info!("Inserted a document with _id: {}", res.inserted_id);

    Ok(())
}