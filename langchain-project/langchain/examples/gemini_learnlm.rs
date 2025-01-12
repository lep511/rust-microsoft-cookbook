#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use langchain::gemini::libs::{Part, Content};
use std::io::{self, Write, Read};
use std::path::Path;
use std::fs::File;

const FILE_PATH: &str = "tests/output/chat_history.json";

async fn save_chat_history(chat_history: Vec<Content>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(&chat_history)?;
    let mut file = match File::create(FILE_PATH) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating file: {}", e);
            return Err(Box::new(e));
        }
    };

    match file.write_all(json.as_bytes()) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            println!("Error writing to file: {}", e);
            Err(Box::new(e))
        }
    }
}

async fn read_chat_history() -> Result<Vec<Content>, Box<dyn std::error::Error>> {
    let mut file = match File::open(FILE_PATH) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return Err(Box::new(e));
        }
    };

    let mut json = String::new();
    match file.read_to_string(&mut json) {
        Ok(_) => {
            let chat_history: Vec<Content> = serde_json::from_str(&json)?;
            Ok(chat_history)
        }
        Err(e) => {
            println!("Error reading from file: {}", e);
            Err(Box::new(e))
        }
    }
}

async fn check_chat_history_file_exist() -> bool {
    let path = Path::new(FILE_PATH);
    if path.exists() {
        true
    } else {
        false
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("learnlm-1.5-pro-experimental")?;

    let system_prompt = "You are a tutor helping a student prepare for a test. If not provided by the \
                    student, ask them what subject and at what level they want to be tested on. \
                    Then, \
                    \
                    *   Generate practice questions. Start simple, then make questions more \
                        difficult if the student answers correctly. \
                    *   Prompt the student to explain the reason for their answer choice. Do not \
                        debate the student. \
                    *   **After the student explains their choice**, affirm their correct answer or \
                        guide the student to correct their mistake. \
                    *   If a student requests to move on to another question, give the correct \
                        answer and move on. \
                    *   If the student requests to explore a concept more deeply, chat with them to \
                        help them construct an understanding. \
                    *   After 5 questions ask the student if they would like to continue with more \
                        questions or if they would like a summary of their session. If they ask for \
                        a summary, provide an assessment of how they have done and where they should \
                        focus studying.";

    let content = Content {
        role: "user".to_string(),
        parts: vec![Part {
            text: Some("I want to learn about the history of the United States.".to_string()),
            function_call: None,
            function_response: None,
            inline_data: None,
            file_data: None,
        }],
    };
    
    let mut chat_history = vec![content];
    let mut prompt = String::new();

    if check_chat_history_file_exist().await {
        println!("Chat history file exists");
        chat_history = read_chat_history().await?;
        prompt = "Another question.".to_string();
        
    } else {
        prompt = "Can you provide me with some practice questions?".to_string();
    }

    let mut response = llm.clone()
        .with_system_prompt(system_prompt)
        .with_chat_history(chat_history)
        .with_top_k(64)   
        .invoke(&prompt)
        .await?;

    loop {
        let mut response_model = String::new();
        
        if let Some(candidates) = response.candidates {
            for candidate in candidates {
                if let Some(content) = candidate.content {
                    for part in content.parts {
                        if let Some(text) = part.text {
                            println!("{}", text);
                            response_model = text.clone();
                        }
                    }
                }
            }
        };

        let chat_history: Vec<Content> = match response.chat_history {
            Some(chat_history) => chat_history,
            None => {
                println!("No chat history");
                Vec::new()
            }
        };

        save_chat_history(chat_history.clone()).await?;

        let response_part = Part {
            text: Some(response_model),
            function_call: None,
            function_response: None,
            inline_data: None,
            file_data: None,
        };

        let mut user_prompt = String::new();
        print!("Enter response: ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to flush stdout: {}", e);
                break
            }
        }

        io::stdin()
            .read_line(&mut user_prompt)
            .expect("Failed to read line");
                    
        response = llm.clone()
            .with_chat_history(chat_history)
            .with_assistant_response(vec![response_part])
            .with_system_prompt(system_prompt)
            .invoke(&user_prompt)
            .await?;
    }

    Ok(())
}