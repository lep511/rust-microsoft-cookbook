#[allow(dead_code)]
use langchain::anthropic::{EmbedRankVoyage, EmbedResponse};
use langchain::anthropic::chat::ChatAnthropic;
use std::cmp::Ordering::Equal;

fn get_highest_relevance_score(response: &EmbedResponse) -> Option<(f64, usize)> {
    response.data.as_ref()
        .and_then(|data| data.iter()
            .filter_map(|item| {
                item.relevance_score
                    .map(|score| (score, item.index.unwrap_or(0)))
            })
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Equal))
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = EmbedRankVoyage::new("rerank-lite-1")?;

    let doc1 = "Document 1: \
                One of the most serious constitutional responsibilities a \
                President has is nominating someone to serve on the \
                United States Supreme Court. \
                \
                And I did that 4 days ago, when I nominated Circuit Court of \
                Appeals Judge Ketanji Brown Jackson. One of our nation's top \
                legal minds, who will continue Justice \
                Breyer's legacy of excellence.".to_string();
    
    let doc2 = "Document 2: \
                \
                My plan to fight inflation will lower your costs and lower the deficit. \
                \
                17 Nobel laureates in economics say my plan will ease long-term \
                inflationary pressures. Top business leaders and most Americans \
                support my plan. And here’s the plan: \
                \
                First cut the cost of prescription drugs. Just look at insulin. \
                One in ten Americans has diabetes. In Virginia, \
                I met a 13-year-old boy named Joshua Davis.".to_string();

    let doc3 = "Document 3: \
                \
                As Ohio Senator Sherrod Brown says, <It's time to bury the label Rust Belt.> \
                But with all the bright spots in our economy, record job growth and higher \
                wages, too many families are struggling to keep up with the bills. \
                \
                Inflation is robbing them of the gains they might otherwise feel. \
                I get it. That's why my top priority is \
                getting prices under control.".to_string();

    let documents = vec![
        doc1,
        doc2,
        doc3,
    ];

    let llm = llm.with_documents(documents.clone());

    // let query = "What did the president say about Ketanji Jackson Brown";
    // let query = "What did the president say about the Rust Belt";
    let query = "What did the president say about inflation";
    let response = llm.embed_content(query).await?;

    // println!("Response: {:?}", response);

    if let Some((score, index)) = get_highest_relevance_score(&response) {
        println!("Question: {}", query);
        println!("Highest relevance score: {} at index: {}", score, index);
        let llm = ChatAnthropic::new("claude-3-5-sonnet-20241022")?;
        let prompt = format!("Based on the following document. Respond to the query. \
            Query: {query}. \
            Document: {documents}. \
            Answer the question using only this document.",
            query=query,
            documents=documents[index]
        );
        let response = llm.invoke(&prompt).await?;
        if let Some(candidates) = response.content {
            for candidate in candidates {
                match candidate.text {
                    Some(text) => println!("{}", text),
                    None => println!(""),
                }
            }
        };
    }

    Ok(())
}