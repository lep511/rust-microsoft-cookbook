use langchain_base::gemini::ChatGemini;
use std::fs::File;
use std::io::{Write, Read};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::json;
use std::path::Path;

static GEMINI_MODEL: &str = "gemini-2.0-flash-exp";
static GEMINI_MODEL_THINK: &str = "gemini-2.0-flash-thinking-exp";
static GEMINI_MODEL_CACHE: &str = "gemini-1.5-flash-001";

#[tokio::test]
async fn gemini_simple_shot() {
    let llm = match ChatGemini::new(GEMINI_MODEL) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);
    let llm = llm.with_system_prompt("You are a helpful assistant.");
    let prompt = "Only say Simple test";

    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };
    
    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        let text_l = text.to_lowercase();
                        let possible_values = vec![
                            "simple test",
                            "simple test\n",
                            "simple test.\n",
                            "simple test."
                        ];

                        // Count how many matches we have
                        let match_count = possible_values.iter()
                            .filter(|&&val| val == text_l)
                            .count();
                        assert_eq!(
                            match_count, 
                            1, 
                            "Text '{}' did not match any of the expected values", 
                            text_l
                        );
                    }
                }
            }
        }
    };
}

#[tokio::test]
async fn gemini_compare_images() {
    let llm = match ChatGemini::new(GEMINI_MODEL) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };

    // Read first image into a byte vector
    let mut file_01 = match File::open("tests/files/image01.jpg") {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e),
    };

    let mut buffer_01 = Vec::new();
    match file_01.read_to_end(&mut buffer_01) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e),
    }
    
    // Read second image into a byte vector
    let mut file_02 = match File::open("tests/files/image03.png") {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e),
    };

    let mut buffer_02 = Vec::new();
    match file_02.read_to_end(&mut buffer_02) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e),
    }

    // Convert to base64
    let base64_string_01 = STANDARD.encode(&buffer_01);
    let base64_string_02 = STANDARD.encode(&buffer_02);

    let llm = llm.with_image(
        &base64_string_01,
        "image/jpeg",
    );
    let llm = llm.with_image(
        &base64_string_02,
        "image/png",
    );
    let prompt = "Compare the two pictures provided. \
            Which of the images shows an office with people working, \
            the first or the second? Just answer: FIRST or SECOND.";

    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        let text_l = text.to_lowercase();
                        let possible_values = vec![
                            "first\n",
                            "first",
                        ];

                        // Count how many matches we have
                        let match_count = possible_values.iter()
                            .filter(|&&val| val == text_l)
                            .count();
                        assert_eq!(
                            match_count, 
                            1, 
                            "Text '{}' did not match any of the expected values", 
                            text_l
                        );
                    }
                }
            }
        }
    };
}

#[tokio::test]
async fn gemini_upload_image() {
    let llm = match ChatGemini::new(GEMINI_MODEL) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };

    let file_path = "tests/files/image03.png";
    let file_mime = "image/png";

    let llm = match llm.media_upload(file_path, file_mime).await {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };

    let prompt = "You can say if this image is real or fantastic. \
                Just respond with one word: REAL or FANTASTIC.";

    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        let text_l = text.to_lowercase();
                        let possible_values = vec![
                            "fantastic\n",
                            "fantastic",
                            "fantastic.\n",
                            "fantastic.",
                        ];

                        // Count how many matches we have
                        let match_count = possible_values.iter()
                            .filter(|&&val| val == text_l)
                            .count();
                        assert_eq!(
                            match_count, 
                            1, 
                            "Text '{}' did not match any of the expected values", 
                            text_l
                        );
                    }
                }
            }
        }
    };
}

#[tokio::test]
async fn gemini_multiple_turns() {
    let llm = match ChatGemini::new(GEMINI_MODEL) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };

    let prompt = "Please answer the following question with only \"yes\" or \"no\": Is the sky blue during a clear day?";

    let response = match llm.clone().invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    let mut response_model = String::new();

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        response_model = text.to_string();
                    }
                }
            }
        }
    };

    let chat_history = match response.chat_history {
        Some(chat_history) => chat_history,
        None => {
            println!("No chat history");
            Vec::new()
        }
    };

    let llm = llm.with_chat_history(chat_history);
    let llm = llm.with_assistant_response(&response_model);

    let prompt = "And during the night? Answer only with \"yes\" or \"no\".";

    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    println!("\n#### Multiple Turn 2 ####");
    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        let text_l = text.to_lowercase();
                        let possible_values = vec![
                            "no\n",
                            "mo"
                        ];

                        // Count how many matches we have
                        let match_count = possible_values.iter()
                            .filter(|&&val| val == text_l)
                            .count();
                        assert_eq!(
                            match_count, 
                            1, 
                            "Text '{}' did not match any of the expected values", 
                            text_l
                        );
                    }
                }
            }
        }
    };
}

#[tokio::test]
async fn gemini_upload_cache() {
    let llm = match ChatGemini::new(GEMINI_MODEL_CACHE) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };

    // Read first file into a byte vector
    let mut file_01 = match File::open("tests/files/apolo11.txt") {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e),
    };
    let mut buffer_01 = Vec::new();
    match file_01.read_to_end(&mut buffer_01) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e),
    }
    
    // Convert to base64
    let base64_string_01 = STANDARD.encode(&buffer_01);

    let system_instruction = "You are an expert at analyzing transcripts.";
    
    let cache_url = match llm.clone().cache_upload(
            base64_string_01,
            "text/plain",
            system_instruction
        ).await {
            Ok(cache_url) => cache_url,
            Err(e) => panic!("Error: {}", e),
    };

    assert!(
        cache_url.starts_with("cachedContents/"), 
        "Cache url '{}' doesn't start with 'cachedContents/'", 
        cache_url
    );

    let llm = llm.with_cached_content(cache_url);
    let prompt = "Is this a transcript of Apollo 11? Answer only with yes or no.";

    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        let _text = text.to_lowercase().replace("\n", "");
                        let _text = _text.trim();
                        assert_eq!(_text, "yes");
                    }
                }
            }
        }
    };
}

#[tokio::test]
async fn gemini_response_schema() {

    let llm = match ChatGemini::new(GEMINI_MODEL) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };

    let response_schema = json!({
        "type":"object",
        "properties":{
            "response":{
                "type":"string",
                "enum":[
                    "yes",
                    "no"
                ]
            }
        }
    });
    let llm = llm.with_response_schema(response_schema);
    let prompt = "Please answer the following question with only yes or no: Is the sky blue during a clear day?";

    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        let json_data = match serde_json::from_str::<serde_json::Value>(&text) {
                            Ok(json) => json,
                            Err(e) => panic!("Error: {}", e),
                        };
                        assert_eq!(json_data["response"], "yes");
                    }
                }
            }
        }
    };
}

#[tokio::test]
async fn gemini_think_mode() {

    let llm = match ChatGemini::new(GEMINI_MODEL_THINK) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };

    let prompt = "What is the geometric monthly fecal coliform mean of a \
                  distribution system with the following FC counts: \
                  24, 15, 7, 16, 31 and 23? The result will be inputted \
                  into a NPDES DMR, therefore, round to the \
                  nearest whole number.";

    let llm = llm.with_temperature(1.0);
    let llm = llm.with_top_k(64);
    
    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    let mut result = String::from("");

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        result.push_str(&text);
                    }
                }
            }
        }
    };

    // Save result to file.md
    let file_path = "tests/output/test-gemini-think-mode.md";
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't create {}", e),
    };

    match file.write_all(result.as_bytes()) {
        Ok(_) => {},
        Err(e) => panic!("Error saving file: {}", e),
    }

    // Check if the file exists
    let path = Path::new(file_path);
    assert!(path.exists(), "File was not created!");

    // Check if the file has content
    assert!(!result.is_empty(), "File is empty!");
}