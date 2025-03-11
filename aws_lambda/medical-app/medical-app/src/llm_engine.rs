use crate::gemini::chat::ChatGemini;
use lambda_http::tracing::info;
// use serde_json::{Value, json};
use chrono::prelude::*;

pub async fn manage_medication(
    medical_data: &str,
) -> Result<String, Box<dyn std::error::Error>> {

    info!("Medical data: {:?}", medical_data);
    let llm = ChatGemini::new("gemini-2.0-flash");

    let today = Local::now();
    let today_fmt = today.format("%B %-d, %Y");

    let prompt = format!(
        "Identify and list the medicines currently consumed by the patient from the provided FHIR data, assuming today's date is {}. \
        \n \
        # Output Format \
        \n \
        - A list of medicines actively consumed by the patient. \
        - Ensure clarity and use proper FHIR data interpretive methods. \
        \n \
        # Notes \
        \n \
        - Consider active prescriptions and consumption statuses only. Show separately the medicines taken by the patient in the past.\n \
        - If the effectiveDateTime field appears instead of effectivePeriod, it means that the patient is currently taking the medicine.\n \
        - Displays all dates in format YYYYY/mm/dd\n \
        - Always show the start and end dates of each medicine, if applicable.\n \
        \n\n \
        # FHIR Data \n \
        {}", 
        today_fmt,
        medical_data,
    );

    let response = llm
        .invoke(&prompt)
        .await?;

    let mut med_result = String::new();

    response.candidates.as_ref().map(|candidates| {
        candidates.iter().for_each(|candidate| {
            candidate.content.as_ref().map(|content| {
                content.parts.iter().for_each(|part| {
                    part.text.as_ref().map(|text| {
                        med_result.push_str(text);
                    });
                });
            });
        });
    });

    Ok(med_result.replace("*", "").replace("#", ""))
}