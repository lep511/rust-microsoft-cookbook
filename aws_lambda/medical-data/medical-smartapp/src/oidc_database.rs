use aws_sdk_dynamodb as dynamodb;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SessionData {
    pub pk: String,
    pub authorized: bool,
    pub access_token: Option<String>,
    pub expires_in: Option<i32>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
    pub id_token: Option<String>,
    pub session_state: Option<String>,
    pub client_id: Option<String>,
    pub code_verifier: Option<String>,
    pub code_challenge: Option<String>,
    pub auth_endpoint: Option<String>,
    pub token_endpoint: Option<String>,
    pub iss: Option<String>,
    pub session_timeout: Option<i64>,
    pub patient: Option<String>,
}

impl SessionData {
    // Convert SessionData to DynamoDB item
    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        
        // Always include primary key
        item.insert("pk".to_string(), AttributeValue::S(self.pk.clone()));
        item.insert("authorized".to_string(), AttributeValue::Bool(self.authorized));
        
        // Add optional fields only if they exist
        if let Some(token) = &self.access_token {
            item.insert("access_token".to_string(), AttributeValue::S(token.clone()));
        }
        if let Some(exp) = self.expires_in {
            item.insert("expires_in".to_string(), AttributeValue::N(exp.to_string()));
        }
        if let Some(scope) = &self.scope {
            item.insert("scope".to_string(), AttributeValue::S(scope.clone()));
        }
        if let Some(token_type) = &self.token_type {
            item.insert("token_type".to_string(), AttributeValue::S(token_type.clone()));
        }
        if let Some(id_token) = &self.id_token {
            item.insert("id_token".to_string(), AttributeValue::S(id_token.clone()));
        }
        if let Some(session_state) = &self.session_state {
            item.insert("session_state".to_string(), AttributeValue::S(session_state.clone()));
        }
        if let Some(client_id) = &self.client_id {
            item.insert("client_id".to_string(), AttributeValue::S(client_id.clone()));
        }
        if let Some(code_verifier) = &self.code_verifier {
            item.insert("code_verifier".to_string(), AttributeValue::S(code_verifier.clone()));
        }
        if let Some(code_challenge) = &self.code_challenge {
            item.insert("code_challenge".to_string(), AttributeValue::S(code_challenge.clone()));
        }
        if let Some(auth_endpoint) = &self.auth_endpoint {
            item.insert("auth_endpoint".to_string(), AttributeValue::S(auth_endpoint.clone()));
        }
        if let Some(token_endpoint) = &self.token_endpoint {
            item.insert("token_endpoint".to_string(), AttributeValue::S(token_endpoint.clone()));
        }
        if let Some(iss) = &self.iss {
            item.insert("iss".to_string(), AttributeValue::S(iss.clone()));
        }
        if let Some(session_timeout) = self.session_timeout {
            item.insert("session_timeout".to_string(), AttributeValue::N(session_timeout.to_string()));
        }
        if let Some(patient) = &self.patient {
            item.insert("patient".to_string(), AttributeValue::S(patient.clone()));
        }
        item
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ClientData {
    pub client_id: String,
    pub state: String,
    pub authorized: bool,
    pub session_timeout: i64,
}

pub async fn get_session_data(
    pk: &str,
    table_name: &str,
) -> Result<Option<SessionData>, dynamodb::Error> {
    let config = aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);

    let result = client
        .get_item()
        .table_name(table_name)
        .key("pk", dynamodb::types::AttributeValue::S(pk.to_string()))
        .send()
        .await?;

    if let Some(item) = result.item() {
        let session_data = SessionData {
            pk: pk.to_string(),
            authorized: item.get("authorized").and_then(|av| av.as_bool().ok()).copied().unwrap_or(false),
            access_token: item.get("access_token").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            expires_in: item.get("expires_in").and_then(|av| av.as_n().ok().and_then(|n| n.parse::<i32>().ok())),
            scope: item.get("scope").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            token_type: item.get("token_type").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            id_token: item.get("id_token").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            session_state: item.get("session_state").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            client_id: item.get("client_id").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            code_verifier: item.get("code_verifier").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            code_challenge: item.get("code_challenge").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            auth_endpoint: item.get("auth_endpoint").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            token_endpoint: item.get("token_endpoint").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            iss: item.get("iss").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
            session_timeout: item.get("session_timeout").and_then(|av| av.as_n().ok().and_then(|n| n.parse::<i64>().ok())),
            patient: item.get("patient").and_then(|av| av.as_s().ok().map(|s| s.to_string())),
        };
        Ok(Some(session_data))
    } else {
        Ok(None)
    }
}

pub async fn get_client_data(
    pk_value: &str,
    table_name: &str,
    index_name: &str,
) -> Result<Option<ClientData>, dynamodb::Error> {
    let config = aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);
    let pk_name = "client_id";

    // Build query request
    let result = client
        .query()
        .table_name(table_name)
        .index_name(index_name)
        .key_condition_expression("#pk = :pk_val")
        .expression_attribute_names("#pk", pk_name)
        .expression_attribute_values(":pk_val", AttributeValue::S(pk_value.to_string()))
        .send()
        .await?;

    // Get the items from the result
    let items = result.items();
    
    // Check if there are any items
    if !items.is_empty() {
        // Get the first item
        if let Some(item) = items.first() {
            let client_data = ClientData {
                client_id: item.get("client_id").and_then(|av| av.as_s().ok().map(|s| s.to_string())).unwrap_or_default(),
                state: item.get("pk").and_then(|av| av.as_s().ok().map(|s| s.to_string())).unwrap_or_default(),
                authorized: item.get("authorized").and_then(|av| av.as_bool().ok()).copied().unwrap_or(false),
                session_timeout: item.get("session_timeout").and_then(|av| av.as_n().ok().and_then(|n| n.parse::<i64>().ok())).unwrap_or_default(),
            };
            return Ok(Some(client_data));
        }
    }
    
    Ok(None)
}

pub async fn remove_session_data(
    table_name: &str,
    key: &str,
    key_value: &str,
) -> Result<(), dynamodb::Error> {
    let config = aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);
    let key_av = dynamodb::types::AttributeValue::S(key_value.to_string());

    client
        .delete_item()
        .table_name(table_name)
        .key(key, key_av)
        .send()
        .await?;

    Ok(())
}

pub async fn save_to_dynamo(
    session_data: &SessionData,
    table_name: &str,
) -> Result<(), dynamodb::Error> {
    let config = aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);

    // Convert SessionData to DynamoDB item
    let item = session_data.to_item();

    // Send the PutItem request to DynamoDB
    client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .send()
        .await?;

    Ok(())
}