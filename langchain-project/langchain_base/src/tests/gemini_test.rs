use langchain_base::gemini::ChatGemini;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::general_purpose::STANDARD};

static GEMINI_MODEL: &str = "gemini-2.0-flash-exp";

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

    let file_url: String = match llm.clone()
        .media_upload(file_path, file_mime)
        .await {
            Ok(file_url) => file_url,
            Err(e) => panic!("Error: {}", e),
    };

    let llm = llm.with_image_url(
        file_url,
        file_mime,
    );
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