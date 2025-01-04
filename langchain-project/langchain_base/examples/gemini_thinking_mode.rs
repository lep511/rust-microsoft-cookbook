#[allow(dead_code)]
use langchain_base::gemini::ChatGemini;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Gemini 2.0 Flash Thinking Mode is an experimental model that's 
    // trained to generate the "thinking process" the model goes through 
    // as part of its response. As a result, Thinking Mode is capable of 
    // stronger reasoning capabilities in its responses than 
    // the base Gemini 2.0 Flash model.
    // https://ai.google.dev/gemini-api/docs/thinking-mode

    let llm = ChatGemini::new("gemini-2.0-flash-thinking-exp")?;

    let prompt = "What is the geometric monthly fecal coliform mean of a \
                  distribution system with the following FC counts: \
                  24, 15, 7, 16, 31 and 23? The result will be inputted \
                  into a NPDES DMR, therefore, round to the nearest whole number. \
                  Response at the end with SOLUTION: number(integer)";

    // NOTE: the correct answer is 18
   
    let response = llm.invoke(prompt).await?;

    println!("\n#### Example Gemini Thinking Mode ####");
    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                    }
                }
            }
        }
    };

    Ok(())
}