use aws_sdk_dynamodb as dynamodb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionData {
    pub session_state: String,
    pub access_token: Option<String>,
    pub expires_in: Option<i32>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
    pub id_token: Option<String>,
    pub iss: Option<String>,
}

pub async fn get_session_data(
    session_state: &str,
    table_name: &str,
) -> Result<Option<SessionData>, dynamodb::Error> {
    let config = aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);

    let result = client
        .get_item()
        .table_name(table_name)
        .key("session_state", dynamodb::types::AttributeValue::S(session_state.to_string()))
        .send()
        .await?;

    if let Some(item) = result.item() {
        let session_data = SessionData {
            session_state: session_state.to_string(),
            access_token: item.get("access_token").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            expires_in: item.get("expires_in").and_then(|av| av.as_n().ok().and_then(|n| n.parse::<i32>().ok())),
            scope: item.get("scope").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            token_type: item.get("token_type").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            id_token: item.get("id_token").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            iss: item.get("iss").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
        };
        Ok(Some(session_data))
    } else {
        Ok(None)
    }
}

pub async fn save_to_dynamo(
    session_data: &SessionData,
    table_name: &str,
) -> Result<(), dynamodb::Error> {
    let config = aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);

    let session_state = dynamodb::types::AttributeValue::S(session_data.session_state.clone());
    let access_token = match &session_data.access_token {
        Some(access_token) => dynamodb::types::AttributeValue::S(access_token.clone()),
        None => dynamodb::types::AttributeValue::S("".to_string()),
    };
    let expires_in = match session_data.expires_in {
        Some(expires_in) => dynamodb::types::AttributeValue::N(expires_in.to_string()),
        None => dynamodb::types::AttributeValue::S("".to_string()),
    };
    let scope = match &session_data.scope {
        Some(scope) => dynamodb::types::AttributeValue::S(scope.clone()),
        None => dynamodb::types::AttributeValue::S("".to_string()),
    };
    let token_type = match &session_data.token_type {
        Some(token_type) => dynamodb::types::AttributeValue::S(token_type.clone()),
        None => dynamodb::types::AttributeValue::S("".to_string()),
    };
    let id_token = match &session_data.id_token {
        Some(id_token) => dynamodb::types::AttributeValue::S(id_token.clone()),
        None => dynamodb::types::AttributeValue::S("".to_string()),
    };
    let iss = match &session_data.iss {
        Some(iss) => dynamodb::types::AttributeValue::S(iss.clone()),
        None => dynamodb::types::AttributeValue::S("".to_string()),
    };

    let mut item = HashMap::new();
    
    // Use the session_state as the primary key
    item.insert("session_state".to_string(), session_state);
    item.insert("access_token".to_string(), access_token);
    item.insert("expires_in".to_string(), expires_in);
    item.insert("scope".to_string(), scope);
    item.insert("token_type".to_string(), token_type);
    item.insert("id_token".to_string(), id_token);
    item.insert("iss".to_string(), iss);

    // Send the PutItem request to DynamoDB
    client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .send()
        .await?;

    Ok(())
}

