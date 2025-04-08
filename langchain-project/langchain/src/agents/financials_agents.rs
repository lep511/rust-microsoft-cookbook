use crate::agents::libs::{Agent, AgentType};
use crate::anthropic::chat::ChatAnthropic;
use crate::openai::chat::ChatOpenAI;

const DEFAULT_OPENAI_MODEL: &str = "gpt-4.5-preview";
const DEFAULT_ANTHROPIC_MODEL: &str = "claude-3";

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ FundamentalsAnalystAgent ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

const FINANCIALS_PROMPT: &str = 
    "You are a financial analyst focused on company fundamentals such as revenue, \
    profit, margins and growth trajectory. Given a collection of web (and optional file) \
    search results about a company, write a concise analysis of its recent financial \
    performance. Pull out key metrics or quotes. Keep it under 2 paragraphs.";

pub async fn create_financials_agent() -> Agent {
    Agent::new(
        "FundamentalsAnalystAgent".to_string(),
        FINANCIALS_PROMPT.to_string(),
    ).await
}

pub async fn run_financials_agent(
    agent: &Agent,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = String::new();
    match agent.agent_type {
        AgentType::OpenAI => {
            let model = agent.model.clone().unwrap_or(DEFAULT_OPENAI_MODEL.to_string());
            let llm = ChatOpenAI::new(&model);
            let response = llm.invoke(&agent.instructions).await?;
            
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
            let model = agent.model.clone().unwrap_or(DEFAULT_ANTHROPIC_MODEL.to_string());
            let llm = ChatAnthropic::new(&model);
            let response = llm.invoke(&agent.instructions).await?;
        }
    }
    Ok(result)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ FinancialPlannerAgent ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

const PLANNER_PROMPT: &str = 
    "You are a financial research planner. Given a request for financial analysis, \
    produce a set of web searches to gather the context needed. Aim for recent \
    headlines, earnings calls or 10-K snippets, analyst commentary, and industry background. \
    Output between 5 and 15 search terms to query for.";

pub async fn create_planner_agent() -> Agent {
    Agent::new(
        "FinancialPlannerAgent".to_string(),
        PLANNER_PROMPT.to_string(),
    ).await
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ RiskAnalystAgent ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

const RISK_PROMPT: &str = 
    "You are a risk analyst looking for potential red flags in a company's outlook.  \
    Given background research, produce a short analysis of risks such as competitive threats, \
    regulatory issues, supply chain problems, or slowing growth. Keep it under 2 paragraphs.";

pub async fn create_risk_agent() -> Agent {
    Agent::new(
        "RiskAnalystAgent".to_string(),
        RISK_PROMPT.to_string(),
    ).await
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ FinancialSearchAgent ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

const SEARCH_PROMPT: &str = 
    "You are a research assistant specializing in financial topics. \
    Given a search term, use web search to retrieve up‑to‑date context and \
    produce a short summary of at most 300 words. Focus on key numbers, events, \
    or quotes that will be useful to a financial analyst.";

pub async fn create_search_agent() -> Agent {
    Agent::new(
        "FinancialSearchAgent".to_string(),
        SEARCH_PROMPT.to_string(),
    ).await
}