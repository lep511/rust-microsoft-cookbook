use langchain::openai::ChatOpenAI;
use serde_json::json;

static OPENAI_MODEL: &str = "gpt-4o-mini";

#[tokio::test]
async fn openai_simple_shot() {
    let llm = match ChatOpenAI::new(OPENAI_MODEL) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };
    let llm = llm.with_temperature(0.9);
    let llm = llm.with_max_completion_tokens(2048);
    let llm = llm.with_timeout_sec(30);
    let llm = llm.with_presence_penalty(1.5);
    let llm = llm.with_frequency_penalty(1.5);
    let llm = llm.with_n_completion(1);

    let llm = llm.with_top_p(0.4); // Recommend altering top_p with temperature but not both.

    let llm = llm.with_system_prompt("You are a helpful assistant.");
    let prompt = "Only say Simple test";

    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let message = candidate.message {
                    let text_l = message
                        .content
                        .expect("Cannot read message")
                        .to_lowercase();

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
        None => panic!("No response choices available"),
    };
}

#[tokio::test]
async fn openai_functions() {
    let llm = match ChatOpenAI::new(OPENAI_MODEL) {
        Ok(llm) => llm,
        Err(e) => panic!("Error: {}", e),
    };

    let llm = llm.with_system_prompt("Don't make assumptions about what values to plug into functions. Ask for clarification if a user request is ambiguous.");

    let weather_function = json!( {
        "type": "function",
        "function": {
            "name": "get_current_weather",
            "description": "Get the current weather in a given location",
            "parameters": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"],
                        "default": "celsius"
                    }
                },
                "required": ["location"]
            }
        }
    });

    let llm = llm.with_tools(vec![weather_function]);

    let tool_choice = json!({"type": "function", "function": {"name": "get_current_weather"}});
    let llm = llm.with_tool_choice(tool_choice);
    
    let prompt = "What is the weather like in Boston today?";
    let response = match llm.invoke(prompt).await {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e),
    };

    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let message = candidate.message {
                    match message.tool_calls {
                        Some(tool_calls) => {
                            let value = tool_calls[0].clone();
                            assert_eq!(value["function"]["name"], "get_current_weather");
                            assert_eq!(value["function"]["arguments"], "{\"location\":\"Boston, MA\"}");
                        },
                        None => panic!("No tool_calls in message"),
                    };                 
                }
            }
        }
        None => panic!("No response choices available"),
    };
}
