**Optional.**use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OpenAI's most advanced interface for generating model responses. Supports text and image 
/// inputs, and text outputs. Create stateful interactions with the model, using the output 
/// of previous responses as input. Extend the model's capabilities with built-in tools for 
/// file search, web search, computer use, and more. Allow the model access to external 
/// systems and data using function calling.
/// 
/// [Docs OpenAI - Responses](https://platform.openai.com/docs/api-reference/responses)
///
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseRequest {
    /// Text, image, or file inputs to the model, used to generate a response.
    pub input: Input,

    /// Model ID used to generate the response, like gpt-4o or o1. 
    pub model: String,

    /// **Optional.** Specify additional output data to include in the model response. 
    /// 
    /// # Currently supported values:
    /// * `file_search_call.results`: Include the search results of the file search tool call.
    /// * `message.input_image.image_url`: Include image urls from the input message.
    /// * `computer_call_output.output.image_url`: Include image urls from the computer call output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,

    /// **Optional.** Inserts a system (or developer) message as the first item in the model's 
    /// context. When using along with previous_response_id, the instructions from a previous 
    /// response will be not be carried over to the next response. 
    /// This makes it simple to swap out system (or developer) messages in new responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// **Optional.** An upper bound for the number of tokens that can be generated 
    /// for a response, including visible output tokens and reasoning tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,

    /// **Optional.** Set of 16 key-value pairs that can be attached to an object. 
    /// This can be useful for storing additional information about 
    /// the object in a structured format, and querying 
    /// for objects via API or the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,

    /// **Optional.** Whether to allow the model to run tool calls in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// **Optional.** The unique ID of the previous response to the model. 
    /// Use this to create multi-turn conversations.
    /// Learn more about [conversation state.](https://platform.openai.com/docs/guides/conversation-state)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// **Optional.** Configuration options for reasoning models.
    /// O-series models only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning:  Option<Reasoning>,

    /// **Optional.** Whether to store the generated model response 
    /// for later retrieval via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store:  Option<bool>,

    /// **Optional.** If set to true, the model response data will be streamed to the client 
    /// as it is generated using server-sent events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// **Optional.** What sampling temperature to use, between 0 and 2. Higher values 
    /// like 0.8 will make the output more random, while lower values like 0.2 will 
    /// make it more focused and deterministic. OpenAI recommend altering this 
    /// or top_p but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// **Optional.** Configuration options for a text response from the model. 
    /// Can be plain text or structured JSON data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseFormat>,

    /// **Optional.** How the model should select which tool (or tools) to use when 
    /// generating a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,

    /// **Optional.** An array of tools the model may call while generating a response.
    /// You can specify which tool to use by setting the tool_choice parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,

    /// **Optional.** An alternative to sampling with temperature, called nucleus sampling, 
    /// where the model considers the results of the tokens with top_p probability mass. 
    /// So 0.1 means only the tokens comprising the top 10% probability 
    /// mass are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// **Optional.** The truncation strategy to use for the model response.
    /// 
    /// # Strategies
    /// - **auto**: If the context of this response and previous ones exceeds 
    ///         the model's context window size, the model will truncate 
    ///         the response to fit the context window by dropping input 
    ///         items in the middle of the conversation.
    /// - **disabled** (default):  If a model response will exceed the context window size 
    ///                        for a model, the request will fail with a 400 error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<String>,

    /// **Optional.** A unique identifier representing your end-user, which can help 
    /// OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

pub enum ToolChoice {
    /// Controls which (if any) tool is called by the model.
    /// `none` means the model will not call any tool and instead generates a message.
    /// `auto` means the model can pick between generating a message or calling one or more tools.
    /// `required` means the model must call one or more tools.
    ToolChoiceMode(String),

    /// Indicates that the model should use a built-in tool to generate a response.
    HostedTool(HostedTool),

    /// Use this option to force the model to call a specific function.
    FunctionTool(FunctionTool),
}

/// Use to force the model to call a specific function.
pub struct FunctionTool {
    /// The name of the function to call.
    pub name: String,

    /// For function calling, the type is always function.
    #[serde(rename = "type")]    
    pub type_: String,
}
/// Indicates that the model should use a built-in tool to generate a response.
pub struct HostedTool {
    /// The type of hosted tool the model should to use. Learn more about built-in tools.
    /// # Allowed values are:
    /// * `file_search`
    /// * `web_search_preview`
    /// * `computer_use_preview`
    #[serde(rename = "type")]
    pub type_: String,
}

/// Configuration options for a text response from the model. 
/// Can be plain text or structured JSON data.
pub struct ResponseFormat {
    /// An object specifying the format that the model must output.
    /// - Configuring `{ "type": "json_schema" }`` enables Structured Outputs, 
    /// which ensures the model will match your supplied JSON schema.
    /// - The default format is `{ "type": "text" }`` with no additional options.
    /// 
    /// # Not recommended for gpt-4o and newer models:
    /// - Setting to `{ "type": "json_object" }`` enables the older JSON mode, 
    /// which ensures the message the model generates is valid JSON. 
    /// Using json_schema is preferred for models that support it.
    pub format: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Input {
    /// A text input to the model, equivalent to a text input with the user role.
    String(String),

    /// A list of one or many input items to the model, containing different
    /// content types.
    ItemList(Vec<InputItemList>),
}

/// A list of one or many input items to the model, 
/// containing different content types.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputItemList {
    /// A message input to the model with a role indicating instruction following hierarchy. 
    /// Instructions given with the developer or system role take precedence over instructions 
    /// given with the user role. Messages with the assistant role are presumed to have been 
    /// generated by the model in previous interactions.
    InputMessage(InputMessage),

    /// An item representing part of the context for the response to be generated by the model. 
    /// Can contain text, images, and audio inputs, as well as previous assistant responses 
    /// and tool call outputs.
    Item(String),

    ItemReference(ItemReference),

    /// An empty vector 
    Null,
}

pub enum Item {
    /// A message input to the model with a role indicating instruction following hierarchy. 
    /// Instructions given with the developer or system role take precedence over 
    /// instructions given with the user role.
    InputMessage(InputMessage),

    /// An output message from the model.
    OutputMessage(OutputMessage),

    /// The results of a file search tool call. See the file search guide for more information.
    FileSearchTool(FileSearchTool),

    /// A tool call to a computer use tool. See the computer use guide for more information.
    ComputerTool(Value),

    /// The output of a computer tool call.
    ComputerToolOutput(Value),

    /// The results of a web search tool call. See the web search guide for more information.
    WebSearchTool(Value),

    /// A tool call to run a function. See the function calling guide for more information.
    FunctionTool(Value),

    /// The output of a function tool call.
    FunctionToolOutput(Value),

    /// A description of the chain of thought used by a reasoning model while 
    /// generating a response.
    Reasoning(ReasoningItem),
}

/// The results of a file search tool call.
pub struct FileSearchTool {
    /// The unique ID of the file search tool call.
    pub id: String,

    /// The queries used to search for files.
    pub queries: Vec<String>,

    /// The status of the file search tool call. One of 
    /// in_progress, searching, incomplete or failed,
    pub status: String,

    /// The type of item to reference. Always item_reference.
    #[serde(rename = "type")]
    pub type_: String,

    /// **Optional.** The results of the file search tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<FileSearchToolResult>>,
}

/// The results of the file search tool call.
pub struct FileSearchToolResult {
    /// **Optional.** Set of 16 key-value pairs that can be attached to an object. 
    /// This can be useful for storing additional information about the object in 
    /// a structured format, and querying for objects via API or the dashboard.
    /// Keys are strings with a maximum length of 64 characters. 
    /// Values are strings with a maximum length of 512 characters, booleans, or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option,

    /// **Optional.** The unique ID of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    /// **Optional.** The name of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// **Optional.** The relevance score of the file - a value between 0 and 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,

    /// **Optional.** The text that was retrieved from the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<u32>,
}

/// An internal identifier for an item to reference.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemReference {
    /// The ID of the item to reference.
    pub id: String,

    /// The type of item to reference. Always item_reference.
    #[serde(rename = "type")]
    pub type_: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct Format {
    pub format: Value,
}

/// Configuration options for reasoning models.
#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct Reasoning {
    /// Constrains effort on reasoning for reasoning models. Currently supported values 
    /// are low, medium, and high. Reducing reasoning effort can result in faster 
    /// responses and fewer tokens used on reasoning in a response.
    /// O-series models only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,

    /// A summary of the reasoning performed by the model. This can be useful for 
    /// debugging and understanding the model's reasoning process. 
    /// One of concise or detailed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_summary: Option<String>,
}

/// A message input to the model with a role indicating instruction following hierarchy. 
/// Instructions given with the developer or system role take precedence over 
/// instructions given with the user role.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputMessage {
    /// A list of one or many input items to the model, containing 
    /// different content types.
    pub content: Vec<Content>,

    /// The role of the message input. One of user, system, or developer.
    pub role: Role,

    /// The status of item. One of in_progress, completed, or incomplete. 
    /// Populated when items are returned via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The type of the message input. Always set to message.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

/// An output message from the model.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputMessage {
    /// The content of the output message.
    pub content: Vec<Content>,

    /// The unique ID of the output message.
    pub id: String,

    /// The role of the output message. Always assistant.
    pub role: Role,

    /// The status of the message input. One of in_progress, completed, or 
    /// incomplete. Populated when input items are returned via API.
    pub status: String,

    /// The type of the output message. Always message.
    #[serde(rename = "type")]
    pub type_: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Platform,
    Developer,
    User,
    Assistant,
    Tool,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageUrl>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Source {
    #[serde(rename = "type")]
    pub type_: String,

    pub media_type: String,

    pub data: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrl {
    pub url: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ResponseObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<String>, // ToDo - Change

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,  // ToDo - Change

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Format>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub code: String, // ToDo - Change
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorDetails {
    pub code: String,

    pub message: String,
}