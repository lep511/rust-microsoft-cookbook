use langchain::langsmith::client::LangsmithClient;
use env_logger::Env;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let client = LangsmithClient::new()?;
    let model_name = "Claude 3.5 Sonnet";
    let prompt_cost = 0.0003;
    let completion_cost = 0.0006;
    let match_pattern = r"^claude-3-5-sonnet-\d{8}$";
    let provider = Some("anthropic");
    let start_time = None;
    let match_path = None;

    let response: Value = client
        .create_model_price(
            model_name,
            prompt_cost,
            completion_cost,
            match_pattern,
            start_time,
            match_path,
            provider,       
        )
        .invoke()
        .await?;

    match serde_json::to_string_pretty(&response) {
        Ok(json) => println!("Pretty-printed JSON:\n{}", json),
        Err(e) => println!("[ERROR] {:?}", e)
    }

    Ok(())
}