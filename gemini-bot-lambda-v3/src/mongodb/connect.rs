use lambda_runtime::tracing;
use mongodb::{
    bson::doc,
    Client,
    Collection
};
use crate::mongodb::CONNECTION_STRING;
use crate::libs::{MedicalData, MedicalDummie};
use std::env;

pub async fn mongodb_update(
    user_id: &str, 
    medical_info: &str,
    medical_result: MedicalDummie,
) -> mongodb::error::Result<()> {
    
    let user_name = env::var("MONGODB_USER_NAME")
        .expect("MONGODB_USER_NAME environment variable not set.");

    let db_password = env::var("MONGODB_PASSWORD")
        .expect("MONGODB_PASSWORD environment variable not set.");
    
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