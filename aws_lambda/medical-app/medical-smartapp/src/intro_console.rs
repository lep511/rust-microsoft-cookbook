use crate::oidc_request::get_mdata;
use crate::http_page::get_main_page;
use crate::libs::{MedicalRecord, MainPageParams, Patient};
use crate::libs::{
    DefaultValueSetter, Medication, VitalSign, Treatment, TimelineEvent,
    Appointment, extract_ethnicity,
};
use lambda_runtime::tracing::error;
use crate::llm_allergies::extract_allergies;
use chrono::{NaiveDate, Utc};
use crate::scrab_errors::ScrabError;

pub struct PatientDetails {
    pub full_name: String,
    pub address: String,
    pub gender: String,
    pub ethnicity: String,
    pub phone: String,
    pub age: u32,
}

pub async fn main_console_page(
    params: &MainPageParams,
) -> Result<String, ScrabError> {

    let query = format!("Patient/{}", params.patient_id);

    let patient_data = match get_mdata(
        &params.iss,
        &query,
        &params.access_token,
    ).await {
        Ok(data) => data,
        Err(e) => {
            error!("Error getting patient data: {:?}", e);
            return Err(ScrabError::GenericError("Error getting metadata".to_string()));
        }
    };
        
    let patient: Patient = match parse_patient_response(&patient_data) {
        Ok(patient) => patient,
        Err(e) => {
            error!("Error parsing patient data: {:?}", e);
            return Err(ScrabError::GenericError("Error parsing patient data".to_string()));
        }
    };

    let patient_details: PatientDetails = extract_patient_details(&patient);

    // Create a new record with all default 'n/a' values
    let mut record = MedicalRecord::new_default();

    // Extract allergies data
    let allergies: Vec<String> = extract_allergies(params).await;

    // Selectively update specific fields
    record.set_fields(|r| {
        r.id = params.patient_id.clone();
        r.name = patient_details.full_name.clone();
        r.age = patient_details.age;
        r.gender = patient_details.gender.clone();
        r.ethnicity = patient_details.ethnicity.clone();
        r.address = patient_details.address.clone();
        r.phone = patient_details.phone.clone();
        r.email = "somemail@example.com".to_string();
        r.emergency_contact = "John Smith (Husband) - (555) 987-6543".to_string();
        
        // r.allergies = vec!["Penicillin".to_string()];
        r.allergies = allergies.clone();
        
        r.current_medications.push(Medication {
            name: "Aspirin".to_string(),
            dosage: "100mg".to_string(),
            frequency: "Daily".to_string(),
        });

        r.current_medications.push(Medication {
            name: "Metformin".to_string(),
            dosage: "500mg".to_string(),
            frequency: "Twice daily".to_string(),
        });

        r.vital_signs.push(VitalSign {
            date: "2023-06-01".to_string(),
            heart_rate: 74,
            blood_pressure: "123/81".to_string(),
            temperature: 98.8,
            respiratory_rate: 16,
            oxygen_saturation: 97,
        });

        r.vital_signs.push(VitalSign {
            date: "2023-05-01".to_string(),
            heart_rate: 71,
            blood_pressure: "119/80".to_string(),
            temperature: 98.6,
            respiratory_rate: 15,
            oxygen_saturation: 98,
        });

        r.vital_signs.push(VitalSign {
            date: "2023-04-01".to_string(),
            heart_rate: 73,
            blood_pressure: "121/79".to_string(),
            temperature: 98.5,
            respiratory_rate: 16,
            oxygen_saturation: 98,
        });

        r.vital_signs.push(VitalSign {
            date: "2023-03-01".to_string(),
            heart_rate: 70,
            blood_pressure: "118/78".to_string(),
            temperature: 98.7,
            respiratory_rate: 15,
            oxygen_saturation: 99,
        });

        r.vital_signs.push(VitalSign {
            date: "2023-02-01".to_string(),
            heart_rate: 75,
            blood_pressure: "122/82".to_string(),
            temperature: 98.4,
            respiratory_rate: 16,
            oxygen_saturation: 97,
        });

        r.vital_signs.push(VitalSign {
            date: "2023-01-01".to_string(),
            heart_rate: 72,
            blood_pressure: "120/80".to_string(),
            temperature: 98.6,
            respiratory_rate: 16,
            oxygen_saturation: 98,
        });

        r.treatments.push(Treatment {
            date: "2023-01-15".to_string(),
            t_type: "Medication Adjustment".to_string(),
            provider: "Medication Adjustment".to_string(),
            notes: "Increased Lisinopril to 10mg".to_string(),
        });

        r.treatments.push(Treatment {
            date: "2023-02-20".to_string(),
            t_type: "Physical Therapy".to_string(),
            provider: "Dr. Michael Rodriguez".to_string(),
            notes: "Started PT for lower back pain".to_string(),
        });

        r.treatments.push(Treatment {
            date: "2023-03-10".to_string(),
            t_type: "Lab Work".to_string(),
            provider: "Dr. Emily Chen".to_string(),
            notes: "Comprehensive metabolic panel and A1C".to_string(),
        });

        r.treatments.push(Treatment {
            date: "2023-04-05".to_string(),
            t_type: "Specialist Consultation".to_string(),
            provider: "Dr. Sarah Johnson".to_string(),
            notes: "Endocrinology consult for diabetes management".to_string(),
        });

        r.treatments.push(Treatment {
            date: "2023-05-18".to_string(),
            t_type: "Medication Adjustment".to_string(),
            provider: "Dr. Emily Chen".to_string(),
            notes: "Added low-dose aspirin".to_string(),
        });

        r.appointments.push(Appointment {
            date: "2023-07-15".to_string(),
            time: "10:00 AM".to_string(),
            provider: "Dr. Emily Chen".to_string(),
            a_type: "Follow-up".to_string(),
            location: "Main Hospital, Room 302".to_string(),
        });

        r.appointments.push(Appointment {
            date: "2023-08-05".to_string(),
            time: "2:30 PM".to_string(),
            provider: "Dr. Michael Rodriguez".to_string(),
            a_type: "Physical Therapy".to_string(),
            location: "Rehabilitation Center".to_string(),
        });

        r.appointments.push(Appointment {
            date: "2023-09-10".to_string(),
            time: "9:15 AM".to_string(),
            provider: "Dr. Sarah Johnson".to_string(),
            a_type: "Endocrinology".to_string(),
            location: "Specialty Clinic, Suite 105".to_string(),
        });

        r.timeline.push(TimelineEvent {
            year: "2023".to_string(),
            title: "Cardiac Assessment".to_string(),
            description: "Comprehensive cardiac evaluation revealed normal heart function with minor irregularities in rhythm. Prescribed beta blockers for management.".to_string(),
            icon: "HeartPulse".to_string(),
            highlight: true,
        });

        r.timeline.push(TimelineEvent {
            year: "2022".to_string(),
            title: "Type 2 Diabetes Diagnosis".to_string(),
            description: "Initial diagnosis of Type 2 Diabetes. Started on Metformin 500mg twice daily. Implemented lifestyle modifications and dietary changes.".to_string(),
            icon: "Activity".to_string(),
            highlight: false,
        });

        r.timeline.push(TimelineEvent {
            year: "2021".to_string(),
            title: "Annual Physical".to_string(),
            description: "Regular check-up showed elevated blood pressure (140/90). Recommended lifestyle changes and monthly monitoring.".to_string(),
            icon: "Stethoscope".to_string(),
            highlight: false,
        });

        r.timeline.push(TimelineEvent {
            year: "2019".to_string(),
            title: "Medication Review".to_string(),
            description: "Comprehensive medication review. Adjusted dosages for better management of chronic conditions.".to_string(),
            icon: "Pill".to_string(),
            highlight: false,
        });
        
        r.timeline.push(TimelineEvent {
            year: "2015".to_string(),
            title: "New Hospital Admission".to_string(),
            description: "Brief hospital stay due to severe pneumonia.".to_string(),
            icon: "Hospital".to_string(),
            highlight: false,
        });

    });

    let patients_json = match serde_json::to_string_pretty(&record) {
        Ok(json) => json,
        Err(e) => {
            error!("Error serializing record to JSON: {:?}", e);
            return Err(ScrabError::GenericError("Error serializing record to JSON".to_string()));
        }
    };

    let html = get_main_page(&patients_json);
    
    Ok(html)
}

/// Helper function to parse JSON string into Patient
pub fn parse_patient_response(json_str: &str) -> Result<Patient, Box<dyn std::error::Error>> {
    match serde_json::from_str(json_str) {
        Ok(patient) => Ok(patient),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn capitalize_first_char(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first_char) => first_char.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// Calculates age based on a birth date string in the format "YYYY-MM-DD"
pub fn calculate_age(birth_date_str: &str) -> u32 {
    // Try to parse the birth date string
    let Ok(birth_date) = NaiveDate::parse_from_str(birth_date_str, "%Y-%m-%d") else {
        return 0;
    };
    
    // Get current date
    let current_date = Utc::now().date_naive();
    
    // Calculate age, return 0 if birth date is in the future
    current_date.years_since(birth_date).unwrap_or(0)
}

fn extract_patient_details(patient: &Patient) -> PatientDetails {

    // Extract first given name
    let first_given_name = patient.name.as_ref()
        .and_then(|names| names.first())
        .and_then(|name| name.given.as_ref())
        .and_then(|names| names.first())
        .cloned()
        .unwrap_or_else(|| "n/a".to_string());

    // Extract family name
    let family_name = patient.name.as_ref()
        .and_then(|names| names.first())
        .and_then(|name| name.family.clone())
        .unwrap_or_else(|| "n/a".to_string());

    // Extract full address
    let full_address = patient.address.as_ref()
        .and_then(|addresses| addresses.first())
        .map(|address| {
            let line = address.line.as_ref()
                .and_then(|lines| lines.first())
                .cloned()
                .unwrap_or_else(|| "n/a".to_string());
            
            let city = address.city.clone().unwrap_or_else(|| "n/a".to_string());
            let state = address.state.clone().unwrap_or_else(|| "n/a".to_string());
            let postal_code = address.postal_code.clone().unwrap_or_else(|| "n/a".to_string());

            format!("{}, {}, {} {}", line, city, state, postal_code)
        })
        .unwrap_or_else(|| "n/a".to_string());

    // Extract phone
    let phone = patient.telecom.as_ref()
        .and_then(|telecoms| telecoms.first())
        .and_then(|telecom| telecom.value.clone())
        .unwrap_or_else(|| "n/a".to_string());

    // Extract ethnicity
    let ethnicity = extract_ethnicity(patient);

    // Extract gender
    let mut gender = patient.gender.clone().unwrap_or_else(|| "n/a".to_string());
    if gender != "n/a" {gender = capitalize_first_char(&gender);}

    // Extract birth date
    let birth_date = patient.birth_date.clone().unwrap_or_else(|| "n/a".to_string());
    let mut age = 0;
    if birth_date != "n/a" {
        age = calculate_age(&birth_date);
    }

    PatientDetails {
        full_name: format!("{} {}", first_given_name, family_name),
        address: full_address,
        ethnicity,
        gender,
        phone,
        age: age,
    }
}