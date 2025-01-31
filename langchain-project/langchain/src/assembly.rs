use reqwest::Client;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::llmerror::AssemblyAIError;
use std::collections::HashMap;
use std::env;
use log::error;

pub static ASSEMBLYAI_BASE_URL: &str = "https://api.assemblyai.com/v2";
static SPEECH_ACCEPT_MODEL: [&str; 2] = ["best", "nano"];

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct TranscriptRequest {
    pub audio_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_end_at: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_start_from: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_chapters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_highlights: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_safety: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_spelling: Option<Vec<CustomSpelling>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_topics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disfluencies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_detection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_profanity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iab_categories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_confidence_threshold: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_detection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multichannel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punctuate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_pii: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_pii_audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_pii_audio_quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_pii_policies: Option<Vec<PiiType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_pii_sub: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_analysis: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_labels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speakers_expected: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_threshold: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summarization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_auth_header_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_auth_header_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_boost: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PiiType {
    #[serde(rename = "account_number")]
    AccountNumber,
    #[serde(rename = "banking_information")] 
    BankingInformation,
    #[serde(rename = "blood_type")]
    BloodType,
    #[serde(rename = "credit_card_cvv")]
    CreditCardCvv,
    #[serde(rename = "credit_card_expiration")]
    CreditCardExpiration,
    #[serde(rename = "credit_card_number")] 
    CreditCardNumber,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "date_of_birth")]
    DateOfBirth,
    #[serde(rename = "drivers_license")]
    DriversLicense, 
    #[serde(rename = "drug")]
    Drug,
    #[serde(rename = "email_address")]
    EmailAddress,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "gender_sexuality")]
    GenderSexuality,
    #[serde(rename = "healthcare_number")]
    HealthcareNumber,
    #[serde(rename = "injury")]
    Injury,
    #[serde(rename = "ip_address")]
    IpAddress,
    #[serde(rename = "language")]
    Language,
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "medical_condition")]
    MedicalCondition,
    #[serde(rename = "medical_process")]
    MedicalProcess,
    #[serde(rename = "money_amount")]
    MoneyAmount,
    #[serde(rename = "nationality")]
    Nationality,
    #[serde(rename = "number_sequence")]
    NumberSequence,
    #[serde(rename = "occupation")]
    Occupation,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "passport_number")]
    PassportNumber,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "person_age")]
    PersonAge,
    #[serde(rename = "person_name")]
    PersonName,
    #[serde(rename = "phone_number")]
    PhoneNumber,
    #[serde(rename = "political_affiliation")]
    PoliticalAffiliation,
    #[serde(rename = "religion")]
    Religion,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "us_social_security_number")]
    UsSocialSecurityNumber,
    #[serde(rename = "username")]
    Username,
    #[serde(rename = "vehicle_id")]
    VehicleId,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ListTranscriptParameters {
    pub limit: Option<i32>,
    pub status: Option<String>,
    pub created_on: Option<String>,
    pub before_id: Option<String>,
    pub after_id: Option<String>,
    pub throttled_only: Option<bool>,
}

pub trait PrintDebug {
    fn print_pre(request: &impl serde::Serialize, active: bool) {
        if !active {
            println!();
        } else {
            match serde_json::to_string_pretty(request) {
                Ok(json) => println!("Pretty-printed JSON:\n{}", json),
                Err(e) => error!("Error {:?}", e)
            }
        }
    }
}

pub trait GetApiKey {
    fn get_api_key() -> Result<String, AssemblyAIError> {
        match env::var("ASSEMBLYAI_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                error!("Error ASSEMBLYAI_API_KEY not found in environment variables");
                Err(AssemblyAIError::ApiKeyNotFound)
            }
            Err(e) => {
                error!("Error {:?}", e);
                Err(AssemblyAIError::EnvError(e))
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TranscriptAssemblyAI {
    pub api_key: String,
    pub request: TranscriptRequest,
    pub timeout: u64,
}

#[allow(dead_code)]
impl TranscriptAssemblyAI {
    pub fn new() -> Result<Self, AssemblyAIError> {
        let api_key = Self::get_api_key()?;

        let speech_model = Some("best".to_string()); // Or nano
        let audio_url = "https://assembly.ai/wildfires.mp3";

        let request = TranscriptRequest {
            audio_url: audio_url.to_string(),
            speech_model: speech_model,
            audio_end_at: None,
            audio_start_from: None,
            auto_chapters: None,
            auto_highlights: None,
            boost_param: None,
            content_safety: None,
            custom_spelling: None,
            custom_topics: None,
            disfluencies: None,
            entity_detection: None,
            filter_profanity: None,
            format_text: None,
            iab_categories: None,
            language_code: None,
            language_confidence_threshold: None,
            language_detection: None,
            multichannel: None,
            punctuate: None,
            redact_pii: None,
            redact_pii_audio: None,
            redact_pii_audio_quality: None,
            redact_pii_policies: None,
            redact_pii_sub: None,
            sentiment_analysis: None,
            speaker_labels: None,
            speakers_expected: None,
            speech_threshold: None,
            summarization: None,
            summary_model: None,
            summary_type: None,
            topics: None,
            webhook_auth_header_name: None,
            webhook_auth_header_value: None,
            webhook_url: None,
            word_boost: None,
        };
        
        Ok(Self {
            api_key: api_key,
            request: request,
            timeout: 15 * 60, // default: 15 minutes
        })
    }

    pub async fn upload_file(
        self,
        file_path: &str,
    ) -> Result<String, AssemblyAIError> {
        let base_url = format!("{}/upload", ASSEMBLYAI_BASE_URL);

        let client = Client::builder()
            .use_rustls_tls()
            .build()?;

        let file = match std::fs::read(file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(AssemblyAIError::FileReadError);
            }
        };

        let response = client
            .post(base_url)
            .timeout(Duration::from_secs(self.timeout))
            .header("Authorization", self.api_key.clone())
            .body(file)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Self::print_pre(&response, DEBUG_POST);

        let response_string = response.to_string();
        let mut response_map: HashMap<String, String> = match serde_json::from_str(&response_string) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error UploadFileResponse: {:?}", e);
                return Err(AssemblyAIError::ResponseContentError);
            }
        };

        let upload_url = response_map.remove("upload_url").unwrap();

        Ok(upload_url)
    }

    pub async fn transcript(
        mut self,
        audio_url: &str,
    ) -> Result<TranscriptResponse, AssemblyAIError> {

        let base_url = format!("{}/transcript", ASSEMBLYAI_BASE_URL);

        let client = Client::builder()
            .use_rustls_tls()
            .build()?;
        
        self.request.audio_url = audio_url.to_string();

        Self::print_pre(&self.request, DEBUG_PRE);

        let response = client
            .post(base_url)
            .timeout(Duration::from_secs(self.timeout))
            .header("Authorization", self.api_key.clone())
            .header("Content-Type", "application/json")
            .json(&self.request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Self::print_pre(&response, DEBUG_POST);
        
        let response_string = response.to_string();
        let transcript_response: TranscriptResponse = match serde_json::from_str(&response_string) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error TranscriptResponse: {:?}", e);
                return Err(AssemblyAIError::ResponseContentError);
            }
        };

        Ok(transcript_response)
    }

    pub async fn get_transcript(
        self, 
        id: &str
    ) -> Result<GetTranscriptResponse, AssemblyAIError> {
        let base_url = format!("{}/transcript/{}", ASSEMBLYAI_BASE_URL, id);
        let client = Client::builder()
            .use_rustls_tls()
            .build()?;

        let response = client
            .get(base_url)
            .timeout(Duration::from_secs(self.timeout))
            .header("Authorization", self.api_key.clone())
            .json(&self.request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        
        Self::print_pre(&response, DEBUG_POST);

        let response_string = response.to_string();
        
        let transcript_response: GetTranscriptResponse = match serde_json::from_str(&response_string) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error GetTranscriptResponse: {:?}", e);
                return Err(AssemblyAIError::ResponseContentError);
            }
        };

        Ok(transcript_response)
    }

    pub async fn list_transcripts(
        self,
        parameters: Option<ListTranscriptParameters>,
    ) -> Result<ListTranscriptResponse, AssemblyAIError> {
        let mut param_data = vec![];

        if let Some(params) = parameters {
            if let Some(limit) = params.limit {
                let param_limit = ("limit", limit.to_string());
                param_data.push(param_limit);
            }

            if let Some(status) = params.status {
                let param_status = ("status", status);
                param_data.push(param_status);
            }

            if let Some(created_on) = params.created_on {
                let param_created_on = ("created_on", created_on);
                param_data.push(param_created_on);
            }

            if let Some(before_id) = params.before_id {
                let param_before_id = ("before_id", before_id);
                param_data.push(param_before_id);
            }

            if let Some(after_id) = params.after_id {
                let param_after_id = ("after_id", after_id);
                param_data.push(param_after_id);
            }

            if let Some(throttled_only) = params.throttled_only {
                let param_throttled_only = ("throttled_only", throttled_only.to_string());
                param_data.push(param_throttled_only);
            }           
        }

        let client = Client::builder()
            .use_rustls_tls()
            .build()?;
        
        let base_url = format!("{}/transcript", ASSEMBLYAI_BASE_URL);
        let base_url = match Url::parse_with_params(&base_url, &param_data) {
            Ok(url) => url,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(AssemblyAIError::ResponseContentError);
            }
        };
        
        let response = client
            .get(base_url)
            .timeout(Duration::from_secs(self.timeout))
            .header("Authorization", self.api_key.clone())
            .json(&self.request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Self::print_pre(&response, DEBUG_POST);

        let response_string = response.to_string();

        let transcript_response: ListTranscriptResponse = match serde_json::from_str(&response_string) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error ListTranscriptResponse: {:?}", e);
                return Err(AssemblyAIError::ResponseContentError);
            }
        };

        Ok(transcript_response)
    }

    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_language_detection(mut self, language_detection: bool) -> Self {
        self.request.language_detection = Some(language_detection);
        self
    }

    pub fn with_speech_model(mut self, speech_model: &str) -> Self {
        if !SPEECH_ACCEPT_MODEL.contains(&speech_model) {
            error!("Error Speech model not accepted");
            return self;
        }
        self.request.speech_model = Some(speech_model.to_string());
        self
    }

    pub fn with_redact_pii_policies(mut self, redact_pii_policies: Vec<PiiType>) -> Self {
        self.request.redact_pii_policies = Some(redact_pii_policies);
        self
    }

    pub fn with_audio_end_at(mut self, audio_end_at: u32) -> Self {
        self.request.audio_end_at = Some(audio_end_at);
        self
    }

    pub fn with_language_code(mut self, language_code: &str) -> Self {
        self.request.language_code = Some(language_code.to_string());
        self
    }
}

impl GetApiKey for TranscriptAssemblyAI {}
impl PrintDebug for TranscriptAssemblyAI {}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TranscriptResponse {
    pub acoustic_model: Option<String>,
    pub audio_duration: Option<u32>,
    pub audio_end_at: Option<u32>,
    pub audio_start_from: Option<u32>,
    pub audio_url: Option<String>,
    pub auto_chapters: Option<bool>,
    pub auto_highlights: Option<bool>,
    pub auto_highlights_result: Option<serde_json::Value>,
    pub boost_param: Option<f64>,
    pub chapters: Option<serde_json::Value>,
    pub confidence: Option<f64>,
    pub content_safety: Option<bool>,
    pub content_safety_labels: Option<HashMap<String, serde_json::Value>>,
    pub custom_spelling: Option<serde_json::Value>,
    pub custom_topics: Option<serde_json::Value>,
    pub custom_topics_results: Option<serde_json::Value>,
    pub disfluencies: Option<bool>,
    pub dual_channel: Option<bool>,
    pub entities: Option<serde_json::Value>,
    pub entity_detection: Option<bool>,
    pub filter_profanity: Option<bool>,
    pub format_text: Option<bool>,
    pub iab_categories: Option<bool>,
    pub iab_categories_result: Option<HashMap<String, serde_json::Value>>,
    pub id: Option<String>,
    pub is_deleted: Option<bool>,
    pub language_code: Option<String>,
    pub language_confidence: Option<f64>,
    pub language_confidence_threshold: Option<f64>,
    pub language_detection: Option<bool>,
    pub language_model: Option<String>,
    pub multichannel: Option<bool>,
    pub punctuate: Option<bool>,
    pub redact_pii: Option<bool>,
    pub redact_pii_audio: Option<bool>,
    pub redact_pii_audio_quality: Option<String>,
    pub redact_pii_policies: Option<Vec<PiiType>>,
    pub redact_pii_sub: Option<String>,
    pub sentiment_analysis: Option<bool>,
    pub sentiment_analysis_results: Option<serde_json::Value>,
    pub speaker_labels: Option<bool>,
    pub speakers_expected: Option<i32>,
    pub speech_model: Option<String>,
    pub speech_threshold: Option<f64>,
    pub speed_boost: Option<bool>,
    pub status: Option<String>,
    pub summarization: Option<bool>,
    pub summary: Option<String>,
    pub summary_model: Option<String>,
    pub summary_type: Option<String>,
    pub text: Option<String>,
    pub throttled: Option<bool>,
    pub topics: Option<Vec<String>>,
    pub utterances: Option<serde_json::Value>,
    pub webhook_auth: Option<bool>,
    pub webhook_auth_header_name: Option<String>,
    pub webhook_status_code: Option<i32>,
    pub webhook_url: Option<String>,
    pub word_boost: Option<Vec<String>>,
    pub words: Option<serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetTranscriptResponse {
    pub id: Option<String>,
    pub audio_url: Option<String>,
    pub status: Option<String>,
    pub webhook_auth: Option<bool>,
    pub auto_highlights: Option<bool>,
    pub redact_pii: Option<bool>,
    pub summarization: Option<bool>,
    pub language_model: Option<String>,
    pub acoustic_model: Option<String>,
    pub language_code: Option<String>,
    pub language_detection: Option<bool>,
    pub language_confidence_threshold: Option<f64>,
    pub language_confidence: Option<f64>,
    pub speech_model: Option<String>,
    pub text: Option<String>,
    pub words: Option<Vec<Word>>,
    pub utterances: Option<Vec<Utterance>>,
    pub confidence: Option<f64>,
    pub audio_duration: Option<u32>,
    pub punctuate: Option<bool>,
    pub format_text: Option<bool>,
    pub disfluencies: Option<bool>,
    pub multichannel: Option<bool>,
    pub audio_channels: Option<u32>,
    pub webhook_url: Option<String>,
    pub webhook_status_code: Option<u32>,
    pub webhook_auth_header_name: Option<String>,
    pub auto_highlights_result: Option<AutoHighlightsResult>,
    pub audio_start_from: Option<u32>,
    pub audio_end_at: Option<u32>,
    pub word_boost: Option<Vec<String>>,
    pub boost_param: Option<String>,
    pub filter_profanity: Option<bool>,
    pub redact_pii_audio: Option<bool>,
    pub redact_pii_audio_quality: Option<String>,
    pub redact_pii_policies: Option<Vec<PiiType>>,
    pub redact_pii_sub: Option<String>,
    pub speaker_labels: Option<bool>,
    pub speakers_expected: Option<u32>,
    pub content_safety: Option<bool>,
    pub content_safety_labels: Option<ContentSafetyLabels>,
    pub iab_categories: Option<bool>,
    pub iab_categories_result: Option<IabCategoriesResult>,
    pub custom_spelling: Option<Vec<CustomSpelling>>,
    pub auto_chapters: Option<bool>,
    pub chapters: Option<Vec<Chapter>>,
    pub summary_type: Option<String>,
    pub summary_model: Option<String>,
    pub summary: Option<String>,
    pub custom_topics: Option<bool>,
    pub topics: Option<Vec<String>>,
    pub sentiment_analysis: Option<bool>,
    pub sentiment_analysis_results: Option<Vec<SentimentAnalysisResult>>,
    pub entity_detection: Option<bool>,
    pub entities: Option<Vec<Entity>>,
    pub speech_threshold: Option<f64>,
    pub throttled: Option<bool>,
    pub error: Option<String>,
    pub dual_channel: Option<bool>,
    pub speed_boost: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Word {
    pub confidence: Option<f64>,
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub text: Option<String>,
    pub channel: Option<String>, // Added channel based on the provided JSON
    pub speaker: Option<String>, // Added speaker based on the provided JSON
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Utterance {
    pub confidence: Option<f64>,
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub text: Option<String>,
    pub words: Option<Vec<Word>>,
    pub speaker: Option<String>,
    pub channel: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AutoHighlightsResult {
    pub status: Option<String>,
    pub results: Option<Vec<AutoHighlight>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AutoHighlight {
    pub count: Option<u32>,
    pub rank: Option<f64>,
    pub text: Option<String>,
    pub timestamps: Option<Vec<Timestamp>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Timestamp {
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentSafetyLabels {
    pub status: Option<String>,
    pub results: Option<Vec<ContentSafetyResult>>,
    pub summary: Option<HashMap<String, f64>>,
    pub severity_score_summary: Option<HashMap<String, SeverityScore>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentSafetyResult {
    pub text: Option<String>,
    pub labels: Option<Vec<ContentSafetyLabel>>,
    pub sentences_idx_start: Option<u32>,
    pub sentences_idx_end: Option<u32>,
    pub timestamp: Option<Timestamp>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentSafetyLabel {
    pub label: Option<String>,
    pub confidence: Option<f64>,
    pub severity: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SeverityScore {
    pub low: Option<f64>,
    pub medium: Option<f64>,
    pub high: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IabCategoriesResult {
    pub status: Option<String>,
    pub results: Option<Vec<IabCategoryResult>>,
    pub summary: Option<HashMap<String, f64>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IabCategoryResult {
    pub text: Option<String>,
    pub labels: Option<Vec<IabCategoryLabel>>,
    pub timestamp: Option<Timestamp>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IabCategoryLabel {
    pub relevance: Option<f64>,
    pub label: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomSpelling {
    pub from: Option<Vec<String>>,
    pub to: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Chapter {
    pub gist: Option<String>,
    pub headline: Option<String>,
    pub summary: Option<String>,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SentimentAnalysisResult {
    pub text: Option<String>,
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub sentiment: Option<String>,
    pub confidence: Option<f64>,
    pub channel: Option<String>,
    pub speaker: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entity {
    pub entity_type: Option<String>,
    pub text: Option<String>,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListTranscriptResponse {
    pub page_details: PageDetails,
    pub transcripts: Vec<Transcript>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageDetails {
    pub limit: Option<u32>,
    pub result_count: Option<u32>,
    pub current_url: Option<String>,
    pub prev_url: Option<String>,
    pub next_url: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transcript {
    pub id: Option<String>,
    pub resource_url: Option<String>,
    pub status: Option<String>,
    pub created: Option<String>,
    pub audio_url: Option<String>,
    pub completed: Option<String>,
    pub error: Option<String>,
}