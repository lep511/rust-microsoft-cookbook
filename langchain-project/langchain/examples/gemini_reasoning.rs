#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use env_logger::Env;

async fn medical_example() -> Result<(), Box<dyn std::error::Error>> {

    let prompt = "You are a helpful assistant designed to validate the quality of medical datasets. \
            You will be given a single row of medical data, and your task is to determine \
            whether the data is valid. \
            \
            - Carefully analyze the data for any inconsistencies, contradictions, missing values, \
            or implausible information.\n \
            - Consider the logical relationships between different fields (e.g., treatments should \
            be appropriate for the diagnoses, medications should not conflict with allergies, \
            lab results should be consistent with diagnoses, etc.).\n \
            - Use your general medical knowledge to assess the validity of the data.\n \
            - Focus solely on the information provided without making assumptions beyond the given data.\n \
        \
        **Return only a JSON object** with the following two properties: \
        \
        - `\"is_valid\"`: a boolean (`true` or `false`) indicating whether the data is valid.\n \
        - `\"issue\"`: if `\"is_valid\"` is `false`, provide a brief explanation of the issue; \
        if `\"is_valid\"` is `true`, set `\"issue\"` to `null`. \
        \
        Both JSON properties must always be present. \
        \
        Do not include any additional text or explanations outside the JSON object. \
        \
        \n\n\nMEDICAL DATA:";

    let file_path = Path::new("tests/files/medicalData.csv");
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Declare header_trim as Vec to own the data
    let header_trim: Vec<String>;
    
    // Read the first line and process it
    if let Some(line) = reader.lines().next() {
        let header = line?;
        let parts: Vec<String> = header
            .split(',')
            .map(|s| s.to_string())
            .collect();
        header_trim = parts[0..9].to_vec();
    } else {
        header_trim = Vec::new(); // Handle empty file case
    }

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let line_21: Vec<String>;

    // Read line 21
    if let Some(line) = reader.lines().nth(20) {
        let line = line?;
        let parts: Vec<String> = line
            .split(',')
            .map(|s| s.to_string())
            .collect();
        line_21 = parts[0..9].to_vec();
    } else {
        line_21 = Vec::new(); // Handle empty file case
    }

    let mdata: Vec<_> = header_trim.iter().zip(line_21.iter()).collect();
    
    let mdata_string = mdata.iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join("\n");

    let f_prompt = format!("{}\n\n{}", prompt, mdata_string);

    let llm = ChatGemini::new("gemini-2.0-flash-thinking-exp-01-21");
    let response = llm
        .invoke(&f_prompt)
        .await?;

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text.replace("```json", "").replace("```", ""));
                    }
                }
            }
        }
    };
    
    Ok(())
}

async fn geometric_example() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-thinking-exp-01-21");

    let prompt = "What is the geometric monthly fecal coliform mean of a \
                  distribution system with the following FC counts: \
                  24, 15, 7, 16, 31 and 23? The result will be inputted \
                  into a NPDES DMR, therefore, round to the nearest whole number. \
                  Response at the end with SOLUTION: number(integer)";

    // NOTE: the correct answer is 18
   
    let response = llm
        .with_temperature(0.7)
        .with_top_k(64)
        .with_top_p(0.95)
        .with_max_tokens(8192)
        .invoke(prompt)
        .await?;
    
    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                    }
                }
            }
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Gemini 2.0 Flash Thinking Mode is an experimental model that's 
    // trained to generate the "thinking process" the model goes through 
    // as part of its response. As a result, Thinking Mode is capable of 
    // stronger reasoning capabilities in its responses than 
    // the base Gemini 2.0 Flash model.
    // https://ai.google.dev/gemini-api/docs/thinking-mode

    // geometric_example().await?;
    medical_example().await?;

    Ok(())
}