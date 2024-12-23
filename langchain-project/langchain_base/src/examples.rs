mod anthropic_base;
mod anthropic_function_gw;
mod anthropic_function_gsp;
mod anthropic_complex;
mod gemini_base;
mod gemini_thinking_mode;
mod openai_base;
mod groc_base;
mod groc_medical_data;

pub(crate)async fn all_examples(model: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    match model {
        "anthropic" => {
            anthropic_base::sample().await?;
            anthropic_complex::sample().await?;
            anthropic_function_gw::sample().await?;
            anthropic_function_gsp::sample().await?;
        },
        "gemini" => {
            gemini_base::sample().await?;
            gemini_thinking_mode::sample().await?;
        },
        "openai" => {
            openai_base::sample().await?;
        },
        "groc" => {
            // groc_base::sample().await?;
            groc_medical_data::sample().await?;
        },
        _ => {
            println!("Model not supported");
        }
    }    
    Ok(())
}