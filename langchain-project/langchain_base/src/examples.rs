// mod anthropic_base;
// mod anthropic_function_gw;
// mod anthropic_complex;
// mod gemini_base;
// mod gemini_thinking_mode;
mod openai_base;

pub(crate)async fn all_examples() -> Result<(), Box<dyn std::error::Error>> {
    
    // ### ANTHROPIC ###
    // anthropic_function_gw::sample().await?;
    // anthropic_base::sample().await?;
    // anthropic_complex::sample().await?;
    
    // ### GEMINI ###
    // gemini_base::sample().await?;
    // gemini_thinking_mode::sample().await?;

    // ### OPENAI ###
    openai_base::sample().await?;
    
    Ok(())
}