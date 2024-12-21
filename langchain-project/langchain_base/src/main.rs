// mod openai;
// use openai::ChatOpenAI;
// mod gemini;
// use gemini::ChatGemini;
mod anthropic;
use anthropic::ChatAnthropic;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let model = ChatOpenAI::new("gpt-4o-mini")?;
    // let model = ChatAnthropic::new("claude-3-5-sonnet-20241022")?;
    let model = ChatAnthropic::new("claude-3-5-haiku-20241022")?;

    let model = model.with_max_tokens(1024);
    let model = model.with_temperature(0.9);
    let model = model.with_max_tokens(2048);
    // let model = model.with_stream(true);
    let tool_data = json!({
        "name":"get_weather",
        "description":"Get the current weather in a given location",
        "input_schema":{
            "type":"object",
            "properties":{
                "location":{
                    "type":"string",
                    "description":"The city and state, e.g. San Francisco, CA"
                }
            },
            "required":[
                "location"
            ]
        }
    });
    let tools = vec![tool_data];
    let tool_choice = json!({"type": "any"});
    let model = model.with_tools(tools, tool_choice);

    let prompt = "Explain the Pythagorean theorem to a 10-year-old.";
    let response = model.invoke(prompt).await?;

    if let Some(ref candidates) = response.content {
        for candidate in candidates {
            println!("{:?}", candidate.text);
        }
    };
    
    // println!("Response: {:?}", response.content.unwrap_or_default());
    
    // let model = ChatGemini::new("gemini-1.5-flash")?;
    
    // let model = model.with_temperature(0.9);
    // let model = model.with_max_tokens(2048);

    // let prompt = "Explain the Pythagorean theorem to a 10-year-old.";
    // let response = model.invoke(prompt).await?;

    // if let Some(candidates) = response.candidates {
    //     for candidate in candidates {
    //         if let Some(content) = candidate.content {
    //             for part in content.parts {
    //                 if let Some(text) = part.text {
    //                     println!("{}", text);
    //                 }
    //             }
    //         }
    //     }
    // };

    Ok(())
}
