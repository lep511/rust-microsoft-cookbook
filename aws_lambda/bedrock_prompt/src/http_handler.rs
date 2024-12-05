use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde::Serialize;
use aws_config::BehaviorVersion;
use aws_sdk_bedrockagent::{
    error::SdkError,
    operation::list_prompts::{ListPromptsOutput, ListPromptsError},
    operation::get_prompt::{GetPromptOutput, GetPromptError},
    types::{PromptTemplateConfiguration, Message as AgentMessage},
    types::{ContentBlock as AgentContentBlock},
};
use aws_sdk_bedrockruntime::{
    operation::converse::{ConverseError, ConverseOutput},
    types::Message as RuntimeMessage,
    types::{ContentBlock, ConversationRole},
    Client,
};

// Set the model ID
const MODEL_ID: &str = "amazon.nova-micro-v1:0";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TextGenConfig {
    temperature: f32,
    top_p: f32,
    max_token_count: i32,
    stop_sequences: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NovaRequest {
    input_text: String,
    text_generation_config: TextGenConfig,
}

impl NovaRequest {
    fn new(prompt: String) -> Self {
        Self {
            input_text: prompt,
            text_generation_config: TextGenConfig {
                // higher temperature allows for more LLM creativity
                // the minimum value, 0.0, allows for a 100% predictable
                // response
                temperature: 0.2,
                // nucleus sampling probability - aka sampling the smallest
                // set of words that exceed the "top_p" threshold for a
                // response
                top_p: 0.0,
                // note here that 1 token is between 1 to 4 words
                // we have kept the max token count low here
                // to avoid high costs
                max_token_count: 100,
                stop_sequences: vec!["|".to_string()],
            },
        }
    }
}

#[derive(Debug)]
struct BedrockConverseError(String);
impl std::fmt::Display for BedrockConverseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't invoke '{}'. Reason: {}", MODEL_ID, self.0)
    }
}
impl std::error::Error for BedrockConverseError {}
impl From<&str> for BedrockConverseError {
    fn from(value: &str) -> Self {
        BedrockConverseError(value.to_string())
    }
}
impl From<&ConverseError> for BedrockConverseError {
    fn from(value: &ConverseError) -> Self {
        BedrockConverseError::from(match value {
            ConverseError::ModelTimeoutException(_) => "Model took too long",
            ConverseError::ModelNotReadyException(_) => "Model is not ready",
            _ => "Unknown",
        })
    }
}

fn get_converse_output_text(output: ConverseOutput) -> Result<String, BedrockConverseError> {
    let text = output
        .output()
        .ok_or("no output")?
        .as_message()
        .map_err(|_| "output not a message")?
        .content()
        .first()
        .ok_or("no content in message")?
        .as_text()
        .map_err(|_| "content is not text")?
        .to_string();
    Ok(text)
}

fn convert_agent_to_runtime_message(agent_message: &AgentMessage) -> RuntimeMessage {   

    let role_data: ConversationRole = ConversationRole::from(agent_message.role.as_str());
    match &agent_message.content[0] {
        AgentContentBlock::Text(text) => {
            let content_data: ContentBlock = ContentBlock::Text(text.to_string());
            RuntimeMessage::builder()
                .role(role_data)
                .content(content_data)
                .build()
                .expect("failed to build message")
        },
        _ => {
            let content_data: ContentBlock = ContentBlock::Text("Another data".to_string());
            RuntimeMessage::builder()
                .role(role_data)
                .content(content_data)
                .build()
                .expect("failed to build message")
        },
    }
}

fn process_variant_data(variant_data: GetPromptOutput) -> Result<RuntimeMessage, BedrockConverseError> {
    let user_message = "Analyze the pros and cons of remote work vs. office work";
    let mut message = RuntimeMessage::builder()
        .role(ConversationRole::User)
        .content(ContentBlock::Text(user_message.to_string()))
        .build()
        .map_err(|_| "failed to build message")?;


    let variants = variant_data.variants.iter().flatten().collect::<Vec<_>>();
    for variant in variants {
        match &variant.template_configuration {
            Some(PromptTemplateConfiguration::Chat(chat_config)) => {
                println!("Chat configuration found");
                message = convert_agent_to_runtime_message(&chat_config.messages[0]);
            },
            Some(PromptTemplateConfiguration::Text(text_config)) => {
                println!("Text configuration found");
            },
            _ => {
                println!("Unknown configuration found");
            }
        }
    }

    Ok(message)
}

async fn get_prompt_data(prompt_id: &str) -> Result<GetPromptOutput, SdkError<GetPromptError>> {
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;
    
    let client = aws_sdk_bedrockagent::Client::new(&sdk_config);
    let response = client.get_prompt().prompt_identifier(prompt_id).send().await;

    response
}

async fn list_prompts() -> Result<Vec<ListPromptsOutput>, SdkError<ListPromptsError>> {
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;
    
    let client = aws_sdk_bedrockagent::Client::new(&sdk_config);
   
    let mut all_prompts = Vec::new();
    let mut next_token: Option<String> = None;

    loop {
        let mut builder = client.list_prompts();
        
        // Set the next token if we have one from previous iteration
        if let Some(token) = next_token {
            builder = builder.next_token(token);
        }

        // Set reasonable max results per page
        builder = builder.max_results(50);

        // Execute the request
        match builder.send().await {
            Ok(output) => {
                all_prompts.push(output.clone());
                
                // Check if there are more results
                match output.next_token {
                    Some(token) => next_token = Some(token),
                    None => break, // No more results
                }
            },
            Err(err) => return Err(err),
        }
    }

    Ok(all_prompts)
}

// Helper function to print prompt summaries
fn print_prompt_summaries(outputs: &[ListPromptsOutput]) {
    for output in outputs {
        for prompt in output.prompt_summaries.iter() {
            println!("Prompt ID: {}", prompt.id);
            println!("Name: {}", prompt.name);
            println!("Created At: {:?}", prompt.created_at);
            println!("Last Updated At: {:?}", prompt.updated_at);
            println!("------------------------");
        }
    }
}

async fn call_bedrock(user_message: RuntimeMessage) -> Result<String, BedrockConverseError> {
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;
    let client = Client::new(&sdk_config);

    let response = client
        .converse()
        .model_id(MODEL_ID)
        .messages(user_message)
        .send()
        .await;

    match response {
        Ok(output) => {
            let text = get_converse_output_text(output)?;
            Ok(text)
        }
        Err(e) => Err(e
            .as_service_error()
            .map(BedrockConverseError::from)
            .unwrap_or_else(|| BedrockConverseError("Unknown service error".into()))),
    }
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let prompt_id = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("prompt_id"))
        .unwrap_or("Not prompt id");
    
    let prompt_data: GetPromptOutput = match get_prompt_data(prompt_id).await {
        Ok(prompt_data) => prompt_data,
        Err(e) => {
            let resp_text = format!("Error getting prompt data. {}", e);
            let resp = Response::builder()
                .status(400)
                .header("content-type", "text/html")
                .body(resp_text.into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    println!("Prompt name: {:?}", prompt_data.name);
    println!("Prompt version: {:?}", prompt_data.version);

    let mut msg = "No response".to_string();
    
    if prompt_data.variants.is_none() {
        println!("No variants found");
    } else {
        let message = process_variant_data(prompt_data)?;
        msg = call_bedrock(message).await?;
    };
 
    // match list_prompts().await {
    //     Ok(prompts) => {
    //         println!("Successfully retrieved prompts:");
    //         print_prompt_summaries(&prompts);
    //     },
    //     Err(err) => {
    //         println!("Error listing prompts: {:?}", err);
    //     }
    // }

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(msg.into())
        .map_err(Box::new)?;
    Ok(resp)
}