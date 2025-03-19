use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    table_bucket_arn: Option<String>,
    template_path: Option<String>,
    athena_bucket: Option<String>,
    xai_api_key: Option<String>,
}

fn main() {
    let variables = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => {
            println!("Error: {:?}", error);
            return;
        }
    };

    println!("Table bucket arn: {:?}", variables.table_bucket_arn);
}