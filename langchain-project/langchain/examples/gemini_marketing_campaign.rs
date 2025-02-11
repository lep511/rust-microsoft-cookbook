use langchain::gemini::chat::ChatGemini;
use env_logger::Env;
use langchain::gemini::utils::{
    generate_schema, get_grounding_response,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct MarketingCampaignBrief {
    campaign_name: String,
    campaign_objectives: Vec<String>,
    target_audience: String,
    media_strategy: Vec<String>,
    timeline: String,
    target_countries: Vec<String>,
    performance_metrics: Vec<String>,
}

#[allow(dead_code)]
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct AdCopy {
    ad_copy_options: Vec<String>,
    localization_notes: Vec<String>,
    visual_description: Vec<String>,
}

async fn marketing_brief() -> Result<String, Box<dyn std::error::Error>> {

    let llm = ChatGemini::new("gemini-2.0-flash-exp");
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

    let llm = ChatGemini::new("gemini-2.0-flash-exp");

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

async fn creative_brief(
    marketing_brief: &str, 
    market_research: &str
) -> Result<String, Box<dyn std::error::Error>> {

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
            marketing_brief, 
            market_research, 
            new_phone_details
    );

    let llm = ChatGemini::new("gemini-2.0-flash-exp");

    // Generate schema
    let schema = schemars::schema_for!(MarketingCampaignBrief);
    let json_schema = generate_schema(schema ,false)?;

    let response = llm
        .with_json_schema(json_schema)
        .invoke(&brief_prompt)
        .await?;

    let mut creative_brief_content = String::from("");

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                        creative_brief_content.push_str(&text);
                    }
                }
            }
        }
    };

    Ok(creative_brief_content)
}

async fn create_assets(
    countries: &str, 
    creative_brief_content: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp");
    
    let ad_prompt = format!("Given the marketing campaign brief, create an Instagram ad-copy \
                        for each target market: {} \
                        Please localize the ad-copy and the visuals to the target markets \
                        for better relevancy to the target audience. \
                        Marketing Campaign Brief: {}",
                        countries,
                        creative_brief_content,
    );

    // Generate schema for an ad copy
    let schema = schemars::schema_for!(AdCopy);
    let json_schema = generate_schema(schema ,false)?;

    let response = llm
        .with_json_schema(json_schema)
        .invoke(&ad_prompt)
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

async fn create_storyboard(
    countries: &str, 
    creative_brief_content: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp");
    
    let short_video_prompt = format!("Given the marketing campaign brief, create a storyboard \
                        for a YouTube Shorts video for target markets: {}. Please localize the \
                        content to the target markets for better relevancy to the target audience. \
                        Marketing Campaign Brief: {}",
                        countries,
                        creative_brief_content,
    );

    let response = llm
        .invoke(&short_video_prompt)
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let marketing_brief_content: String = marketing_brief().await?;
    let market_research_content: String = market_research().await?;

    let creative_brief_content: String = creative_brief(
        &marketing_brief_content,
        &market_research_content
    ).await?;

    // ~~~~~~~~~~~~~~~~~~~ Creating Assets for the Marketing Campaign ~~~~~~~~~~~~~~~~~~~

    let campaign_brief: MarketingCampaignBrief = serde_json::from_str(&creative_brief_content)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let countries = campaign_brief.target_countries.join(", ");

    create_assets(&countries, &creative_brief_content).await?;

    create_storyboard(&countries, &creative_brief_content).await?;

    Ok(())
}