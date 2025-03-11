use serde_json::Value;
use serde::{Deserialize, Serialize};

/// Root struct for the hook response
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HookResponse {
    pub hook_instance: Option<String>,
    pub hook: Option<String>,
    pub fhir_server: Option<String>,
    pub context: Option<Context>,
    pub prefetch: Option<Prefetch>,
    pub fhir_authorization: Option<Value>,
}

/// Context of the hook
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub patient_id: Option<String>,
    pub user_id: Option<String>,
}

/// Prefetch data included in the response
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Prefetch {
    pub conditions: Option<Bundle>,
    pub patient: Option<Patient>,
    pub allergies: Option<Bundle>,
    // pub medications: Option<Bundle>,
    // pub observations: Option<Bundle>,
    // pub procedures: Option<Bundle>,
    // pub immunizations: Option<Bundle>,
    // pub careplans: Option<Bundle>,
    // pub devices: Option<Bundle>,
    // pub encounters: Option<Bundle>,
    // pub immunizations: Option<Bundle>,
    // pub medications: Option<Bundle>,
    // pub observations: Option<Bundle>,
    // pub procedures: Option<Bundle>,
}

/// Bundle of resources (e.g., conditions)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    pub resource_type: Option<String>,
    #[serde(rename = "type")]
    pub bundle_type: Option<String>,
    pub total: Option<i32>,
    pub link: Option<Vec<Link>>,
    pub entry: Option<Vec<BundleEntry>>,
}

/// Patient resource
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    pub resource_type: Option<String>,
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Option<Vec<Extension>>,
    pub identifier: Option<Vec<Identifier>>,
    pub name: Option<Vec<HumanName>>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub address: Option<Vec<Address>>,
    pub communication: Option<Vec<Value>>,
    pub managing_organization: Option<Reference>,
}

/// Link within a bundle
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Link {
    pub relation: Option<String>,
    pub url: Option<String>,
}

/// Entry in a bundle
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntry {
    pub full_url: Option<String>,
    pub resource: Option<Resource>,
    pub response: Option<EntryResponse>,
}

/// EntryResponse metadata for an entry
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryResponse {
    pub status: Option<String>,
    pub last_modified: Option<String>,
}

/// Resource condition
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub resource_type: Option<String>,
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub identifier: Option<Vec<Identifier>>,
    pub clinical_status: Option<CodingWrapper>,
    pub verification_status: Option<CodingWrapper>,
    pub category: Option<Vec<CodingWrapper>>,
    pub code: Option<CodeableConcept>,
    pub subject: Option<Reference>,
    pub patient: Option<Reference>,
    pub reaction: Option<Vec<Reaction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onset_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onset_period: Option<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abatement_date_time: Option<String>,
}

/// Metadata for a resource
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub version_id: Option<String>,
    pub last_updated: Option<String>,
    pub profile: Vec<String>,
}

/// Identifier for a resource
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Identifier {
    pub system: Option<String>,
    pub value: Option<String>,
}

/// CodingWrapper concept (e.g., coded values with text)
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CodingWrapper {
    pub coding: Option<Vec<Coding>>,
}

/// Coding within a codeable concept
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Coding {
    pub system: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
}

/// CodeableConcept 
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CodeableConcept {
    pub coding: Option<Vec<Coding>>,
    pub text: Option<String>,
}

/// Reference to another resource
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Reference {
    pub reference: Option<String>,
    pub display: Option<String>,
}

/// Reaction to allergy intolerance 
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Reaction {
    pub manifestation: Option<Vec<CodeableConcept>>,
    pub severity: Option<String>,
}

/// Period represents a time period with optional start and end timestamps
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Period {
    pub start: Option<String>,
    pub end: Option<String>,
}

/// Extension for additional data
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    #[serde(rename = "extension")]
    pub extensions: Option<Vec<ExtensionItem>>,
    pub url: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionItem {
    pub url: Option<String>,
    pub value_string: Option<String>,
    pub value_code: Option<String>,
    pub value_coding: Option<ValueCoding>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueCoding {
    pub system: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
}

/// Human name
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HumanName {
    pub family: Option<String>,
    pub given: Option<Vec<String>>,
}

/// Contact point (e.g., phone, email)
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContactPoint {
    pub system: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "use")]
    pub contact_use: Option<String>,
}

/// Address
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(rename = "use")]
    pub address_use: Option<String>,
    pub line: Option<Vec<String>>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

pub fn get_patient_id_from_context(context: &Context) -> Option<String> {
    context.patient_id.clone()
}

// Enum for clinical status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ClinicalStatus {
    Active,
    Inactive,
    Resolved,
}

// Enum for verification status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VerificationStatus {
    Unconfirmed,
    Presumed,
    Confirmed,
    Refuted,
    EnteredInError,
}

// Enum for reaction severity
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Severity {
    Mild,
    Moderate,
    Severe,
}

fn extract_patient_name(response: &HookResponse) -> Option<(String, String)> {
    if let Some(prefetch) = &response.prefetch {
        if let Some(patient) = &prefetch.patient {
            if let Some(names) = &patient.name {
                // Assuming the first name in the list is the primary name
                if let Some(human_name) = names.get(0) {
                    let family_name = human_name.family.clone().unwrap_or_default();
                    let given_names = human_name.given.clone().unwrap_or_default();
                    let given_name = given_names.join(" ");
                    return Some((given_name, family_name));
                }
            }
        }
    }
    None
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~ MEDICAL RECORD ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MedicalRecord {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub ethnicity: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub emergency_contact: String,
    pub allergies: Vec<String>,
    pub chronic_conditions: Vec<String>,
    pub current_medications: Vec<Medication>,
    pub vital_signs: Vec<VitalSign>,
    pub treatments: Vec<Treatment>,
    pub appointments: Vec<Appointment>,
    pub timeline: Vec<TimelineEvent>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Medication {
    pub name: String,
    pub dosage: String,
    pub frequency: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VitalSign {
    pub date: String,
    pub heart_rate: u32,
    pub blood_pressure: String,
    pub temperature: f32,
    pub respiratory_rate: u32,
    pub oxygen_saturation: u32,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Treatment {
    pub date: String,
    #[serde(rename = "type")]
    pub t_type: String,
    pub provider: String,
    pub notes: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Appointment {
    pub date: String,
    pub time: String,
    pub provider: String,
    #[serde(rename = "type")]
    pub a_type: String,
    pub location: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TimelineEvent {
    pub year: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub highlight: bool,
}

// Trait for setting default values and selective updates
pub trait DefaultValueSetter {
    /// Create a new instance with default values
    fn new_default() -> Self;

    /// Selectively update fields with new values
    fn set_fields(&mut self, updater: impl Fn(&mut Self));
}

impl DefaultValueSetter for MedicalRecord {
    fn new_default() -> Self {
        let timeline = vec![];
        let appointments = vec![];
        let treatments = vec![];
        let vital_signs = vec![];
        let current_medications = vec![];

        Self {
            id: "n/a".to_string(),
            name: "n/a".to_string(),
            age: 0,
            gender: "n/a".to_string(),
            ethnicity: "n/a".to_string(),
            address: "n/a".to_string(),
            phone: "n/a".to_string(),
            email: "n/a".to_string(),
            emergency_contact: "n/a".to_string(),
            allergies: vec![],
            chronic_conditions: vec![],
            current_medications: current_medications,
            vital_signs: vital_signs,
            treatments: treatments,
            appointments: appointments,
            timeline: timeline,
        }
    }

    fn set_fields(&mut self, updater: impl Fn(&mut Self)) {
        updater(self);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~ MAIN PAGE PARAMETERS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct MainPageParams {
    pub iss: String,
    pub access_token: String,
    pub patient_id: String,
}

pub fn extract_ethnicity(patient: &Patient) -> String {
    match extract_ethnicity_handle(patient) {
        Some(ethnicity) => ethnicity,
        None => "Unknown".to_string(),
    }
}

fn extract_ethnicity_handle(patient: &Patient) -> Option<String> {
    let eth_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity";
    patient.extension.as_ref().and_then(|extensions| {
        extensions.iter().find(|ext| ext.url.as_deref() == Some(eth_url)).and_then(|ext| {
            ext.extensions.as_ref()?.iter().find_map(|sub_ext| {
                sub_ext.value_coding.as_ref()?.display.clone()
            })
        })
    })
}
