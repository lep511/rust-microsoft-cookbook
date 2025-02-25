use aws_sdk_dynamodb as dynamodb;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub user_id: String,
    pub ehr_id: String,
    pub access_token: String,
    pub timestamp: String,
    pub expiry_timestamp: String,
    pub expires_at: i64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
    pub last_updated: Option<String>,
}

pub async fn get_db_data(
    client: &dynamodb::Client,
    table_name: &str,
    user_id: &str,
) -> Result<Option<String>, dynamodb::Error> {
    let result = client
        .get_item()
        .table_name(table_name)
        .key("user_name", dynamodb::types::AttributeValue::S(user_id.to_string()))
        .send()
        .await?;

    if let Some(item) = result.item() {
        if let Some(user_id) = item.get("user_id") {
            if let dynamodb::types::AttributeValue::S(user_id) = user_id {
                return Ok(Some(user_id.to_string()));
            }
        }
    }

    Ok(None)
}

