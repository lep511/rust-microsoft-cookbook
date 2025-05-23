use crate::anthropic::chat::ChatAnthropic;
use crate::openai::chat::ChatOpenAI;

const DEFAULT_OPENAI_MODEL: &str = "gpt-4.5-preview";
const DEFAULT_ANTHROPIC_MODEL: &str = "claude-3-7-sonnet-20250219";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AgentType {
    /// An agent that uses the OpenAI API.
    OpenAI,
    
    /// An agent that uses the Anthropic API.
    Anthropic,
}

/// The behavior of the agent when it uses tools. This determines how the agent interacts with
/// the tools it has access to. The behavior can be customized to suit different use cases.
/// The behavior can be one of the following:
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ToolUseBehavior {
    /// The default behavior. Tools are run, and then the LLM receives the results
    /// and gets to respond.
    RunLLMAgain,
    
    /// The output of the first tool call is used as the final output. This
    /// means that the LLM does not process the result of the tool call.
    StopOnFirstTool,

    /// The agent will stop running if any of the tools in the list are called.
    /// The final output will be the output of the first matching tool call. The LLM does not
    /// process the result of the tool call.
    StopAtTools(Vec<String>),
    
    /// If you pass a function, it will be called with the run context and the list of
    /// tool results. It must return a `ToolToFinalOutputResult`, which determines whether the tool
    /// calls result in a final output.
    CustomFunction(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModelSettings {
    /// The temperature to use when generating text. This controls the randomness of the output.
    pub temperature: f32,
    
    /// The number of tokens to use when generating text. This controls the length of the output.
    pub max_tokens: u32,

    /// Configuration for enabling LLM's extended thinking.
    pub thinking: Option<Thinking>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Thinking {
    /// The agent is thinking. This means that the agent is currently processing the input
    /// and generating a response.
    Enabled(i32),
    
    /// The agent is not thinking. This means that the agent is not currently processing the input
    /// and generating a response.
    Disabled,
}

/// An agent is an AI model configured with instructions, tools, guardrails, handoffs and more.
///
/// We strongly recommend passing `instructions`, which is the "system prompt" for the agent. In
/// addition, you can pass `handoff_description`, which is a human-readable description of the
/// agent, used when the agent is used inside tools/handoffs.

/// Agents are generic on the context type. The context is a (mutable) object you create. It is
/// passed to tool functions, handoffs, guardrails, etc.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Agent {
    /// The name of the agent.
    pub name: String,
    
    /// The instructions for the agent. Will be used as the "system prompt" when this agent is
    /// invoked. Describes what the agent should do, and how it responds.
    pub instructions: String,

    /// The type of the agent. This is used to determine which model to use when invoking the agent.
    pub agent_type: AgentType,
    
    /// A description of the agent. This is used when the agent is used as a handoff, so that an
    /// LLM knows what it does and when to invoke it.
    pub handoff_description: Option<String>,
    
    /// Handoffs are sub-agents that the agent can delegate to. You can provide a list of handoffs,
    /// and the agent can choose to delegate to them if relevant. Allows for separation of concerns and
    /// modularity.
    pub handoffs: Option<Vec<String>>,
   
    /// The model implementation to use when invoking the LLM.
    pub model: Option<String>,

    /// Configures model-specific tuning parameters (e.g. temperature, top_p)
    pub model_settings: Option<ModelSettings>,

    /// A list of tools that the agent can use.
    pub tools: Option<Vec<String>>,

    /// A list of checks that run in parallel to the agent's execution, before generating a
    /// response. Runs only if the agent is the first agent in the chain.
    pub input_guardrails: Option<Vec<String>>,

    /// A list of checks that run on the final output of the agent, after generating a response.
    /// Runs only if the agent produces a final output.
    pub output_guardrails: Option<Vec<String>>,

    ///The type of the output object. If not provided, the output will be <str>.
    pub output_type: Option<String>,

    /// A class that receives callbacks on various lifecycle events for this agent.
    pub hooks: Option<Vec<String>>,

    /// A list of tools that the agent can use. These are the tools that the agent can use to
    /// perform actions. The agent can choose to use these tools, and the results of the tools
    /// will be passed to the agent as input.
    pub tool_use_behavior: Option<ToolUseBehavior>,    

}

impl Agent {
    /// Creates a new agent with the given name and instructions.
    pub async fn new(name: String, instructions: String) -> Self {
        Self {
            name,
            instructions,
            agent_type: AgentType::OpenAI, // Default to OpenAI
            handoff_description: None,
            handoffs: None,
            model: None,
            model_settings: None,
            tools: None,
            input_guardrails: None,
            output_guardrails: None,
            output_type: None,
            hooks: None,
            tool_use_behavior: None,
        }
    }

    pub async fn model_settings(
        &mut self,
        temperature: f32,
        max_tokens: u32,
        thinking: Option<Thinking>,
    ) -> &mut Self {
        self.model_settings = Some(ModelSettings {
            temperature,
            max_tokens,
            thinking,
        });
        self
    }

    pub async fn run(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();
        match self.agent_type {
            AgentType::OpenAI => {
                let model = self.model.clone().unwrap_or(DEFAULT_OPENAI_MODEL.to_string());
                let mut llm = ChatOpenAI::new(&model);
                llm = llm.with_system_prompt(&self.instructions)
                    .with_temperature(self.model_settings.as_ref().map_or(1.0, |s| s.temperature))
                    .with_max_tokens(self.model_settings.as_ref().map_or(4096, |s| s.max_tokens));

                // if let Some(thinking) = &self.model_settings {
                //     if let Some(Thinking::Enabled(_)) = thinking.thinking {
                //         llm = llm.with_thinking(thinking.thinking.clone().unwrap());
                //     }
                // }

                let response = llm.invoke(prompt).await?;

                match response.choices {
                    Some(candidates) => {
                        candidates.iter()
                            .filter_map(|candidate| candidate
                                .message.as_ref()?
                                .content.as_ref()
                            ).for_each(|content| {
                                result.push_str(content);
                            });
                    }
                    None => result = "No response choices available".to_string(),
                };
            }
            AgentType::Anthropic => {
                let model = self.model.clone().unwrap_or(DEFAULT_ANTHROPIC_MODEL.to_string());
                let mut llm = ChatAnthropic::new(&model);
                llm = llm.with_system_prompt(&self.instructions)
                    .with_temperature(self.model_settings.as_ref().map_or(1.0, |s| s.temperature))
                    .with_max_tokens(self.model_settings.as_ref().map_or(4096, |s| s.max_tokens));

                let response = llm.invoke(prompt).await?;
                
                if let Some(candidates) = response.content {
                    candidates.iter()
                        .filter_map(|c| c.text.as_ref())
                        .for_each(|text| result.push_str(text));
                } else {
                    result = "No response choices available".to_string();
                }
            }
        }
        Ok(result)
    }
}

/// A struct that represents the result of a tool call. This is used to determine whether the
/// tool call should be used as the final output or not. The result can be one of the following:
#[derive(Debug, Clone)]
pub enum ToolToFinalOutputResult {
    /// The tool call was successful, and the result should be used as the final output.
    Success(String),
    
    /// The tool call was successful, but the result should not be used as the final output.
    Ignore,
    
    /// The tool call failed, and the error message should be used as the final output.
    Error(String),
}

impl ToolToFinalOutputResult {
    /// Creates a new `ToolToFinalOutputResult` with the given success message.
    pub fn new_success(message: String) -> Self {
        Self::Success(message)
    }

    /// Creates a new `ToolToFinalOutputResult` with the given error message.
    pub fn new_error(message: String) -> Self {
        Self::Error(message)
    }
}
