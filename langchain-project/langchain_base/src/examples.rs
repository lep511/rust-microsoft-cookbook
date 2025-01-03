#[allow(dead_code)]
mod anthropic_base;
mod anthropic_function_gw;
mod anthropic_function_gsp;
mod anthropic_code_execution;
mod anthropic_image;
mod anthropic_complex;
mod gemini_base;
mod gemini_thinking_mode;
mod gemini_find_brands;
mod gemini_find_multiple_turns;
mod gemini_cache_content;
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
mod replicate_base;
mod replicate_generate_image;
mod replicate_predictions;

#[allow(dead_code)]
pub enum Models {
    Anthropic,
    OpenAI,
    Gemini,
    Groc,
    Xai,
    Replicate,
}

pub(crate)async fn all_examples(model: Models) -> Result<(), Box<dyn std::error::Error>> {
    
    match model {
        Models::Anthropic => {
            anthropic_base::sample().await?;
            // anthropic_complex::sample().await?;
            // anthropic_function_gw::sample().await?;
            // anthropic_function_gsp::sample().await?;
            // anthropic_code_execution::sample().await?;
            // anthropic_image::sample().await?;
        }
        Models::OpenAI => {
            openai_base::sample().await?;
            // openai_multiple_turns::sample().await?;
            // openai_functions::sample().await?
        }
        Models::Gemini => {
            // gemini_base::sample().await?;
            // gemini_thinking_mode::sample().await?;
            // gemini_find_brands::sample().await?
            // gemini_find_multiple_turns::sample().await?
            gemini_cache_content::sample().await?
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
        Models::Replicate => {
            // replicate_base::sample().await?;
            // replicate_generate_image::sample().await?;
            replicate_predictions::sample().await?;
        }
    }    
    Ok(())
}