#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use env_logger::Env;
use langchain::gemini::libs::{SafetySetting, HarmCategory, HarmBlock};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatGemini::new("gemini-1.5-pro");

    let prompt = "I support Martians Soccer Club and I think Jupiterians Football Club sucks! Write a ironic phrase about them.";

    let safety_settings = vec![
        SafetySetting {
            category: HarmCategory::HarmCategoryHarassment,
            threshold: HarmBlock::BlockLowAndAbove,
        },
        SafetySetting {
            category: HarmCategory::HarmCategoryHateSpeech,
            threshold: HarmBlock::BlockMediumAndAbove,
        }
    ];
    
    let response = llm
        .with_safety_settings(safety_settings)
        .invoke(prompt)
        .await?;

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                    }
                }
            } else if let Some(finish_reason) = candidate.finish_reason {
                println!("Finish reason: {:?}", finish_reason);
            }
        }
    };

    Ok(())
}