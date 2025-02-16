use crate::assembly::utils::GetApiKey;
use crate::assembly::libs::{
    TranscriptRequest, TranscriptResponse, GetTranscriptResponse,
    ListTranscriptParameters, ListTranscriptResponse, PiiType,
};
use crate::assembly::requests::{
    upload_media, request_engine, get_engine,
};
use crate::assembly::{ASSEMBLYAI_BASE_URL, SPEECH_ACCEPT_MODEL};
use crate::assembly::error::AssemblyError;
use std::time::Duration;
use reqwest::Url;
use log::error;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TranscriptAssemblyAI {
    pub api_key: String,
    pub request: TranscriptRequest,
    pub timeout: Duration,
    pub max_retries: u32,
}


#[allow(dead_code)]
impl TranscriptAssemblyAI {
    pub fn new() -> Self {
        let api_key: String = match Self::get_api_key() {
            Ok(api_key) => api_key,
            Err(_) => "not_key".to_string()
        };

        let request = TranscriptRequest::default();
        
        Self {
            api_key: api_key,
            request: request,
            timeout: Duration::from_secs(900), // default: 15 minutes
            max_retries: 3,                    // default: 3
        }
    }

    pub async fn upload_file(
        self,
        file_path: &str,
    ) -> Result<String, AssemblyError> {
        let base_url = format!("{}/upload", ASSEMBLYAI_BASE_URL);

        let buffer = match std::fs::read(file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Error reading local file [E023]: {:?}", e);
                return Err(AssemblyError::RequestUploadError);
            }
        };

        let upload_url = upload_media(
            &base_url,
            &self.api_key,
            &buffer,
            self.timeout,
        ).await;

        match upload_url {
            Ok(url) => Ok(url),
            Err(e) => {
                error!("Failed to get the upload URL [E024]. {:?}", e);
                Err(AssemblyError::RequestUploadError)
            }
        }
    }

    pub async fn transcript(
        mut self,
        audio_url: &str,
    ) -> Result<TranscriptResponse, AssemblyError> {

        let base_url = format!("{}/transcript", ASSEMBLYAI_BASE_URL);
        self.request.audio_url = Some(audio_url.to_string());

        let response = request_engine(
            &self.request,
            &base_url,
            &self.api_key,
            self.timeout,
            self.max_retries,
        ).await;

        let response_string = match response {
            Ok(response) => response,
            Err(e) => {
                error!("Error TranscriptResponse [E025]: {:?}", e);
                return Err(AssemblyError::ResponseContentError);
            }
        };

        let transcript_response: TranscriptResponse = match serde_json::from_str(&response_string) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error TranscriptResponse [E026]: {:?}", e);
                return Err(AssemblyError::ResponseContentError);
            }
        };

        Ok(transcript_response)
    }

    pub async fn get_transcript(
        self, 
        id: &str
    ) -> Result<GetTranscriptResponse, AssemblyError> {
        let base_url = format!("{}/transcript/{}", ASSEMBLYAI_BASE_URL, id);

        let response = get_engine(
            &base_url,
            &self.api_key,        
        ).await;

        let response_string = match response {
            Ok(response) => response,
            Err(e) => {
                error!("Error GetTranscriptResponse [E035]: {:?}", e);
                return Err(AssemblyError::ResponseContentError);
            }
        };
        
        let transcript_response: GetTranscriptResponse = match serde_json::from_str(&response_string) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error GetTranscriptResponse [E036]: {:?}", e);
                return Err(AssemblyError::ResponseContentError);
            }
        };

        Ok(transcript_response)
    }

    pub async fn list_transcripts(
        self,
        parameters: Option<ListTranscriptParameters>,
    ) -> Result<ListTranscriptResponse, AssemblyError> {
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
       
        let base_url = format!("{}/transcript", ASSEMBLYAI_BASE_URL);
        let base_url = match Url::parse_with_params(&base_url, &param_data) {
            Ok(url) => url,
            Err(e) => {
                error!("Error [E043]{:?}", e);
                return Err(AssemblyError::ResponseContentError);
            }
        };
        
        let response = get_engine(
            &base_url.to_string(),
            &self.api_key,        
        ).await;

        let response_string = match response {
            Ok(response) => response,
            Err(e) => {
                error!("Error ListTranscriptResponse [E044]: {:?}", e);
                return Err(AssemblyError::ResponseContentError);
            }
        };

        let transcript_response: ListTranscriptResponse = match serde_json::from_str(&response_string) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error ListTranscriptResponse [E045]: {:?}", e);
                return Err(AssemblyError::ResponseContentError);
            }
        };

        Ok(transcript_response)
    }

    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = Duration::from_secs(timeout);
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

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }
}

impl GetApiKey for TranscriptAssemblyAI {}
