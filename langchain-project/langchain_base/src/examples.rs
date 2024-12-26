#[allow(dead_code)]
mod anthropic_base;
mod anthropic_function_gw;
mod anthropic_function_gsp;
mod anthropic_code_execution;
mod anthropic_image;
mod anthropic_complex;
mod gemini_base;
mod gemini_thinking_mode;
mod openai_base;
mod openai_multiple_turns;
mod openai_functions;
mod groc_base;
mod groc_medical_data;
mod groc_multiple_turns;
mod xai_base;
mod xai_medical_prompt;
mod xai_multiple_turns;
mod xai_functions;

#[allow(dead_code)]
pub enum Models {
    Anthropic,
    OpenAI,
    Gemini,
    Groc,
    Xai,
}

pub(crate)async fn all_examples(model: Models) -> Result<(), Box<dyn std::error::Error>> {
    
    match model {
        Models::Anthropic => {
            // anthropic_base::sample().await?;
            // anthropic_complex::sample().await?;
            // anthropic_function_gw::sample().await?;
            anthropic_function_gsp::sample().await?;
            // anthropic_code_execution::sample().await?;
            // anthropic_image::sample().await?;
        }
        Models::OpenAI => {
            // penai_base::sample().await?;
            // openai_multiple_turns::sample().await?;
            openai_functions::sample().await?
        }
        Models::Gemini => {
            // gemini_base::gemini_base().await?;
            // gemini_thinking_mode::gemini_thinking_mode().await?;
        }
        Models::Groc => {
            // groc_base::sample().await?;
            // groc_medical_data::sample().await?;
            groc_multiple_turns::sample().await?;
        }
        Models::Xai => {
            // xai_base::sample().await?;
            // xai_multiple_turns::sample().await?;
            // xai_medical_prompt::sample().await?
            xai_functions::sample().await?
        }
    }    
    Ok(())
}