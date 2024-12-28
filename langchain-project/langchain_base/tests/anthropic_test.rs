use langchain_base::anthropic::ChatAnthropic;

static ANTHROPIC_MODEL: &str = "claude-3-5-haiku-20241022";

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
