use crate::assembly::utils::GetApiKey;
use crate::assembly::libs::{
    TranscriptRequest, TranscriptResponse,
};
use crate::assembly::requests::{upload_media, request_engine};
use crate::assembly::ASSEMBLYAI_BASE_URL;
use crate::llmerror::AssemblyError;
use std::time::Duration;
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
    pub fn new() -> Result<Self, AssemblyError> {
        let api_key = Self::get_api_key()?;

        let request = TranscriptRequest::default();
        
        Ok(Self {
            api_key: api_key,
            request: request,
            timeout: Duration::from_secs(900), // default: 15 minutes
            max_retries: 3,                    // default: 3
        })
    }

    pub async fn upload_file(
        self,
        file_path: &str,
    ) -> Result<String, AssemblyError> {
        let base_url = format!("{}/upload", ASSEMBLYAI_BASE_URL);

        let buffer = match std::fs::read(file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Error reading local file: {:?}", e);
                return Err(AssemblyError::RequestUploadError);
            }
        };

        let timeout =  Duration::from_secs(300);

        let upload_url = upload_media(
            &base_url,
            &self.api_key,
            &buffer,
            timeout,
        ).await;

        match upload_url {
            Ok(url) => Ok(url),
            Err(e) => {
                error!("Failed to get the upload URL. {:?}", e);
                Err(AssemblyError::RequestUploadError)
            }
        }
    }




    // pub async fn transcript(
    //     mut self,
    //     audio_url: &str,
    // ) -> Result<TranscriptResponse, AssemblyError> {

    //     let base_url = format!("{}/transcript", ASSEMBLYAI_BASE_URL);

    //     let client = Client::builder()
    //         .use_rustls_tls()
    //         .build()?;
        
    //     self.request.audio_url = audio_url.to_string();

    //     let response = client
    //         .post(base_url)
    //         .timeout(Duration::from_secs(self.timeout))
    //         .header("Authorization", self.api_key.clone())
    //         .header("Content-Type", "application/json")
    //         .json(&self.request)
    //         .send()
    //         .await?
    //         .json::<serde_json::Value>()
    //         .await?;

       
    //     let response_string = response.to_string();
    //     let transcript_response: TranscriptResponse = match serde_json::from_str(&response_string) {
    //         Ok(response_form) => response_form,
    //         Err(e) => {
    //             error!("Error TranscriptResponse: {:?}", e);
    //             return Err(AssemblyError::ResponseContentError);
    //         }
    //     };

    //     Ok(transcript_response)
    // }



    // pub async fn get_transcript(
    //     self, 
    //     id: &str
    // ) -> Result<GetTranscriptResponse, AssemblyError> {
    //     let base_url = format!("{}/transcript/{}", ASSEMBLYAI_BASE_URL, id);
    //     let client = Client::builder()
    //         .use_rustls_tls()
    //         .build()?;

    //     let response = client
    //         .get(base_url)
    //         .timeout(Duration::from_secs(self.timeout))
    //         .header("Authorization", self.api_key.clone())
    //         .json(&self.request)
    //         .send()
    //         .await?
    //         .json::<serde_json::Value>()
    //         .await?;
        

    //     let response_string = response.to_string();
        
    //     let transcript_response: GetTranscriptResponse = match serde_json::from_str(&response_string) {
    //         Ok(response_form) => response_form,
    //         Err(e) => {
    //             error!("Error GetTranscriptResponse: {:?}", e);
    //             return Err(AssemblyError::ResponseContentError);
    //         }
    //     };

    //     Ok(transcript_response)
    // }




    // pub async fn list_transcripts(
    //     self,
    //     parameters: Option<ListTranscriptParameters>,
    // ) -> Result<ListTranscriptResponse, AssemblyError> {
    //     let mut param_data = vec![];

    //     if let Some(params) = parameters {
    //         if let Some(limit) = params.limit {
    //             let param_limit = ("limit", limit.to_string());
    //             param_data.push(param_limit);
    //         }

    //         if let Some(status) = params.status {
    //             let param_status = ("status", status);
    //             param_data.push(param_status);
    //         }

    //         if let Some(created_on) = params.created_on {
    //             let param_created_on = ("created_on", created_on);
    //             param_data.push(param_created_on);
    //         }

    //         if let Some(before_id) = params.before_id {
    //             let param_before_id = ("before_id", before_id);
    //             param_data.push(param_before_id);
    //         }

    //         if let Some(after_id) = params.after_id {
    //             let param_after_id = ("after_id", after_id);
    //             param_data.push(param_after_id);
    //         }

    //         if let Some(throttled_only) = params.throttled_only {
    //             let param_throttled_only = ("throttled_only", throttled_only.to_string());
    //             param_data.push(param_throttled_only);
    //         }           
    //     }

    //     let client = Client::builder()
    //         .use_rustls_tls()
    //         .build()?;
        
    //     let base_url = format!("{}/transcript", ASSEMBLYAI_BASE_URL);
    //     let base_url = match Url::parse_with_params(&base_url, &param_data) {
    //         Ok(url) => url,
    //         Err(e) => {
    //             error!("Error {:?}", e);
    //             return Err(AssemblyError::ResponseContentError);
    //         }
    //     };
        
    //     let response = client
    //         .get(base_url)
    //         .timeout(Duration::from_secs(self.timeout))
    //         .header("Authorization", self.api_key.clone())
    //         .json(&self.request)
    //         .send()
    //         .await?
    //         .json::<serde_json::Value>()
    //         .await?;

    //     let response_string = response.to_string();

    //     let transcript_response: ListTranscriptResponse = match serde_json::from_str(&response_string) {
    //         Ok(response_form) => response_form,
    //         Err(e) => {
    //             error!("Error ListTranscriptResponse: {:?}", e);
    //             return Err(AssemblyError::ResponseContentError);
    //         }
    //     };

    //     Ok(transcript_response)
    // }




    // pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
    //     self.timeout = timeout;
    //     self
    // }

    // pub fn with_language_detection(mut self, language_detection: bool) -> Self {
    //     self.request.language_detection = Some(language_detection);
    //     self
    // }

    // pub fn with_speech_model(mut self, speech_model: &str) -> Self {
    //     if !SPEECH_ACCEPT_MODEL.contains(&speech_model) {
    //         error!("Error Speech model not accepted");
    //         return self;
    //     }
    //     self.request.speech_model = Some(speech_model.to_string());
    //     self
    // }

    // pub fn with_redact_pii_policies(mut self, redact_pii_policies: Vec<PiiType>) -> Self {
    //     self.request.redact_pii_policies = Some(redact_pii_policies);
    //     self
    // }

    // pub fn with_audio_end_at(mut self, audio_end_at: u32) -> Self {
    //     self.request.audio_end_at = Some(audio_end_at);
    //     self
    // }

    // pub fn with_language_code(mut self, language_code: &str) -> Self {
    //     self.request.language_code = Some(language_code.to_string());
    //     self
    // }
}

impl GetApiKey for TranscriptAssemblyAI {}

