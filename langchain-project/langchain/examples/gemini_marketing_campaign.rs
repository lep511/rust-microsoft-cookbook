use langchain::gemini::chat::ChatGemini;
use langchain::gemini::utils::{
    generate_schema, get_grounding_response,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct MarketingCampaignBrief {
    campaign_name: String,
    campaign_objectives: Vec<String>,
    target_audience: String,
    media_strategy: Vec<String>,
    timeline: String,
    target_countries: Vec<String>,
    performance_metrics: Vec<String>,
}

async fn marketing_brief() -> Result<String, Box<dyn std::error::Error>> {

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;
    let file_path = Some("tests/files/sample_marketing_campaign.pdf");
    let upload_data = None;
    let display_name = "campaign.pdf";
    let mime_type = "auto";

    // Generate schema
    let schema = schemars::schema_for!(MarketingCampaignBrief);
    let json_schema = generate_schema(schema ,false)?;

    let prompt = "Extract the details from the sample marketing brief.";

    let response = llm
        .with_json_schema(json_schema)
        .media_upload(
            file_path,
            upload_data,
            display_name,
            mime_type,
        )
        .await?
        .invoke(prompt)
        .await?;

    let mut response_string = String::from("");
    
    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                        response_string.push_str(&text);
                    }
                }
            }
        }
    };

    Ok(response_string)
}

async fn market_research() -> Result<String, Box<dyn std::error::Error>> {
    // Use Grounding with Google Search to do market research
    let market_prompt = "I am planning to launch a mobile phone campaign and I want \
                        to understand the latest trends in the phone industry. Please answer \
                        the following questions: \
                        - What are the latest phone models and their selling point from the top 2 phone makers?
                        - What is the general public sentiment about mobile phones?";

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let response = llm
        .with_google_search()
        .invoke(market_prompt)
        .await?;

    let mut response_string = String::from("");
    let mut metadata_string = String::from("");

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            metadata_string = get_grounding_response(&candidate);
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                        response_string.push_str(&text);
                    }
                }
            }
        }
    };

    println!("Metadata: {}", metadata_string);

    Ok(response_string)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let marketing_brief_content: String = marketing_brief().await?;
    let market_research_content: String = market_research().await?;

    let new_phone_details = "Phone Name: Pix Phone 10 \
                    Short description: Pix Phone 10 is the flagship phone with a \
                    focus on AI-powered features and a completely redesigned form factor.\
                    \
                    Tech Specs: \
                        - Camera: 50MP main sensor with 48MP ultrawide lens with autofocus for macro shots \
                        - Performance: P5 processor for fast performance and AI capabilities \
                        - Battery: 4700mAh battery for all-day usage \
                    \
                    Key Highlights: \
                        - Powerful camera system \
                        - Redesigned software user experience to introduce more fun \
                        - Compact form factor \
                    Launch timeline: Jan 2025 \
                    Target countries: US, France and Japan";

    let brief_prompt = format!(
        "Given the following details, create a marketing campaign brief for the new phone launch: \
        Sample campaign brief: \
        {} \
        \
        Market research: \
        {} \
        \
        New phone details: \
        {}",
         marketing_brief_content, 
         market_research_content, 
         new_phone_details
    );

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    // Generate schema
    let schema = schemars::schema_for!(MarketingCampaignBrief);
    let json_schema = generate_schema(schema ,false)?;

    let response = llm
        .with_json_schema(json_schema)
        .invoke(&brief_prompt)
        .await?;

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                    }
                }
            }
        }
    };

    Ok(())
}