// mod openai;
// use openai::{ChatOpenAI, Message};
mod gemini;
use gemini::{ChatGemini, Content, ChatRequest, Part};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let model = ChatOpenAI::new("gpt-4o-mini")?;

    // let messages = vec![Message {
    //     role: "user".to_string(),
    //     content: "Explain the Pythagorean theorem to a 10-year-old.".to_string(),
    // }];

    // let model = model.with_max_tokens(1024);

    // let response = model.invoke(messages).await?;
    // println!("Response: {:?}", response);

    

    let model = ChatGemini::new("gemini-2.0-flash-thinking-exp")?;
    let content: Content = {
        Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: Some("Explain the Pythagorean theorem to a 10-year-old.".to_string()),
                function_call: None,
            }],
        }
    };
    
    let chat_request = ChatRequest {
        contents: vec![content],
        tools: None,
        generation_config: None,
    };
    let response = model.invoke(chat_request).await?;
    println!("Response: {:?}", response);

    Ok(())
}