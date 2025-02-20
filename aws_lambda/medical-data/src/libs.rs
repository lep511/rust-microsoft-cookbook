use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct CDSHooksResponse {
    pub hook_instance: String,
    pub hook: String,
    #[serde(rename = "fhirServer")]
    pub fhir_server: String,
    pub context: Context,
    pub prefetch: Prefetch,
    #[serde(rename = "fhirAuthorization")]
    pub fhir_authorization: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    #[serde(rename = "patientId")]
    pub patient_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Prefetch {
    pub conditions: Bundle,
    pub patient: Patient,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bundle {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "type")]
    pub bundle_type: String,
    pub total: i32,
    pub link: Vec<Link>,
    pub entry: Vec<BundleEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub relation: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BundleEntry {
    #[serde(rename = "fullUrl")]
    pub full_url: String,
    pub resource: Resource,
    pub response: EntryResponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryResponse {
    pub status: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Resource {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    pub meta: Meta,
    pub identifier: Vec<Identifier>,
    #[serde(rename = "clinicalStatus")]
    pub clinical_status: CodingWrapper,
    #[serde(rename = "verificationStatus")]
    pub verification_status: CodingWrapper,
    pub category: Vec<CodingWrapper>,
    pub code: CodeableConcept,
    pub subject: Reference,
    #[serde(rename = "onsetDateTime", skip_serializing_if = "Option::is_none")]
    pub onset_date_time: Option<String>,
    #[serde(rename = "onsetPeriod", skip_serializing_if = "Option::is_none")]
    pub onset_period: Option<Period>,
    #[serde(rename = "abatementDateTime", skip_serializing_if = "Option::is_none")]
    pub abatement_date_time: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    pub profile: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Identifier {
    pub system: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CodingWrapper {
    pub coding: Vec<Coding>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coding {
    pub system: String,
    pub code: String,
    pub display: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CodeableConcept {
    pub coding: Vec<Coding>,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reference {
    pub reference: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Period {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Patient {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    pub meta: Meta,
    pub extension: Vec<Extension>,
    pub identifier: Vec<Identifier>,
    pub name: Vec<HumanName>,
    pub telecom: Vec<ContactPoint>,
    pub gender: String,
    #[serde(rename = "birthDate")]
    pub birth_date: String,
    pub address: Vec<Address>,
    pub communication: Vec<Communication>,
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Reference,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Extension {
    pub extension: Option<Vec<Extension>>,
    pub url: String,
    #[serde(rename = "valueString", skip_serializing_if = "Option::is_none")]
    pub value_string: Option<String>,
    #[serde(rename = "valueCode", skip_serializing_if = "Option::is_none")]
    pub value_code: Option<String>,
    #[serde(rename = "valueCoding", skip_serializing_if = "Option::is_none")]
    pub value_coding: Option<Coding>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HumanName {
    pub family: String,
    pub given: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactPoint {
    pub system: String,
    pub value: String,
    pub use: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    pub use: String,
    pub line: Vec<String>,
    pub city: String,
    pub state: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Communication {
    pub language: CodeableConcept,
}

// Example usage function
pub fn process_cds_hooks_response(json_str: &str) -> Result<CDSHooksResponse, serde_json::Error> {
    let response: CDSHooksResponse = serde_json::from_str(json_str)?;
    Ok(response)
}