use reqwest::Client;
use serde_json:: { Value, json };
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://api.cohere.com/v1/chat";
    let token = env::var("COHERE_API_KEY")
        .expect("COHERE_API_KEY environment variable not set.");

    let client = Client::new();
    let response = client
        .post(url)
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .header("Authorization", format!("bearer {}", token))
        .json(&json!({
            "message": "I’ve been using your email marketing platform for a few weeks, and I must say, the core features like campaign creation, email templates, and contact management are fantastic. The drag-and-drop editor makes it easy to design professional-looking emails, and the segmentation options help me target the right audience. However, I’ve had some issues with the mobile responsiveness of the emails. Some of my subscribers have reported that the layouts look broken on their smartphones, which is concerning. I’d love to see improvements in this area. Also, I noticed that the platform is missing some key integrations with popular CRM tools, which would be incredibly helpful for managing our leads and customers. On a positive note, the customer support team has been responsive and helpful whenever I’ve reached out with questions. Overall, it’s a great tool, but there’s definitely room for improvement in terms of mobile compatibility and third-party integrations.\n",
            "model": "command-r-plus-08-2024",
            "temperature": 0.9,
            "max_tokens": 2048,
            "preamble": "You are an AI assistant trained to categorize user feedback into predefined categories, along with sentiment analysis for each category. Your goal is to analyze each piece of feedback, assign the most relevant categories, and determine the sentiment (positive, negative, or neutral) associated with each category based on the feedback content. Predefined Categories: Product Features and Functionality Core Features Add-ons and Integrations Customization and Configuration User Experience and Design Ease of Use Navigation and Discoverability Visual Design and Aesthetics Accessibility Performance and Reliability Speed and Responsiveness Uptime and Availability Scalability Bug Fixes and Error Handling Customer Support and Service Responsiveness and Availability Knowledge and Expertise Issue Resolution and Follow-up Self-Service Resources Billing, Pricing, and Licensing Pricing Model and Tiers Billing Processes and Invoicing License Management Upgrades and Renewals Security, Compliance, and Privacy Data Protection and Confidentiality Access Control and Authentication Regulatory Compliance Incident Response and Monitoring Mobile and Cross-Platform Compatibility Mobile App Functionality Synchronization and Data Consistency Responsive Design Device and OS Compatibility Third-Party Integrations and API Integration Functionality and Reliability API Documentation and Support Customization and Extensibility Onboarding, Training, and Documentation User Guides and Tutorials In-App Guidance and Tooltips Webinars and Live Training Knowledge Base and FAQs",
            "search_queries_only": false,
        }))
        .send()
        .await?;

    let status = response.status();
    println!("Status: {}", status);

    let body = response.text().await?;
    let json: Value = serde_json::from_str(&body)?;

    if let Some(text) = json["text"].as_str() {
        println!("Response: {}", text);
    } else {
        println!("Text field not found");
    }

    Ok(())
}