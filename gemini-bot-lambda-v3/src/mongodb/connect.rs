use log::info;
use mongodb::{
    bson::doc,
    Client,
    Collection
};
use crate::mongodb::CONNECTION_STRING;
use crate::libs::{MedicalData, MedicalTerms};
use std::env;

pub async fn mongodb_update(
    user_id: &str, 
    medical_info: &str, 
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
        medical_terms: vec![
            MedicalTerms {
                code_type: "ICD-10".to_string(),
                code_value: "A00".to_string(),
                code_explain: "Cholera".to_string(),
            },
            MedicalTerms {
                code_type: "ICD-10".to_string(),
                code_value: "B00".to_string(),
                code_explain: "Herpesviral [herpes simplex] infections".to_string(),
            },
        ],
    };
    
    let res = my_coll.insert_one(medical_data).await?;
    info!("Inserted a document with _id: {}", res.inserted_id);

    Ok(())
}