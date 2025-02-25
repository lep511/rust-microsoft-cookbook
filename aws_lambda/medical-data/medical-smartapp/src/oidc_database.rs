use aws_sdk_dynamodb as dynamodb;
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc, DateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionData {
    pub session_state: String,
    pub access_token: String,
    pub timestamp: String,
    pub expiry_timestamp: String,
    pub expires_at: i64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
}

pub async fn get_session_token(
    session_state: &str,
    table_name: &str,
) -> Result<String, dynamodb::Error> {
    let config = aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);

    let result = client
        .get_item()
        .table_name(table_name)
        .key("session_state", dynamodb::types::AttributeValue::S(session_state.to_string()))
        .send()
        .await?;

    if let Some(item) = result.item() {
        if let Some(access_token) = item.get("access_token") {
            if let dynamodb::types::AttributeValue::S(access_token) = access_token {
                return Ok(access_token.to_string());
            }
        }
    }

    Ok("nan".to_string())
}

// pub async fn save_session_token(
//     session_state: &str,
//     access_token: &str,
//     expires_at: i64,
//     scope: &str,
//     token_type: &str,
//     refresh_token: &str,
//     table_name: &str,
// ) -> Result<(), dynamodb::Error> {
//     let config = aws_config::load_from_env().await;
//     let client = dynamodb::Client::new(&config);

//     // Get current UTC time
//     let now = Utc::now();
    
//     // Set timestamp RFC 3339 format
//     let timestamp = now.to_rfc3339();





//     let item = dynamodb::types::AttributeValue::M(
//         [
//             (
//                 "session_state".to_string(),
//                 dynamodb::types::AttributeValue::S(session_data.session_state.clone()),
//             ),
//             (
//                 "access_token".to_string(),
//                 dynamodb::types::AttributeValue::S(session_data.access_token.clone()),
//             ),
//             (
//                 "timestamp".to_string(),
//                 dynamodb::types::AttributeValue::S(session_data.timestamp.clone()),
//             ),
//             (
//                 "expiry_timestamp".to_string(),
//                 dynamodb::types::AttributeValue::S(session_data.expiry_timestamp.clone()),
//             ),
//             (
//                 "expires_at".to_string(),
//                 dynamodb::types::AttributeValue::N(session_data.expires_at.to_string()),
//             ),
//             (
//                 "last_updated".to_string(),
//                 dynamodb::types::AttributeValue::S(
//                     chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
//                 ),
//             ),
//         ]
//         .into_iter()
//         .collect(),
//     );

//     client
//         .put_item()
//         .table_name(table_name)
//         .item("session_state", item)
//         .send()
//         .await?;

//     Ok(())
// }

