use aws_config::meta::region::RegionProviderChain;
use aws_sdk_config::Client;
use aws_sdk_config::Error;
use aws_config::BehaviorVersion;
use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Serialize)]
struct ConfigRuleJson {
    config_rule_name: Option<String>,
    config_rule_arn: Option<String>,
    config_rule_id: Option<String>,
    description: Option<String>,
    scope: Option<String>, // Simplified for demonstration
    source: Option<String>, // Simplified for demonstration
    input_parameters: Option<String>,
    maximum_execution_frequency: Option<String>,
    config_rule_state: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load the default region and credentials
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::defaults(BehaviorVersion::v2024_03_28()).region(region_provider).load().await;
    let client = Client::new(&config);

    // List the config rules
    let rules = client.describe_config_rules().send().await?;

    let rules_json: Vec<ConfigRuleJson> = rules
        .config_rules()
        .iter()
        .map(|rule| ConfigRuleJson {
            config_rule_name: rule.config_rule_name().map(String::from),
            config_rule_arn: rule.config_rule_arn().map(String::from),
            config_rule_id: rule.config_rule_id().map(String::from),
            description: rule.description().map(String::from),
            scope: rule.scope().map(|s| format!("{:?}", s)), // Simplified
            source: rule.source().map(|s| format!("{:?}", s)), // Simplified
            input_parameters: rule.input_parameters().map(String::from),
            maximum_execution_frequency: rule.maximum_execution_frequency().map(|f| format!("{:?}", f)),
            config_rule_state: rule.config_rule_state().map(|s| format!("{:?}", s)),
        })
        .collect();

    println!("{}", to_string_pretty(&rules_json).unwrap());

    Ok(())
}