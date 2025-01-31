use langchain::langsmith::client::LangsmithClient;
use env_logger::Env;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let client = LangsmithClient::new()?;
    // Repo: rlm/rag-prompt - https://smith.langchain.com/hub/rlm/rag-prompt?organizationId=ef225b34-72b4-4fad-9967-e9948f522d2a
    let owner = "rlm";
    let repo = "rag-prompt";

    let response: Value = client
        .clone()
        .get_repo(
            owner,
            repo,      
        )
        .invoke()
        .await?;

    let commit_str = response
        .get("repo")
        .and_then(|repo| repo.get("last_commit_hash"))
        .and_then(|commit| commit.as_str())
        .unwrap_or_default();

    let response: Value = client
        .get_commit(
            owner,
            repo,      
            commit_str,
        )
        .invoke()
        .await?;

    match serde_json::to_string_pretty(&response) {
        Ok(json) => println!("Pretty-printed JSON:\n{}", json),
        Err(e) => println!("[ERROR] {:?}", e)
    }

    Ok(())
}