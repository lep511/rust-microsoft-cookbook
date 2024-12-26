#[allow(dead_code)]
use crate::xai::{ChatXAI, ChatResponse};
use std::fs;
use std::io::Write;

#[allow(dead_code)]
pub async fn sample() -> Result<(), Box<dyn std::error::Error>> {

    let system_prompt = "You are a highly experienced medical professional with a specialty in translating \
                complex patient histories into concise, actionable summaries. Your role is to analyze patient records, \
                identify critical information, and present it in a clear, structured format that aids in diagnosis and \
                treatment planning. Your summaries are invaluable for busy healthcare providers who need quick insights \
                into a patient's medical history before appointments.";   
    
    let base_prompt_file = "src/examples/files/medical_prompt.txt";
    let base_prompt = fs::read_to_string(base_prompt_file)?;
    println!("\n#### Example Groc Medical Prompt ####");

    for i in 1..=5 {
        let llm = ChatXAI::new("grok-2-1212")?;
        let llm = llm.with_system_prompt(system_prompt);
        let llm = llm.with_max_tokens(8092);
        let file_txt = format!("src/examples/files/patient_record{}.txt", i);
        let file_json = format!("src/examples/files/patient_record_result{}.json", i);
        let contents = fs::read_to_string(file_txt.clone())?;
        let format_prompt = format!("{}\n{}", base_prompt, contents);

        println!("\nProcessing file... {}", file_txt);
        let response: ChatResponse = llm.invoke(&format_prompt).await?;

        match response.choices {
            Some(candidates) => {
                for candidate in candidates {
                    #[allow(irrefutable_let_patterns)]
                    if let message = candidate.message {
                        let json_str = match message.content {
                            Some(content) => content.lines()
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