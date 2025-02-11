// Structured Data Extraction: Social Determinants of Health from Clinical Notes with Groq API's Json Mode
//
// Since the dawn of the Electronic Health Record, deriving meaningful insights about the social determinants
// of health of a patient population has been the holy grail of healthcare analytics. While discrete clinical
// data (vitals, lab results, diagnoses, etc) is well understood, social determinants - things like financial
// insecurity, which can determine patient outcomes and barriers to care as much as the patient's clinical
// chart - are often hidden in clinical notes and unused by analytics departments. While some providers code
// social determinants using Z codes, these are often too inconsistently documented and many risk models
// seeking to add a social determinant score will simply default to using zip code as a crude proxy. With
// the emergence of Large Language Models, AI has the ability to extract and structure meaningful insights
// from free-text clinical notes at scale, enabling more effective patient outreach, better risk modeling
// and a more robust understanding of a patient population as a whole.
//
// This example shows how we can use Groq API's JSON mode feature to extract social determinants of health
// from fake clinical notes, structure them into a neat table that can be used for analytics and load them
// into MongoDB. With JSON mode, we can return structured data from the chat completion in a pre-defined
// format, making it a great feature for structuring unstructrued data. We will read in each note, ask the
// LLM to determine if certain social determinant features are met, output structured data and load it into
// a database to be incorporated with the rest of our clinical data marts.

#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use langchain::compatible::libs::ChatResponse;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use env_logger::Env;

#[derive(Debug, Serialize, Deserialize)]
struct SocialDeterminants {
    employment_status: Option<String>,
    financial_stress: Option<bool>,
    housing_insecurity: Option<bool>,
    neighborhood_unsafety: Option<bool>,
    food_insecurity: Option<bool>,
    education_level: Option<String>,
    transportation_inaccessibility: Option<bool>,
    social_isolation: Option<bool>,
    health_insurance_inadequacy: Option<bool>,
    skipped_care_due_to_cost: Option<bool>,
    marital_status: Option<String>,
    language_barrier: Option<bool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let prompt = "You are a medical coding API specializing in social determinants of health that responds in JSON. \
            Your job is to extract structured SDOH data from an unstructured clinical note and output the structured data in JSON. \
            The JSON schema should include: \
            { \
                \"employment_status\": \"string (categorical: 'Unemployed', 'Part-time', 'Full-time', 'Retired')\", \
                \"financial_stress\": \"boolean (TRUE if the patient mentions financial difficulties)\", \
                \"housing_insecurity\": \"boolean (TRUE if the patient does not live in stable housing conditions)\", \
                \"neighborhood_unsafety\": \"boolean (TRUE if the patient expresses concerns about safety)\", \
                \"food_insecurity\": \"boolean (TRUE if the patient does not have reliable access to sufficient food)\", \
                \"education_level\": \"string (categorical: 'None', 'High School', 'College', 'Graduate')\", \
                \"transportation_inaccessibility\": \"boolean (TRUE if the patient does not have reliable transportation to healthcare appointments)\", \
                \"social_isolation\": \"boolean (TRUE if the patient mentions feeling isolated or having a lack of social support)\", \
                \"health_insurance_inadequacy\": (boolean: TRUE if the patient's health insurance is insufficient), \
                \"skipped_care_due_to_cost\": \"boolean (TRUE if the patient mentions skipping medical tests or treatments due to cost)\", \
                \"marital_status\": \"string (categorical: 'Single', 'Married', 'Divorced', 'Widowed')\", \
                \"language_barrier\": \"boolean (TRUE if the patient has language barriers to healthcare access)\" \
            } \
            Use information from following clinical note to construct the proper JSON output: \
            \n
            ";   
    
    for i in 1..=5 {
        let base_url = "https://api.groq.com/openai/v1/chat/completions";
        let model = "llama-3.3-70b-specdec";
        let llm = ChatCompatible::new(base_url, model);
        let file_txt = format!("tests/files/note_medical{}.txt", i);
        let file_json = format!("tests/files/note_medical{}.json", i);
        let contents = fs::read_to_string(file_txt)?;
        let format_prompt = format!("{}\n{}", prompt, contents);
        let response: ChatResponse = llm.invoke(&format_prompt).await?;

        println!("\n#### Example Groc Medical Data ####");
        match response.choices {
            Some(candidates) => {
                for candidate in candidates {
                    #[allow(irrefutable_let_patterns)]
                    if let message = candidate.message {
                        let json_str = message.content.lines()
                            .skip(1) // Skip ```json
                            .take_while(|line| !line.starts_with("```")) // Take until closing ```
                            .collect::<Vec<&str>>()
                            .join("\n");
                        
                        let social_data: SocialDeterminants = serde_json::from_str(&json_str)
                            .expect("Failed to parse JSON");
                        
                        let json_string = serde_json::to_string_pretty(&social_data)
                            .expect("Failed to serialize JSON");

                        // Save to file
                        let mut file = fs::File::create(file_json.clone())?;
                        file.write_all(json_string.as_bytes())?;
                        println!("Data saved to {}", file_json);
                    }
                }
            }
            None => println!("No response choices available"),
        }
    };
    
    Ok(())
}