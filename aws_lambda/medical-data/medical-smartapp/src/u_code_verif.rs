use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define the FHIR structures we need
// Note: We're creating a simplified model that captures the key elements
// from the example FHIR Patient resource

#[derive(Debug, Serialize, Deserialize)]
struct Patient {
    #[serde(rename = "resourceType")]
    resource_type: String,
    id: String,
    meta: Meta,
    extension: Option<Vec<Extension>>,
    identifier: Vec<Identifier>,
    name: Vec<HumanName>,
    telecom: Vec<ContactPoint>,
    gender: String,
    #[serde(rename = "birthDate")]
    birth_date: String,
    address: Vec<Address>,
    communication: Option<Vec<Communication>>,
    #[serde(rename = "managingOrganization")]
    managing_organization: Option<Reference>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    #[serde(rename = "versionId")]
    version_id: String,
    #[serde(rename = "lastUpdated")]
    last_updated: String,
    profile: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Extension {
    url: String,
    extension: Option<Vec<ExtensionDetail>>,
    #[serde(rename = "valueCode")]
    value_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExtensionDetail {
    url: String,
    #[serde(rename = "valueCoding")]
    value_coding: Option<Coding>,
    #[serde(rename = "valueString")]
    value_string: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Coding {
    system: String,
    code: String,
    display: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Identifier {
    system: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HumanName {
    family: String,
    given: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContactPoint {
    system: String,
    value: String,
    use: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    use: Option<String>,
    line: Vec<String>,
    city: String,
    state: String,
    #[serde(rename = "postalCode")]
    postal_code: String,
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Communication {
    language: Coding,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reference {
    reference: String,
}

// A more user-friendly representation of a patient with extracted demographic information
#[derive(Debug)]
struct PatientDemographics {
    id: String,
    primary_name: String,
    birth_date: String,
    gender: String,
    race: Vec<String>,
    ethnicity: String,
    birth_sex: String,
    addresses: Vec<String>,
    phone_numbers: HashMap<String, String>,
    primary_language: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The FHIR Patient JSON string
    let patient_json = r#"{
      "resourceType": "Patient",
      "id": "8a6ee6c1-a763-4580-adfb-a584a4e0d25f",
      "meta": {
        "versionId": "1",
        "lastUpdated": "2025-02-19T18:06:36.487742+00:00",
        "profile": [
          "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"
        ]
      },
      "extension": [
        {
          "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
          "extension": [
            {
              "url": "ombCategory",
              "valueCoding": {
                "system": "urn:oid:2.16.840.1.113883.6.238",
                "code": "1002-5",
                "display": "American Indian or Alaska Native"
              }
            },
            {
              "url": "detailed",
              "valueCoding": {
                "system": "urn:oid:2.16.840.1.113883.6.238",
                "code": "2108-9",
                "display": "Blackfoot Sioux"
              }
            },
            {
              "url": "ombCategory",
              "valueCoding": {
                "system": "urn:oid:2.16.840.1.113883.6.238",
                "code": "2106-3",
                "display": "White"
              }
            },
            {
              "url": "detailed",
              "valueCoding": {
                "system": "urn:oid:2.16.840.1.113883.6.238",
                "code": "2111-3",
                "display": "French"
              }
            },
            {
              "url": "text",
              "valueString": "Mixed"
            }
          ]
        },
        {
          "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity",
          "extension": [
            {
              "url": "ombCategory",
              "valueCoding": {
                "system": "urn:oid:2.16.840.1.113883.6.238",
                "code": "2186-5",
                "display": "Non Hispanic or Latino"
              }
            },
            {
              "url": "text",
              "valueString": "Non Hispanic or Latino"
            }
          ]
        },
        {
          "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex",
          "valueCode": "F"
        }
      ],
      "identifier": [
        {
          "system": "https://terminology.meldrx.com/",
          "value": "159654"
        }
      ],
      "name": [
        {
          "family": "Newman",
          "given": [
            "Alice",
            "Jones"
          ]
        },
        {
          "family": "Newman",
          "given": [
            "Alicia"
          ]
        }
      ],
      "telecom": [
        {
          "system": "phone",
          "value": "+1555-723-1544",
          "use": "home"
        },
        {
          "system": "phone",
          "value": "+1555-777-1234",
          "use": "mobile"
        }
      ],
      "gender": "female",
      "birthDate": "1970-05-01",
      "address": [
        {
          "use": "home",
          "line": [
            "1357 Amber Drive"
          ],
          "city": "Beaverton",
          "state": "OR",
          "postalCode": "97006",
          "country": "US"
        }
      ],
      "communication": [
        {
          "language": {
            "coding": [
              {
                "system": "urn:ietf:bcp:47",
                "code": "en-US",
                "display": "English (United States)"
              }
            ]
          }
        }
      ],
      "managingOrganization": {
        "reference": "Organization/d6c1dd98-be1a-4d02-badc-63a89948d9b1"
      }
    }"#;

    // Parse the JSON into our Rust model
    let patient: Patient = serde_json::from_str(patient_json)?;
    
    // Extract demographics into a more accessible format
    let demographics = extract_demographics(&patient)?;
    
    // Display the demographics information
    println!("Patient Information Summary");
    println!("==========================");
    println!("ID: {}", demographics.id);
    println!("Name: {}", demographics.primary_name);
    println!("Birth Date: {}", demographics.birth_date);
    println!("Gender: {}", demographics.gender);
    println!("Birth Sex: {}", demographics.birth_sex);
    
    println!("\nRace Information:");
    for race in &demographics.race {
        println!("  - {}", race);
    }
    
    println!("\nEthnicity: {}", demographics.ethnicity);
    
    println!("\nContact Information:");
    println!("Addresses:");
    for address in &demographics.addresses {
        println!("  - {}", address);
    }
    
    println!("Phone Numbers:");
    for (use_type, number) in &demographics.phone_numbers {
        println!("  {} phone: {}", use_type, number);
    }
    
    println!("\nPrimary Language: {}", demographics.primary_language);
    
    println!("\nManaging Organization: {}", 
             patient.managing_organization
                   .as_ref()
                   .map_or("None", |org| &org.reference));

    Ok(())
}

// Extract demographic information from the FHIR Patient resource
fn extract_demographics(patient: &Patient) -> Result<PatientDemographics, Box<dyn std::error::Error>> {
    // Extract primary name
    let primary_name = if !patient.name.is_empty() {
        let name = &patient.name[0];
        format!("{}, {}", 
                name.family, 
                name.given.join(" "))
    } else {
        "Unknown".to_string()
    };
    
    // Extract race information
    let mut races = Vec::new();
    if let Some(extensions) = &patient.extension {
        for ext in extensions {
            if ext.url == "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race" {
                if let Some(race_exts) = &ext.extension {
                    for race_ext in race_exts {
                        if race_ext.url == "ombCategory" || race_ext.url == "detailed" {
                            if let Some(coding) = &race_ext.value_coding {
                                races.push(coding.display.clone());
                            }
                        } else if race_ext.url == "text" {
                            if let Some(text) = &race_ext.value_string {
                                races.push(format!("Self-identified as: {}", text));
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Extract ethnicity
    let mut ethnicity = "Unknown".to_string();
    if let Some(extensions) = &patient.extension {
        for ext in extensions {
            if ext.url == "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity" {
                if let Some(eth_exts) = &ext.extension {
                    for eth_ext in eth_exts {
                        if eth_ext.url == "text" {
                            if let Some(text) = &eth_ext.value_string {
                                ethnicity = text.clone();
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Extract birth sex
    let mut birth_sex = "Unknown".to_string();
    if let Some(extensions) = &patient.extension {
        for ext in extensions {
            if ext.url == "http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex" {
                if let Some(code) = &ext.value_code {
                    birth_sex = match code.as_str() {
                        "M" => "Male".to_string(),
                        "F" => "Female".to_string(),
                        other => other.to_string(),
                    };
                    break;
                }
            }
        }
    }
    
    // Extract addresses
    let addresses = patient.address.iter()
        .map(|addr| {
            format!("{}, {}, {} {} ({})",
                addr.line.join(" "),
                addr.city,
                addr.state,
                addr.postal_code,
                addr.country)
        })
        .collect();
    
    // Extract phone numbers
    let mut phone_numbers = HashMap::new();
    for telecom in &patient.telecom {
        if telecom.system == "phone" {
            let use_type = telecom.use.as_ref().unwrap_or(&"unknown".to_string()).to_string();
            phone_numbers.insert(use_type, telecom.value.clone());
        }
    }
    
    // Extract primary language
    let primary_language = if let Some(comms) = &patient.communication {
        if !comms.is_empty() {
            comms[0].language.coding[0].display.clone()
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    };
    
    Ok(PatientDemographics {
        id: patient.id.clone(),
        primary_name,
        birth_date: patient.birth_date.clone(),
        gender: patient.gender.clone(),
        race: races,
        ethnicity,
        birth_sex,
        addresses,
        phone_numbers,
        primary_language,
    })
}