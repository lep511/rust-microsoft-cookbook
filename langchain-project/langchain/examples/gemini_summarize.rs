#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use std::fs;
use env_logger::Env;

fn read_file_to_string(path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatGemini::new("gemini-2.0-pro-exp-02-05");

    let article = read_file_to_string("tests/files/article.txt")?;

    let prompt = format!("Your task is to perform the following actions: \
            1 - Summarize the following text delimited by \
                <> with 1 sentence. \
            2 - Translate the summary into Spanish. \
            3 - List each name in the Spanish summary. \
            4 - Output a json object that contains the \
            following keys: spanish_summary, num_names. \
            \
            Use the following format: \
            Text: <text to summarize> \
            Summary: <summary> \
            Translation: <summary translation> \
            Names: <list of names in summary> \
            Output JSON: <json with summary and num_names> \
            \
            Text: <{article}>");

    let response = llm
        .with_temperature(0.9)
        .with_max_tokens(8192)
        .with_max_retries(3)
        .invoke(&prompt)
        .await?;

    // println!("{:?}", response);
   
    response.candidates.as_ref().map(|candidates| {
        candidates.iter().for_each(|candidate| {
            candidate.content.as_ref().map(|content| {
                content.parts.iter().for_each(|part| {
                    part.text.as_ref().map(|text| {
                        println!("{}", text);
                    });
                });
            });
        });
    });

    Ok(())
}