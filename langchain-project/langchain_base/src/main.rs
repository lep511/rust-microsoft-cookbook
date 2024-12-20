mod openai;
use openai::{ChatOpenAI, Message};
// mod gemini;
// use gemini::{ChatGemini, Content, ChatRequest, Part};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = ChatOpenAI::new("gpt-4o-mini");

    let messages = vec![Message {
        role: "user".to_string(),
        content: "Write a haiku about ai".to_string(),
    }];

    let response = model.invoke(messages).await?;

    

    // let model = ChatGemini::new("gemini-1.5-flash")?;
    // let content: Content = {
    //     Content {
    //         role: "user".to_string(),
    //         parts: vec![Part {
    //             text: Some("Which theaters in Mountain View show Barbie movie?".to_string()),
    //             function_call: None,
    //         }],
    //     }
    // };
    
    // let chat_request = ChatRequest {
    //     contents: vec![content],
    //     tools: None,
    //     generation_config: None,
    // };

    // let response = model.invoke(chat_request).await?;
    
    println!("Response: {:?}", response);
    Ok(())
}
