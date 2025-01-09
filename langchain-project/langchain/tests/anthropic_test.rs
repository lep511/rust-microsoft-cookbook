use langchain::anthropic::ChatAnthropic;
use serde_json::json;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::general_purpose::STANDARD};

static ANTHROPIC_MODEL: &str = "claude-3-5-sonnet-20241022";

#[tokio::test]
async fn anthropic_simple_shot() {
    let llm = match ChatAnthropic::new(ANTHROPIC_MODEL) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };

    let llm = llm.with_max_tokens(1024);
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_tokens(2048);
    let llm = llm.with_timeout_sec(30);
    
    let llm = llm.with_system_prompt("You are a helpful assistant.");
    let prompt = "Only say Simple test";

    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    if let Some(candidates) = response.content {
        for candidate in candidates {
            match candidate.text {
                Some(message) => {
                    let text_l = message.to_lowercase();
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
                None => panic!("No response candidates available"),
            }
        }
    };
}

#[tokio::test]
async fn anthropic_function() {
    let llm = match ChatAnthropic::new(ANTHROPIC_MODEL) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };
    
    let tool_data = json!({
        "name": "get_stock_price",
        "description": "Retrieves the current stock price for a given ticker symbol. \
                        The ticker symbol must be valid for a publicly traded company \
                        on a major US stock exchange like NYSE or NASDAQ. \
                        The tool will return the latest trade price in USD. \
                        It should be used when the user asks about the current \
                        or most recent price of a specific stock. \
                        It will not provide any other information \
                        about the stock or company.",
        "input_schema": {
        "type": "object",
        "properties": {
            "ticker": {
            "type": "string",
            "description": "The stock ticker symbol, e.g. AAPL for Apple Inc."
            }
        },
        "required": ["ticker"]
        }
    });

    let tools = vec![tool_data];
    let tool_choice = json!({"type": "auto"});
    let llm = llm.with_tools(tools, tool_choice);

    let prompt = "How much is Tesla stock trading for? Before answering, explain your reasoning step-by-step in tags.";
    
    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    if let Some(contents) = &response.content {
        let function_content = &contents[1];
        assert_eq!(function_content.name, Some("get_stock_price".to_string()));
        assert_eq!(function_content.content_type, Some("tool_use".to_string()));
        if let Some(input) = &function_content.input {
            assert_eq!(input["ticker"], "TSLA");
        } else {
            panic!("Input should not be None");
        }
    } else {
        panic!("Content should not be None");
    }
}

#[tokio::test]
async fn anthropic_images() {
    let llm = match ChatAnthropic::new(ANTHROPIC_MODEL) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };

    // Read first image into a byte vector
    let mut file_01 = match File::open("tests/files/image01.jpg") {
        Ok(file) => file, 
        Err(e) => panic!("Error: {}", e)
    };
    let mut buffer_01 = Vec::new();
    match file_01.read_to_end(&mut buffer_01) {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e)
    };

    // Read second image into a byte vector
    let mut file_02 = match File::open("tests/files/image03.png") {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e)
    };
    let mut buffer_02 = Vec::new();
    match file_02.read_to_end(&mut buffer_02) {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e)
    };

    // Convert to base64
    let base64_string_01 = STANDARD.encode(&buffer_01);
    let base64_string_02 = STANDARD.encode(&buffer_02);

    let llm = llm.with_image_jpeg(&base64_string_01);
    let llm = llm.with_image_png(&base64_string_02);

    let prompt = "Compare the two pictures provided. \
        Which of the images shows an office with people working, \
        the first or the second? Just answer: FIRST or SECOND.";

    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    if let Some(candidates) = response.content {
        for candidate in candidates {
            match candidate.text {
                Some(message) => {
                    let text_l = message.to_lowercase();
                    let possible_values = vec![
                        "first",
                        "first\n",
                        "first.\n",
                        "first."
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
                None => panic!("No response candidates available"),
            }
        }
    };
}

#[tokio::test]
async fn anthropic_multiple_turns() {
    let llm = match ChatAnthropic::new(ANTHROPIC_MODEL) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };

    let prompt = "Please answer the following question with only \"yes\" or \"no\": Is the sky blue during a clear day?";

    let response = match llm.clone().invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    let mut response_model = String::new();

    if let Some(candidates) = response.content {
        for candidate in candidates {
            match candidate.text {
                Some(text) => {
                    response_model = text.to_string();
                }
                None => panic!("No response candidates available"),
            }

        }
    }

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

    if let Some(candidates) = response.content {
        for candidate in candidates {
            match candidate.text {
                Some(message) => {
                    let text_l = message.to_lowercase();
                    let possible_values = vec![
                        "no",
                        "no\n",
                        "no.\n",
                        "no."
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
                None => panic!("No response candidates available"),
            }
        }
    };
}
