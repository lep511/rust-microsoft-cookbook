use futures::pin_mut;
use futures::StreamExt;
use async_stream::stream;
use crate::compatible::{
    CHAT_COMPLETION, DEFERRED_COMPLETION,
};
use crate::compatible::requests::{
    request_chat, get_request, strem_chat,
};
use crate::compatible::utils::{GetApiKey, read_file_data};
use crate::compatible::libs::{
    ChatRequest, Message, ChatResponse, ChatStreamResponse, 
    Content, ImageUrl,
};
use crate::compatible::error::CompatibleChatError;
use tokio::time::sleep;
use std::time::Duration;
use log::{info, error};
use serde_json::{Value, json, from_str};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatCompatible {
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: u64,
    pub max_retries: i32,
    pub url: String,
    pub model: String,
}

#[allow(dead_code)]
impl ChatCompatible {
    pub fn new(url: &str, model: &str) -> Self {
        let api_key: String = match Self::get_api_key() {
            Ok(api_key) => api_key,
            Err(_) => "not_key".to_string()
        };

        let request = ChatRequest {
            model: None,
            messages: None,
            input: None,
            temperature: None,
            max_tokens: None,
            tools: None,
            tool_choice: None,
            frequency_penalty: None,
            presence_penalty: None,
            top_p: None,
            min_p: None,
            top_k: None,
            stop: None,
            n_completion: None,
            response_format: None,
            stream: Some(false),
            deferred: None,
        };
        
        Self {
            api_key: api_key,
            request: request,
            timeout: 15 * 60, // default: 15 minutes
            max_retries: 3,         // default: 3 times
            url: url.to_string(),
            model: model.to_string(),
        }
    }

    pub async fn invoke(
        mut self,
        prompt: &str,
    ) -> Result<ChatResponse, CompatibleChatError> {

        let content = vec![Content {
            content_type: "text".to_string(),
            text: Some(prompt.to_string()),
            source: None,
            image_url: None,
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];
        
        let new_message = Message {
            role: Some("user".to_string()),
            content: content,
            tool_calls: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self.request.model = Some(self.model.clone());
        let url = format!("{}/{}", self.url, CHAT_COMPLETION);

        let response = match request_chat(
            &url,
            &self.request,
            &self.api_key,
            self.timeout,
            self.max_retries,
        ).await {
            Ok(response) => response,
            Err(error) => {
                error!("Error {:?}", error);
                return Err(error);
            }
        };

        let response_string = response.to_string();
        
        let chat_response: ChatResponse = match from_str(&response_string) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            error!("Error {}", error.message);
            return Err(CompatibleChatError::ResponseContentError);
        } else {
            let format_response = ChatResponse {
                choices: chat_response.choices,
                created: chat_response.created,
                id: chat_response.id,
                model: chat_response.model,
                object: chat_response.object,
                system_fingerprint: chat_response.system_fingerprint,
                usage: chat_response.usage,
                chat_history: self.request.messages,
                request_id: chat_response.request_id,
                error: None,
            };
            Ok(format_response)
        }
    }

    pub async fn with_input_replicate(
        mut self, 
        input: Value,
    ) -> Result<Value, CompatibleChatError> {
        let url_format = format!("{}/{}", self.url, self.model);
       
        let request = ChatRequest {
            input: Some(input),
            ..Default::default()
        };

        self.request = request;
    
        let response: Value = match request_chat(
            &url_format,
            &self.request,
            &self.api_key,
            self.timeout,
            self.max_retries,
        ).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };

        let url_str = response
            .get("urls")
            .and_then(|urls| urls.get("get"))
            .and_then(|url| url.as_str())
            .ok_or(CompatibleChatError::ResponseContentError)?;

        let mut status = String::from("processing");

        while status == "processing" {
            let response = get_request(url_str, &self.api_key)
                .await
                .map_err(|e| {
                    error!("Error fetching response: {:?}", e);
                    CompatibleChatError::ResponseContentError
                })?;
    
            status = response
                .get("status")
                .and_then(|s| s.as_str())
                .unwrap_or("error")
                .to_string();
    
            if status == "processing" {
                info!("Status: {}", status);
                sleep(Duration::from_secs(4)).await;
            } else {
                return Ok(response);
            }
        }
    
        Ok(json!({"detail": "Not found"}))
    }

    /// Sends a chat completion request to the Baseten API endpoint and returns the response.
    /// This method handles the non-streaming chat completion request, formatting the prompt
    /// and managing the API communication.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `prompt` - The user's input text to generate a response for
    ///
    /// # Returns
    ///
    /// * `Result<Value, CompatibleChatError>` - Either the API response as JSON or an error
    ///
    /// # Errors
    ///
    /// Returns `CompatibleChatError::ResponseContentError` if:
    /// - The API request fails
    /// - The response cannot be parsed
    /// - Network communication issues occur
    ///
    /// # Example
    ///
    /// ```rust
    /// let response = ChatCompletion::new()
    ///     .with_api_key("your-api-key")
    ///     .baseten_invoke("What is the weather today?")
    ///     .await?;
    /// ```
    ///
    pub async fn baseten_invoke(
        mut self, 
        prompt: &str,
    ) -> Result<Value, CompatibleChatError> {
        let content = vec![Content {
            content_type: "text".to_string(),
            text: Some(prompt.to_string()),
            source: None,
            image_url: None,
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];
        
        let new_message = Message {
            role: Some("user".to_string()),
            content: content,
            tool_calls: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }
        
        self.request.stream = Some(false);
        let api_key_format = format!("Api-Key {}", self.api_key);

        let url = format!("{}/{}", self.url, CHAT_COMPLETION);
           
        let response: Value = match request_chat(
            &url,
            &self.request,
            &api_key_format,
            self.timeout,
            self.max_retries,
        ).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };
        
        Ok(response)
    }

    /// Retrieves information by making a GET request to the specified endpoint
    /// 
    /// # Arguments
    /// 
    /// * `self` - The instance containing base URL and API key configuration
    /// * `url` - The URL path to make get request.
    /// 
    /// # Returns
    /// 
    /// * `Result<Value, CompatibleChatError>` - JSON response on success, or error on failure
    /// 
    /// # Errors
    /// 
    /// Returns `CompatibleChatError::ResponseContentError` if the request fails or response cannot be parsed
    ///
    pub async fn handle_get_request(
        self,
        url: &str
    ) -> Result<Value, CompatibleChatError> {
        let response: Value = match get_request(
            &url,
            &self.api_key,
        ).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };

        Ok(response)
    }

    pub async fn get_models(
        self,
        suffix_url: &str
    ) -> Result<Value, CompatibleChatError> {
        let url = format!(
            "{}/{}/{}", 
            self.url, 
            CHAT_COMPLETION, 
            suffix_url
        );

        let response: Value = self.handle_get_request(&url).await?;
        
        Ok(response)
    }

    /// Retrieves model information by making a GET request to the specified endpoint
    /// 
    /// # Arguments
    /// 
    /// * `self` - The instance containing base URL and API key configuration
    /// * `deferred_request_id` - The deferred request id returned by a previous deferred chat request.
    /// 
    pub async fn get_deferred(
        self,
        deferred_request_id: &str
    ) -> Result<ChatResponse, CompatibleChatError> {
        let url = format!(
            "{}/{}/{}", 
            self.url,
            DEFERRED_COMPLETION,
            deferred_request_id);
        let response: Value = self.handle_get_request(&url).await?;
        let response_string = response.to_string();
        
        let chat_response: ChatResponse = match from_str(&response_string) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };

        Ok(chat_response)
    }

    /// Creates a streaming response for chat completions.
    /// This function enables real-time streaming of the AI's response, returning chunks
    /// of the response as they become available instead of waiting for the complete response.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `prompt` - The user's input text to generate a streaming response for
    ///
    /// # Returns
    ///
    /// * `impl futures::Stream<Item = ChatStreamResponse>` - A stream that yields chat response chunks
    ///
    /// # Example
    ///
    /// ```rust
    /// use futures::StreamExt;
    /// use futures::pin_mut;
    ///
    /// let chat_stream = ChatCompletion::new()
    ///     .stream_response("Tell me a story");
    ///
    /// pin_mut!(chat_stream);
    /// 
    /// while let Some(stream_response) = stream.next().await { 
    ///     if let Some(choices) = stream_response.choices {
    ///         for choice in choices {
    ///             if let Some(delta) = choice.delta {
    ///                 if let Some(content) = delta.content {
    ///                     if content.is_empty() {
    ///                         continue;
    ///                     }
    ///                     print!("{}", content);
    ///                 }
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    ///
    /// # Technical Details
    ///
    /// The streaming process works as follows:
    ///
    /// 1. Message Preparation:
    ///    - Converts the user prompt into a Content structure
    ///    - Creates a Message with "user" role
    ///    - Adds the message to the conversation history
    ///
    /// 2. Stream Configuration:
    ///    - Sets the model to be used
    ///    - Enables streaming mode (stream = true)
    ///    - Maintains existing temperature, max_tokens, and other parameters
    ///
    /// 3. Stream Processing:
    ///    - Uses Server-Sent Events (SSE) for real-time data streaming
    ///    - Each chunk contains a partial completion in ChatStreamResponse format
    ///    - Chunks are delivered as soon as they're generated by the model
    ///
    /// 4. Response Structure:
    ///    - Each ChatStreamResponse contains:
    ///      * id: Unique identifier for the response
    ///      * choices: Array of completion choices
    ///      * delta: Incremental updates to the response
    ///      * content: The actual text fragment
    ///
    /// 5. Error Handling:
    ///    - Network errors are propagated through the stream
    ///    - Empty content chunks are filtered out
    ///    - Stream automatically closes when completion is finished
    ///
    /// 6. Resource Management:
    ///    - Stream is pinned to ensure proper async handling
    ///    - Resources are automatically cleaned up when the stream is dropped
    ///    - Connection is maintained until the complete response is received
    ///
    /// Note: The stream must be properly awaited and handled in an async context.
    /// Dropping the stream before completion will terminate the response generation.
    ///
    pub fn stream_response(
        mut self,
        prompt: String,  // Don't change type for stream
    ) -> impl futures::Stream<Item = ChatStreamResponse> {
        stream! {            
            let content = vec![Content {
                content_type: "text".to_string(),
                text: Some(prompt.to_string()),
                source: None,
                image_url: None,
                image_base64: None,
                id: None,
                name: None,
                input: None,
                content: None,
                tool_use_id: None,
            }];
            
            let new_message = Message {
                role: Some("user".to_string()),
                content: content,
                tool_calls: None,
            };
    
            if let Some(messages) = &mut self.request.messages {
                messages.push(new_message);
            } else {
                self.request.messages = Some(vec![new_message]);
            }

            self.request.model = Some(self.model.clone());
            self.request.stream = Some(true);
            let url = format!("{}/{}", self.url, CHAT_COMPLETION);

            let stream = strem_chat(
                url.clone(),
                self.api_key.clone(),
                self.request.clone(),
            );

            pin_mut!(stream);

            while let Some(chat_response) = stream.next().await {
                yield chat_response;
            }
        }
    }

    /// Sets the temperature for response generation.
    /// Temperature controls the randomness of the output. Higher values (e.g., 0.8) make the output
    /// more random, while lower values (e.g., 0.2) make it more focused and deterministic.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `temperature` - A float between 0 and 2, where 0 is the most deterministic
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated temperature
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_temperature(0.7);
    /// ```
    ///
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature);
        self
    }

    /// Sets the maximum number of tokens to generate.
    /// This limits the length of the response. One token is roughly 4 characters for normal English text.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `max_tokens` - The maximum number of tokens to generate
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated max tokens
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_max_tokens(150);
    /// ```
    ///
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the frequency penalty for response generation.
    /// Reduces the likelihood of repeating the same information by penalizing tokens 
    /// based on their frequency in the text so far.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `frequency_penalty` - Number between -2.0 and 2.0. Positive values decrease repetition
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated frequency penalty
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_frequency_penalty(0.5);
    /// ```
    ///
    pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.request.frequency_penalty = Some(frequency_penalty);
        self
    }

    /// Sets the timeout duration in seconds for the API request.
    /// If the request takes longer than this duration, it will be cancelled.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `timeout` - The number of seconds to wait before timing out
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated timeout
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_timeout_sec(30);
    /// ```
    ///
    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the presence penalty for response generation.
    /// Increases the likelihood of talking about new topics by penalizing tokens 
    /// that have appeared in the text at all.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `presence_penalty` - Number between -2.0 and 2.0. Positive values encourage new topics
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated presence penalty
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_presence_penalty(0.5);
    /// ```
    ///
    pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.request.presence_penalty = Some(presence_penalty);
        self
    }

    /// Sets the top_p value for nucleus sampling.
    /// Controls diversity by using only the most likely tokens whose cumulative probability 
    /// exceeds the top_p value.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `top_p` - Float between 0 and 1. Lower values increase focus
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated top_p value
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_top_p(0.9);
    /// ```
    ///
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.request.top_p = Some(top_p);
        self
    }

    /// Sets the minimum probability threshold for token selection.
    /// Only tokens with probability greater than min_p will be considered 
    /// in the generation process.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `min_p` - Float between 0 and 1. Higher values increase quality but reduce diversity
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated min_p value
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_min_p(0.05);
    /// ```
    ///
    pub fn with_min_p(mut self, min_p: f32) -> Self {
        self.request.min_p = Some(min_p);
        self
    }

    /// Sets the number of alternative completions to generate.
    /// The API will return multiple alternative completions based on this number.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `n_completion` - The number of completions to generate
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated completion count
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_n_completion(3);
    /// ```
    ///
    pub fn with_n_completion(mut self, n_completion: u32) -> Self {
        self.request.n_completion = Some(n_completion);
        self
    }

    /// Sets the stop sequences for response generation.
    /// The API will stop generating further tokens when it encounters any of these sequences.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `stop` - A vector of strings that will cause the API to stop generating
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated stop sequences
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_stop(vec!["END".to_string(), "STOP".to_string()]);
    /// ```
    ///
    pub fn with_stop(mut self, stop: Vec<String>) -> Self {
        self.request.stop = Some(stop);
        self
    }

    /// Sets the maximum number of retry attempts for failed API requests.
    /// If an API request fails, it will be retried up to this many times.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `max_retries` - The maximum number of retry attempts
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated retry count
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_max_retries(3);
    /// ```
    ///
    pub fn with_max_retries(mut self, max_retries: i32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets the system prompt for the chat completion.
    /// A system prompt provides initial context or instructions that guide
    /// the behavior and responses of the AI assistant throughout the conversation.
    /// This message is always inserted at the beginning of the conversation.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `system_prompt` - A string slice containing the system instructions or context
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the added system prompt
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_system_prompt("You are a helpful assistant that speaks in a friendly tone.");
    /// ```
    ///
    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {

        let content = vec![Content {
            content_type: "text".to_string(),
            text: Some(system_prompt.to_string()),
            source: None,
            image_url: None,
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];
        
        let new_message = Message {
            role: Some("system".to_string()),
            content: content,
            tool_calls: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.insert(0, new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    /// Adds an assistant's response to the beginning of the chat history.
    /// This method prepends a new message with the assistant's response text
    /// to the conversation history.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `assistant_response` - A string slice containing the assistant's response text
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the added assistant message
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_assistant_response("Hello! How can I help you today?");
    /// ```
    ///
    pub fn with_assistant_response(mut self,  assistant_response: &str) -> Self {
        
        let content = vec![Content {
            content_type: "text".to_string(),
            text: Some(assistant_response.to_string()),
            source: None,
            image_url: None,
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];
        
        let new_message = Message {
            role: Some("assistant".to_string()),
            content: content,
            tool_calls: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.insert(0, new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    /// Sets the chat history for the conversation.
    /// This method allows setting a complete conversation history by providing
    /// a vector of previous messages.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `history` - A vector of Message objects representing the conversation history
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated chat history
    ///
    /// # Example
    ///
    /// ```rust
    /// let history = vec![
    ///     Message::new("user", "Hello"),
    ///     Message::new("assistant", "Hi there!")
    /// ];
    /// let chat = ChatCompletion::new()
    ///     .with_chat_history(history);
    /// ```
    ///
    pub fn with_chat_history(mut self, history: Vec<Message>) -> Self {
        self.request.messages = Some(history);
        self
    }

    /// Configures the available tools for the chat completion.
    /// This method sets up the tools that the model can use during the conversation.
    /// Tools are represented as JSON values that define their capabilities and parameters.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `tools_data` - A vector of JSON values defining the available tools
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the configured tools
    ///
    /// # Example
    ///
    /// ```rust
    /// let tools = vec![
    ///     json!({
    ///         "type": "function",
    ///         "function": {
    ///             "name": "get_weather",
    ///             "description": "Get the weather for a location"
    ///         }
    ///     })
    /// ];
    /// let chat = ChatCompletion::new()
    ///     .with_tools(tools);
    /// ```
    ///
    pub fn with_tools(mut self, tools_data: Vec<Value>) -> Self {
        self.request.tools = Some(tools_data);
        self
    }
    
    /// Sets the tool choice for the chat completion.
    /// This method specifies which tool should be used by the model
    /// when processing the conversation.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `tool_choice` - A JSON value specifying the tool choice configuration
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the specified tool choice
    ///
    /// # Example
    ///
    /// ```rust
    /// let tool_choice = json!({
    ///     "type": "function",
    ///     "function": {"name": "get_weather"}
    /// });
    /// let chat = ChatCompletion::new()
    ///     .with_tool_choice(tool_choice);
    /// ```
    ///
    pub fn with_tool_choice(mut self, tool_choice: Value) -> Self {
        self.request.tool_choice = Some(tool_choice);
        self
    }

    /// Deferred Chat Completions allow you to create a chat completion, 
    /// get a response_id, and retrieve the response at a later time. 
    /// The result would be available to be requested exactly once within 
    /// 24 hours, after which it would be discar
    ///
    /// # Arguments
    /// 
    /// * `self` - The instance containing base URL and API key configuration
    /// * `deferred` - A boolean flag to enable or disable deferred chat completion
    /// 
    /// # Returns
    /// 
    /// * `Self` - Returns the modified instance with updated deferred setting
    ///
    pub fn with_deferred(mut self, deferred: bool) -> Self {
        self.request.deferred = Some(deferred);
        self
    }

    /// Adds an image URL to the chat completion request.
    /// This method allows adding an image to the conversation by providing its URL.
    /// The image will be processed with high detail quality.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `image_url` - A string slice containing the URL of the image to be included
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the added image message
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_image_url("https://example.com/image.jpg");
    /// ```
    ///
    pub fn with_image_url(
        mut self, 
        image_url: &str, 
    ) -> Self {

        let image_url = ImageUrl {
            url: image_url.to_string(),
            detail: "high".to_string(),
        };

        let content = vec![Content {
            content_type: "image_url".to_string(),
            text: None,
            source: None,
            image_url: Some(image_url),
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];

        let new_message = Message {
            role: Some("user".to_string()),
            content: content,
            tool_calls: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    /// Adds a base64-encoded image to the chat completion request.
    /// This method allows adding an image to the conversation by providing its base64 encoding
    /// and MIME type. The image will be processed with high detail quality.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `image_base64` - A string slice containing the base64-encoded image data
    /// * `mime_type` - A string slice specifying the MIME type of the image (e.g., "image/jpeg", "image/png")
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the added image message
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_image_base64("iVBORw0KGgoAAAANSUhE...", "image/png");
    /// ```
    ///
    pub fn with_image_base64(
        mut self, 
        image_base64: &str, 
        mime_type: &str
    ) -> Self {

        let url = format!("data:{};base64,{}", mime_type, image_base64);

        let image_url = ImageUrl {
            url: url,
            detail: "high".to_string(),
        };
        
        let content = vec![Content {
            content_type: "image_url".to_string(),
            text: None,
            source: None,
            image_url: Some(image_url),
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];

        let new_message = Message {
            role: Some("user".to_string()),
            content: content,
            tool_calls: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    /// Sets the API key for authentication with the service.
    /// 
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `api_key` - A string slice containing the API key for authentication
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the updated API key
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_api_key("your-api-key-here");
    /// ```
    ///
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    /// Adds an image from a local file to the chat completion request.
    /// This method reads an image file from the specified path, converts it to base64,
    /// and adds it to the conversation. The image will be processed with high detail quality.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance containing the chat completion configuration
    /// * `file_path` - A string slice containing the path to the image file
    /// * `mime_type` - A string slice specifying the MIME type of the image (e.g., "image/jpeg", "image/png")
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the modified instance with the added image message
    ///
    /// # Errors
    ///
    /// Returns the unmodified instance if there's an error reading the file,
    /// and logs the error using the error! macro.
    ///
    /// # Example
    ///
    /// ```rust
    /// let chat = ChatCompletion::new()
    ///     .with_image_file("path/to/image.jpg", "image/jpeg");
    /// ```
    ///
    pub fn with_image_file(
        mut self, 
        file_path: &str, 
        mime_type: &str
    ) -> Self {

        let image_base64 = match read_file_data(file_path) {
            Ok(data) => data,
            Err(e) => {
                error!("Error {:?}", e);
                return self;
            }
        };

        let url = format!("data:{};base64,{}", mime_type, image_base64);

        let image_url = ImageUrl {
            url: url,
            detail: "high".to_string(),
        };
        
        let content = vec![Content {
            content_type: "image_url".to_string(),
            text: None,
            source: None,
            image_url: Some(image_url),
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];

        let new_message = Message {
            role: Some("user".to_string()),
            content: content,
            tool_calls: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }
}

impl GetApiKey for ChatCompatible {}