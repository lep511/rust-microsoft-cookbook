use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum ChatError {
    #[error("Gemini API key not found in environment variables")]
    ApiKeyNotFound,
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    #[error("Failed to get response content")]
    ResponseContentError,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    contents: Vec<Content>,
    tools: Option<Vec<Value>>,
    #[serde(rename = "generationConfig")]
    generation_config: Option<GenerationConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Part {
    #[serde(default)]
    text: Option<String>,
    #[serde(rename = "functionCall", default)]
    function_call: Option<FunctionCall>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  GenerationConfig {
    temperature: f32,
    #[serde(rename = "topK")]
    top_p: f32,
    #[serde(rename = "topP")]
    top_k: i32,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: i32,
    #[serde(rename = "responseMimeType")]
    response_mime_type: String,
}

#[derive(Debug, Clone)]
pub struct ChatGemini {
    base_url: String,
    client: Client,
}

impl ChatGemini {
    pub fn new(model: &str) -> Result<Self, ChatError> {
        let api_key = match env::var("GEMINI_API_KEY") {
            Ok(key) => key,
            Err(env::VarError::NotPresent) => {
                println!("[ERROR] GEMINI_API_KEY not found in environment variables");
                return Err(ChatError::ApiKeyNotFound);
            }
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(ChatError::EnvError(e));
            }
        };
        
        let base_url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model,
            api_key,
        );
        
        Ok(Self {
            base_url,
            client: Client::builder()
                .use_rustls_tls()
                .build()?,
        })
    }

    pub async fn chat(&self, contents: ChatRequest) -> Result<Response, ChatError> {
        let response = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .json(&contents)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        if response["error"].is_object() {
            println!("Response: {:?}", response);
            return Err(ChatError::ResponseContentError)
        };

        let response = response.to_string();
        match serde_json::from_str(&response) {
            Ok(response_form) => Ok(response_form),
            Err(e) => {
                println!("Error: {:?}", e);
                Err(ChatError::ResponseContentError)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    candidates: Vec<Candidate>,
    model_version: Option<String>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: UsageMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    #[serde(rename = "avgLogprobs")]
    avg_logprobs: f64,
    content: Content,
    #[serde(rename = "finishReason")]
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    args: FunctionArgs,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionArgs {
    location: String,
    movie: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageMetadata {
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: i32,
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: i32,
    #[serde(rename = "totalTokenCount")]
    total_token_count: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = "gemini-1.5-flash";
    // let model = "gemini-1.5-pro";
    // let model = "gemini-2.0-flash-exp";
    // let chat = ChatGemini::new()?;
    
    // let content: Content = {
    //     Content {
    //         role: "user".to_string(),
    //         parts: vec![Part {
    //             text: Some("When is the next total solar eclipse, and where is the best location for viewing?".to_string()),
    //             function_call: None,
    //         }],
    //     }
    // };

    // let f_declaration = serde_json::json!({
    //     "google_search_retrieval": {
    //         "dynamic_retrieval_config": {
    //             "mode": "MODE_DYNAMIC",
    //             "dynamic_threshold": 0
    //         }
    //     }
    // });

    // let tools = vec![f_declaration];

    // let chat_request = ChatRequest {
    //     contents: vec![content],
    //     tools: Some(tools),
    //     generation_config: None,
    // };

    // let response = chat.chat(chat_request).await?;
    // println!("Response: {}", response);
    
    let chat = ChatGemini::new(model)?;
    let content: Content = {
        Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: Some("Which theaters in Mountain View show Barbie movie?".to_string()),
                function_call: None,
            }],
        }
    };

    let f_declaration = serde_json::json!({
        "function_declarations": [
            {
                "name": "find_movies",
                "description": "find movie titles currently playing in theaters based on any description, genre, title words, etc",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA or a zip code e.g. 95616"
                        },
                        "description": {
                            "type": "string",
                            "description": "Any kind of description including category or genre, title words, attributes, etc."
                        }
                    },
                    "required": [
                        "description"
                    ]
                }
            },
            {
                "name": "find_theaters",
                "description": "find theaters based on location and optionally movie title which is currently playing in theaters",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA or a zip code e.g. 95616"
                        },
                        "movie": {
                            "type": "string",
                            "description": "Any movie title"
                        }
                    },
                    "required": [
                        "location"
                    ]
                }
            },
            {
                "name": "get_showtimes",
                "description": "Find the start times for movies playing in a specific theater",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA or a zip code e.g. 95616"
                        },
                        "movie": {
                            "type": "string",
                            "description": "Any movie title"
                        },
                        "theater": {
                            "type": "string",
                            "description": "Name of the theater"
                        },
                        "date": {
                            "type": "string",
                            "description": "Date for requested showtime"
                        }
                    },
                    "required": [
                        "location",
                        "movie",
                        "theater",
                        "date"
                    ]
                }
            }
        ]
    });
    let tools = vec![f_declaration];
    let chat_request = ChatRequest {
        contents: vec![content],
        tools: Some(tools),
        generation_config: None,
    };

    let response = chat.chat(chat_request).await?;
    println!("Response: {:?}", response);

    Ok(())
}

