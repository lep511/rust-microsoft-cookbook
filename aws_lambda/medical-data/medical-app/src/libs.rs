use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CDSHooksResponse {
    #[serde(rename = "hookInstance")]
    pub hook_instance: Option<String>,
    pub hook: Option<String>,
    #[serde(rename = "fhirServer")]
    pub fhir_server: Option<String>,
    pub context: Option<Context>,
    pub prefetch: Option<Prefetch>,
    #[serde(rename = "fhirAuthorization")]
    pub fhir_authorization: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    #[serde(rename = "patientId")]
    pub patient_id: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Prefetch {
    pub conditions: Option<Bundle>,
    pub patient: Option<Patient>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bundle {
    #[serde(rename = "resourceType")]
    pub resource_type: Option<String>,
    #[serde(rename = "type")]
    pub bundle_type: Option<String>,
    pub total: Option<i32>,
    pub link: Option<Vec<Link>>,
    pub entry: Option<Vec<BundleEntry>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub relation: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BundleEntry {
    #[serde(rename = "fullUrl")]
    pub full_url: Option<String>,
    pub resource: Option<Resource>,
    pub response: Option<EntryResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryResponse {
    pub status: Option<String>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Resource {
    #[serde(rename = "resourceType")]
    pub resource_type: Option<String>,
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub identifier: Option<Vec<Identifier>>,
    #[serde(rename = "clinicalStatus")]
    pub clinical_status: Option<CodingWrapper>,
    #[serde(rename = "verificationStatus")]
    pub verification_status: Option<CodingWrapper>,
    pub category: Option<Vec<CodingWrapper>>,
    pub code: Option<CodeableConcept>,
    pub subject: Option<Reference>,
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
    pub version_id: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    pub profile: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Identifier {
    pub system: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CodingWrapper {
    pub coding: Option<Vec<Coding>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coding {
    pub system: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CodeableConcept {
    pub coding: Option<Vec<Coding>>,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reference {
    pub reference: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Period {
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Patient {
    #[serde(rename = "resourceType")]
    pub resource_type: Option<String>,
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Option<Vec<Extension>>,
    pub identifier: Option<Vec<Identifier>>,
    pub name: Option<Vec<HumanName>>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub gender: Option<String>,
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,
    pub address: Option<Vec<Address>>,
    pub communication: Option<Vec<Communication>>,
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Reference>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Extension {
    pub extension: Option<Vec<Extension>>,
    pub url: Option<String>,
    #[serde(rename = "valueString", skip_serializing_if = "Option::is_none")]
    pub value_string: Option<String>,
    #[serde(rename = "valueCode", skip_serializing_if = "Option::is_none")]
    pub value_code: Option<String>,
    #[serde(rename = "valueCoding", skip_serializing_if = "Option::is_none")]
    pub value_coding: Option<Coding>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HumanName {
    pub family: Option<String>,
    pub given: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactPoint {
    pub system: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "use")]
    pub c_use: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    #[serde(rename = "use")]
    pub a_use: Option<String>,
    pub line: Option<Vec<String>>,
    pub city: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Communication {
    pub language: Option<CodeableConcept>,
}