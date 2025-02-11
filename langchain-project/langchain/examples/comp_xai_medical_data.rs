#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use langchain::compatible::libs::ChatResponse;
use std::fs;
use std::io::Write;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://api.x.ai/v1/chat/completions";
    let model = "grok-2-latest";
    
    let system_prompt = "You are a highly experienced medical professional with a specialty in translating \
                complex patient histories into concise, actionable summaries. Your role is to analyze patient records, \
                identify critical information, and present it in a clear, structured format that aids in diagnosis and \
                treatment planning. Your summaries are invaluable for busy healthcare providers who need quick insights \
                into a patient's medical history before appointments.";   
    
    let base_prompt_file = "tests/files/medical_prompt.txt";
    let base_prompt = fs::read_to_string(base_prompt_file)?;

    for i in 1..=5 {
        let llm = ChatCompatible::new(base_url, model);

        let file_txt = format!("tests/files/patient_record{}.txt", i);
        let file_json = format!("tests/files/patient_record_result{}.json", i);
        let contents = fs::read_to_string(file_txt.clone())?;
        let format_prompt = format!("{}\n{}", base_prompt, contents);

        println!("\nProcessing file... {}", file_txt);
        
        let response: ChatResponse = llm
            .with_system_prompt(system_prompt)
            .with_max_tokens(8092)
            .invoke(&format_prompt)
            .await?;

        match response.choices {
            Some(candidates) => {
                for candidate in candidates {
                    #[allow(irrefutable_let_patterns)]
                    if let message = candidate.message {
                        let json_str = match message.unwrap().content {
                            Some(content) => content
                                .lines()
                                .skip(1) // Skip ```json
                                .take_while(|line| !line.starts_with("```")) // Take until closing ```
                                .collect::<Vec<&str>>()
                                .join("\n"),
                            None => "".to_string(),
                        };                     
                        // Save to file
                        let mut file = fs::File::create(file_json.clone())?;
                        file.write_all(json_str.as_bytes())?;
                        println!("Data saved to {}", file_json);
                    }
                }
            }
            None => println!("No response choices available"),
        }
    };
    
    Ok(())
}