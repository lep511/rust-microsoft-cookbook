use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use lazy_static::lazy_static;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Mutex;
use url::Url;

// Configuration constants
// Replace "your_client_id" with the actual client ID obtained from MeldRx
const CLIENT_ID: &str = "your_client_id";
const REDIRECT_URI: &str = "http://localhost:8080/callback";

// Global variables to store the access token, SMART configuration, and issuer (iss)
// Using lazy_static and Mutex for thread-safe global state
lazy_static! {
    static ref ACCESS_TOKEN: Mutex<Option<String>> = Mutex::new(None);
    static ref SMART_CONFIG: Mutex<Option<Value>> = Mutex::new(None);
    static ref ISS: Mutex<Option<String>> = Mutex::new(None);
}

// Structs to deserialize query parameters
#[derive(Deserialize)]
struct LaunchQuery {
    iss: String,
    launch: String,
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}

// Fetch SMART on FHIR configuration from the issuer (iss)
async fn fetch_smart_config(iss: &str) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/.well-known/smart-configuration", iss);
    let response = client.get(&url).send().await?.json().await?;
    Ok(response)
}

// Build the authorization URL with necessary query parameters
fn build_auth_url(config: &Value, launch: &str) -> String {
    let auth_endpoint = config["authorization_endpoint"].as_str().unwrap();
    let mut url = Url::parse(auth_endpoint).unwrap();
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", CLIENT_ID)
        .append_pair("scope", "launch patient/*.*")
        .append_pair("redirect_uri", REDIRECT_URI)
        .append_pair("launch", launch);
    url.to_string()
}

// Exchange authorization code for access token
async fn exchange_code_for_token(code: &str) -> Result<Value, reqwest::Error> {
    // Retrieve the SMART configuration
    let smart_config = SMART_CONFIG.lock().unwrap();
    let config = smart_config.as_ref().unwrap();
    let token_endpoint = config["token_endpoint"].as_str().unwrap();

    let client = Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", REDIRECT_URI),
        ("client_id", CLIENT_ID),
    ];

    let response = client
        .post(token_endpoint)
        .form(&params)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

// Handler for the /launch route
async fn launch_handler(query: web::Query<LaunchQuery>) -> impl Responder {
    let iss = &query.iss;
    let launch = &query.launch;

    // Store the issuer (iss) for later use in FHIR API requests
    let mut iss_store = ISS.lock().unwrap();
    *iss_store = Some(iss.clone());

    // Fetch SMART configuration
    let config = fetch_smart_config(iss).await;

    match config {
        Ok(config) => {
            // Store the SMART configuration
            let mut smart_config = SMART_CONFIG.lock().unwrap();
            *smart_config = Some(config.clone());

            // Build and redirect to the authorization URL
            let auth_url = build_auth_url(&config, launch);
            HttpResponse::Found()
                .header("Location", auth_url)
                .finish()
        }
        Err(_) => HttpResponse::InternalServerError()
            .body("Failed to fetch SMART configuration"),
    }
}

// Handler for the /callback route
async fn callback_handler(query: web::Query<CallbackQuery>) -> impl Responder {
    let code = &query.code;

    // Exchange the authorization code for an access token
    let token_response = exchange_code_for_token(code).await;

    match token_response {
        Ok(token_response) => {
            // Store the access token
            let mut access_token = ACCESS_TOKEN.lock().unwrap();
            *access_token = Some(
                token_response["access_token"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            );

            HttpResponse::Ok().body("Authorization successful")
        }
        Err(_) => HttpResponse::InternalServerError()
            .body("Failed to exchange code for token"),
    }
}

// Handler for the /patient route to fetch patient data using the access token
async fn patient_handler() -> impl Responder {
    let access_token = ACCESS_TOKEN.lock().unwrap();
    if let Some(token) = access_token.as_ref() {
        // Retrieve the issuer (iss) for constructing the FHIR API URL
        let iss_store = ISS.lock().unwrap();
        let iss = iss_store.as_ref().unwrap();
        let client = Client::new();
        let url = format!("{}/Patient", iss);

        // Make authenticated request to fetch patient data
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;

        match response {
            Ok(response) => {
                let text = response.text().await.unwrap();
                HttpResponse::Ok().body(text)
            }
            Err(_) => HttpResponse::InternalServerError()
                .body("Failed to fetch patient data"),
        }
    } else {
        HttpResponse::Unauthorized().body("No access token")
    }
}

// Main function to set up and run the Actix Web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/launch", web::get().to(launch_handler))
            .route("/callback", web::get().to(callback_handler))
            .route("/patient", web::get().to(patient_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}