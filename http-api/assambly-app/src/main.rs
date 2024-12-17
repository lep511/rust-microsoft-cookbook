use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;
use std::env;

// const FILE_URL: &str = "https://assembly.ai/wildfires.mp3";
// const FILE_URL: &str = "https://github.com/rafaelreis-hotmart/Audio-Sample-files/raw/refs/heads/master/sample.ogg";
const FILE_URL: &str = "https://github.com/rafaelreis-hotmart/Audio-Sample-files/raw/refs/heads/master/sample.m4a";
const TRANSCRIPT_ENDPOINT: &str = "https://api.assemblyai.com/v2/transcript";


#[derive(Serialize)]
struct TranscriptRequest {
    audio_url: String,
}

#[derive(Deserialize)]
struct TranscriptResponse {
    id: String,
    status: String,
    text: Option<String>,
    error: Option<String>,
}

fn main() {
    let your_api_key = env::var("ASSEMBLYAI_API_KEY").expect("ASSEMBLYAI_API_KEY not set");
    let assemblyai_key = your_api_key.as_str();
    let request_data = TranscriptRequest {
        audio_url: FILE_URL.to_string(),
    };

    let client = Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "authorization",
                reqwest::header::HeaderValue::from_str(assemblyai_key).unwrap(),
            );
            headers.insert(
                "content-type",
                reqwest::header::HeaderValue::from_static("application/json"),
            );
            headers
        })
        .build()
        .unwrap();

    let response: Response = client
        .post(TRANSCRIPT_ENDPOINT)
        .json(&request_data)
        .send()
        .unwrap();

    let response_json: TranscriptResponse = response.json().unwrap();
    let transcript_id = response_json.id;

    let polling_endpoint = format!("{}/{}", TRANSCRIPT_ENDPOINT, transcript_id);
    println!("Proccessing file: {}", FILE_URL);
    
    loop {
        let polling_response: Response = client
            .get(&polling_endpoint)
            .send()
            .unwrap();

        let transcription_result: TranscriptResponse = polling_response.json().unwrap();

        match transcription_result.status.as_str() {
            "completed" => {
                println!("{}", transcription_result.text.unwrap());
                break;
            }
            "error" => {
                panic!("Transcription failed: {}", transcription_result.error.unwrap());
            }
            _ => {
                thread::sleep(Duration::from_secs(3));
            }
        }
    }
}