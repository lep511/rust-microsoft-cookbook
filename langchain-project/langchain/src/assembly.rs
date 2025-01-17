use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::llmerror::AssemblyAIError;
use std::collections::HashMap;
use std::env;

pub static ASSEMBLYAI_BASE_URL: &str = "https://api.assemblyai.com/v2";

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = true;

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct TranscriptRequest {
    pub audio_url: String,
    pub speech_model: SpeechModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_end_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_start_from: Option<i32>,
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
pub enum SpeechModel {
    #[serde(rename = "best")]
    Best,
    #[serde(rename = "nano")]
    Nano,
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

pub trait PrintDebug {
    fn print_pre(request: &impl serde::Serialize, active: bool) {
        if !active {
            println!();
        } else {
            match serde_json::to_string_pretty(request) {
                Ok(json) => println!("Pretty-printed JSON:\n{}", json),
                Err(e) => println!("[ERROR] {:?}", e)
            }
        }
    }
}

pub trait GetApiKey {
    fn get_api_key() -> Result<String, AssemblyAIError> {
        match env::var("ASSEMBLYAI_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                println!("[ERROR] ASSEMBLYAI_API_KEY not found in environment variables");
                Err(AssemblyAIError::ApiKeyNotFound)
            }
            Err(e) => {
                println!("[ERROR] {:?}", e);
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
    pub fn new(model: &str) -> Result<Self, AssemblyAIError> {
        let api_key = Self::get_api_key()?;

        let speech_model: SpeechModel = match model {
            "best" => SpeechModel::Best,
            "nano" => SpeechModel::Nano,
            _ => {
                println!("[ERROR] The model must be best or nano");
                return Err(AssemblyAIError::InvalidModel);
            }
        };

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
                println!("[ERROR] TranscriptResponse: {:?}", e);
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
                println!("[ERROR] GetTranscriptResponse: {:?}", e);
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
}

impl GetApiKey for TranscriptAssemblyAI {}
impl PrintDebug for TranscriptAssemblyAI {}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TranscriptResponse {
    acoustic_model: Option<String>,
    audio_duration: Option<f64>,
    audio_end_at: Option<f64>,
    audio_start_from: Option<f64>,
    audio_url: Option<String>,
    auto_chapters: Option<bool>,
    auto_highlights: Option<bool>,
    auto_highlights_result: Option<serde_json::Value>,
    boost_param: Option<f64>,
    chapters: Option<serde_json::Value>,
    confidence: Option<f64>,
    content_safety: Option<bool>,
    content_safety_labels: Option<HashMap<String, serde_json::Value>>,
    custom_spelling: Option<serde_json::Value>,
    custom_topics: Option<serde_json::Value>,
    custom_topics_results: Option<serde_json::Value>,
    disfluencies: Option<bool>,
    dual_channel: Option<bool>,
    entities: Option<serde_json::Value>,
    entity_detection: Option<bool>,
    filter_profanity: Option<bool>,
    format_text: Option<bool>,
    iab_categories: Option<bool>,
    iab_categories_result: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    is_deleted: Option<bool>,
    language_code: Option<String>,
    language_confidence: Option<f64>,
    language_confidence_threshold: Option<f64>,
    language_detection: Option<bool>,
    language_model: Option<String>,
    multichannel: Option<bool>,
    punctuate: Option<bool>,
    redact_pii: Option<bool>,
    redact_pii_audio: Option<bool>,
    redact_pii_audio_quality: Option<String>,
    redact_pii_policies: Option<Vec<String>>,
    redact_pii_sub: Option<String>,
    sentiment_analysis: Option<bool>,
    sentiment_analysis_results: Option<serde_json::Value>,
    speaker_labels: Option<bool>,
    speakers_expected: Option<i32>,
    speech_model: Option<String>,
    speech_threshold: Option<f64>,
    speed_boost: Option<bool>,
    status: Option<String>,
    summarization: Option<bool>,
    summary: Option<String>,
    summary_model: Option<String>,
    summary_type: Option<String>,
    text: Option<String>,
    throttled: Option<bool>,
    topics: Option<Vec<String>>,
    utterances: Option<serde_json::Value>,
    webhook_auth: Option<bool>,
    webhook_auth_header_name: Option<String>,
    webhook_status_code: Option<i32>,
    webhook_url: Option<String>,
    word_boost: Option<Vec<String>>,
    words: Option<serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetTranscriptResponse {
    id: Option<String>,
    audio_url: Option<String>,
    status: Option<String>,
    webhook_auth: Option<bool>,
    auto_highlights: Option<bool>,
    redact_pii: Option<bool>,
    summarization: Option<bool>,
    language_model: Option<String>,
    acoustic_model: Option<String>,
    language_code: Option<String>,
    language_detection: Option<bool>,
    language_confidence_threshold: Option<f64>,
    language_confidence: Option<f64>,
    speech_model: Option<String>,
    text: Option<String>,
    words: Option<Vec<Word>>,
    utterances: Option<Vec<Utterance>>,
    confidence: Option<f64>,
    audio_duration: Option<u32>,
    punctuate: Option<bool>,
    format_text: Option<bool>,
    disfluencies: Option<bool>,
    multichannel: Option<bool>,
    audio_channels: Option<u32>,
    webhook_url: Option<String>,
    webhook_status_code: Option<u32>,
    webhook_auth_header_name: Option<String>,
    auto_highlights_result: Option<AutoHighlightsResult>,
    audio_start_from: Option<u32>,
    audio_end_at: Option<u32>,
    word_boost: Option<Vec<String>>,
    boost_param: Option<String>,
    filter_profanity: Option<bool>,
    redact_pii_audio: Option<bool>,
    redact_pii_audio_quality: Option<String>,
    redact_pii_policies: Option<Vec<String>>,
    redact_pii_sub: Option<String>,
    speaker_labels: Option<bool>,
    speakers_expected: Option<u32>,
    content_safety: Option<bool>,
    content_safety_labels: Option<ContentSafetyLabels>,
    iab_categories: Option<bool>,
    iab_categories_result: Option<IabCategoriesResult>,
    custom_spelling: Option<Vec<CustomSpelling>>,
    auto_chapters: Option<bool>,
    chapters: Option<Vec<Chapter>>,
    summary_type: Option<String>,
    summary_model: Option<String>,
    summary: Option<String>,
    custom_topics: Option<bool>,
    topics: Option<Vec<String>>,
    sentiment_analysis: Option<bool>,
    sentiment_analysis_results: Option<Vec<SentimentAnalysisResult>>,
    entity_detection: Option<bool>,
    entities: Option<Vec<Entity>>,
    speech_threshold: Option<f64>,
    throttled: Option<bool>,
    error: Option<String>,
    dual_channel: Option<bool>,
    speed_boost: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Word {
    confidence: Option<f64>,
    start: Option<u32>,
    end: Option<u32>,
    text: Option<String>,
    channel: Option<String>, // Added channel based on the provided JSON
    speaker: Option<String>, // Added speaker based on the provided JSON
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Utterance {
    confidence: Option<f64>,
    start: Option<u32>,
    end: Option<u32>,
    text: Option<String>,
    words: Option<Vec<Word>>,
    speaker: Option<String>,
    channel: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AutoHighlightsResult {
    status: Option<String>,
    results: Option<Vec<AutoHighlight>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AutoHighlight {
    count: Option<u32>,
    rank: Option<f64>,
    text: Option<String>,
    timestamps: Option<Vec<Timestamp>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Timestamp {
    start: Option<u32>,
    end: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentSafetyLabels {
    status: Option<String>,
    results: Option<Vec<ContentSafetyResult>>,
    summary: Option<HashMap<String, f64>>,
    severity_score_summary: Option<HashMap<String, SeverityScore>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentSafetyResult {
    text: Option<String>,
    labels: Option<Vec<ContentSafetyLabel>>,
    sentences_idx_start: Option<u32>,
    sentences_idx_end: Option<u32>,
    timestamp: Option<Timestamp>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentSafetyLabel {
    label: Option<String>,
    confidence: Option<f64>,
    severity: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SeverityScore {
    low: Option<f64>,
    medium: Option<f64>,
    high: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IabCategoriesResult {
    status: Option<String>,
    results: Option<Vec<IabCategoryResult>>,
    summary: Option<HashMap<String, f64>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IabCategoryResult {
    text: Option<String>,
    labels: Option<Vec<IabCategoryLabel>>,
    timestamp: Option<Timestamp>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IabCategoryLabel {
    relevance: Option<f64>,
    label: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomSpelling {
    from: Option<Vec<String>>,
    to: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Chapter {
    gist: Option<String>,
    headline: Option<String>,
    summary: Option<String>,
    start: Option<u32>,
    end: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SentimentAnalysisResult {
    text: Option<String>,
    start: Option<u32>,
    end: Option<u32>,
    sentiment: Option<String>,
    confidence: Option<f64>,
    channel: Option<String>,
    speaker: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entity {
    entity_type: Option<String>,
    text: Option<String>,
    start: Option<u32>,
    end: Option<u32>,
}