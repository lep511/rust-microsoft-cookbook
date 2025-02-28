use crate::oidc_request::get_mdata;
use crate::oidc_database::{SessionData, get_session_data};
use crate::http_page::get_main_page;
use serde_json::Value;

pub async fn main_console_page(
    state: &str,
    table_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {

    let session_data: SessionData = get_session_data(state, table_name)
        .await?
        .ok_or("Session data not found")?;

    let iss = session_data.iss.clone().unwrap_or_default();
    let param = "Patient?_elements=name";
    let access_token = match session_data.access_token {
        Some(token) => token.clone(),
        None => return Err("Access token not found".into()),
    };
    
    let patients: String = get_mdata(
        &iss,
        &param,
        &access_token,
    ).await?;

    // Convert to json
    let patients_json: Value = serde_json::from_str(&patients)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let html = get_main_page(
        &patients_json,
        &iss,
        &state,
    );

    Ok(html)
}