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
mod groc_base;
mod groc_medical_data;
mod groc_multiple_turns;
mod xai_base;
mod xai_multiple_turns;

pub(crate)async fn all_examples(model: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    match model {
        "anthropic" => {
            // anthropic_base::sample().await?;
            // anthropic_complex::sample().await?;
            // anthropic_function_gw::sample().await?;
            anthropic_function_gsp::sample().await?;
            // anthropic_code_execution::sample().await?;
            // anthropic_image::sample().await?;
        },
        "gemini" => {
            gemini_base::sample().await?;
            gemini_thinking_mode::sample().await?;
        },
        "openai" => {
            // openai_base::sample().await?;
            // openai_multiple_turns::sample().await?;
        },
        "groc" => {
            // groc_base::sample().await?;
            // groc_medical_data::sample().await?;
            groc_multiple_turns::sample().await?;
        },
        "xai" => {
            // xai_base::sample().await?;
            xai_multiple_turns::sample().await?;
        },
        _ => {
            println!("Model not supported");
        }
    }    
    Ok(())
}