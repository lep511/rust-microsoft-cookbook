#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use futures::StreamExt;
use futures::pin_mut;
use std::fs;
use std::path::Path;
use env_logger::Env;

pub struct Metric {
    name: String,
    criteria: String,
    steps: String,
}

pub enum EvalType {
    Relevance,
    Coherence,
    Consistency,
    Fluency,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("error")).init();

    let endpoint_url = "https://integrate.api.nvidia.com/v1";
    let model = "deepseek-ai/deepseek-r1";
    
    // Files path
    let document_path = Path::new("tests/files/article.txt");
    let summary_path = Path::new("tests/files/article-summary.txt");

    let document = fs::read_to_string(document_path)?;
    let summary = fs::read_to_string(summary_path)?;

    let metrics = [EvalType::Relevance, EvalType::Coherence, 
                EvalType::Consistency, EvalType::Fluency];
    
    for metric in metrics {
        let llm = ChatCompatible::new(endpoint_url, model);
        let prompt = generate_prompt(metric, &document, &summary);
        let stream = llm
            .stream_response(prompt);
        
        pin_mut!(stream);

        while let Some(stream_response) = stream.next().await {
            if let Some(choices) = stream_response.choices {
                for choice in choices {
                    if let Some(delta) = choice.delta {
                        if let Some(content) = delta.content {
                            if content.is_empty() {
                                continue;
                            }
                            print!("{}", content);
                        }
                    }
                }
            };
        }
        println!("\n\n");
    }
    
    Ok(())
}

pub fn generate_prompt(
    metric: EvalType,
    document: &str, 
    summary: &str,
) -> String {
    
    let metric = handler_test(metric);
    let criteria = &metric.criteria;
    let steps = &metric.steps;
    let metric_name = &metric.name;

    let evaluation_prompt = format!("You will be given one summary written for an article. \
            Your task is to rate the summary on one metric. Please make sure you read and \
            understand these instructions very carefully. Please keep this document open \
            while reviewing, and refer to it as needed. \
            Evaluation Criteria: \
            \n \
            {criteria} \
            \n \
            Evaluation Steps: \
            \n \
            {steps} \
            \n \
            Example: \
            \n \
            Source Text:
            \n \
            {document} \
            \n \
            Summary: \
            \n \
            {summary} \
            \n \
            Evaluation Form (scores ONLY): \
            \n \
            - {metric_name}");
    
    evaluation_prompt
}

pub fn handler_test(metric: EvalType) -> Metric {
    match metric {
        EvalType::Relevance => relevance_metric(),
        EvalType::Coherence => coherence_metric(),
        EvalType::Consistency => consistency_metric(),
        EvalType::Fluency => fluency_metric(),
    }
}

pub fn relevance_metric() -> Metric {
    let criteria = "Relevance(1-10) - selection of important content from the source. \
                    The summary should include only important information from the source document. \
                    Annotators were instructed to penalize summaries which contained redundancies \
                    and excess information.";

    let steps = "1. Read the summary and the source document carefully. \n \
                2. Compare the summary to the source document and identify \
                the main points of the article. \n \
                3. Assess how well the summary covers the main points of the article, \
                and how much irrelevant or redundant information it contains. \n \
                4. Assign a relevance score from 1 to 10.";

    let metric_name = "Relevance";

    Metric {
        name: metric_name.to_string(),
        criteria: criteria.to_string(),
        steps: steps.to_string(),
    }
}

pub fn coherence_metric() -> Metric {
    let criteria = "Coherence(1-10) - the collective quality of all sentences. \
                We align this dimension with the DUC quality question of structure and coherence \
                whereby the summary should be well-structured and well-organized. \
                The summary should not just be a heap of related information, but should build from sentence to a \
                coherent body of information about a topic.";

    let steps = "1. Read the article carefully and identify the main topic and key points. \n \
                2. Read the summary and compare it to the article. Check if the summary covers \
                the main topic and key points of the article, and if it presents them in a clear \
                and logical order. \n \
                3. Assign a score for coherence on a scale of 1 to 10, where 1 is the lowest and 10 is \
                the highest based on the Evaluation Criteria.";

    let metric_name = "Coherence";

    Metric {
        name: metric_name.to_string(),
        criteria: criteria.to_string(),
        steps: steps.to_string(),
    }
}

pub fn consistency_metric() -> Metric {
    let criteria = "Consistency(1-10) - the quality of the summary in terms of factuality and logical consistency. \
                We align this dimension with the DUC quality question of factuality and logical consistency \
                whereby the summary should be factually correct and logically coherent. \
                The summary should not just be a heap of related information, but should build from sentence to a \
                coherent body of information about a topic.";

    let steps = "1. Read the article carefully and identify the main topic and key points. \n \
                2. Read the summary and compare it to the article. Check if the summary covers \
                the main topic and key points of the article, and if it presents them in a clear \
                and logical order. \n \
                3. Assign a score for coherence on a scale of 1 to 10, where 1 is the lowest and 10 is \
                the highest based on the Evaluation Criteria.";

    let metric_name = "Consistency";

    Metric {
        name: metric_name.to_string(),
        criteria: criteria.to_string(),
        steps: steps.to_string(),
    }
}

pub fn fluency_metric() -> Metric {
    let criteria = "Fluency(1-10) - the quality of the summary in terms of factuality and logical consistency. \
                We align this dimension with the DUC quality question of factuality and logical consistency \
                whereby the summary should be factually correct and logically coherent. \
                The summary should not just be a heap of related information, but should build from sentence to a \
                coherent body of information about a topic.";

    let steps = "1. Read the article carefully and identify the main topic and key points. \n \
                2. Read the summary and compare it to the article. Check if the summary covers \
                the main topic and key points of the article, and if it presents them in a clear \
                and logical order. \n \
                3. Assign a score for coherence on a scale of 1 to 10, where 1 is the lowest and 10 is \
                the highest based on the Evaluation Criteria.";

    let metric_name = "Fluency";

    Metric {
        name: metric_name.to_string(),
        criteria: criteria.to_string(),
        steps: steps.to_string(),
    }
}