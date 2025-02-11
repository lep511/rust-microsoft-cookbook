#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use env_logger::Env;
use langchain::gemini::libs::Part;

#[derive(Debug)]
struct SampleCase<'a> {
    text: &'a str,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatGemini::new("gemini-2.0-flash-exp");

    let prompt = "input: We prepared our impairment test as of December 2022 and determined that the fair \
            values of each of our reporting units exceeded net book value by more than 25%. \
            Among our reporting units, the narrowest difference between the calculated fair \
            value and net book value was in our Principal Markets segment’s Canada reporting unit, \
            whose calculated fair value exceeded its net book value by 26.5%. Future developments \
            related to macroeconomic factors, including increases to the discount rate used, or \
            changes to other inputs and assumptions, including revenue growth, could reduce the \
            fair value of this and/or other reporting units and lead to impairment. There were \
            no goodwill impairment losses recorded for the nine months ended December 31, 2022. \
            Cumulatively, the Company has recorded $234.5 million in goodwill impairment charges \
            within its former EMEA ($146.5 million) and current United States ($88 million) \
            reporting units. Revolving Credit Agreement In December 2021, we entered into a $2 \
            billion multi-currency revolving credit agreement (the “Revolving Credit Agreement”) \
            for our future liquidity needs. The Revolving Credit Agreement expires, unless extended, \
            in October 2026. Interest rates on borrowings under the Revolving Credit Agreement \
            are based on prevailing market interest rates, plus a margin, as further described \
            in the Revolving Credit Agreement. The total expense recorded by the Company for the \
            Revolving Credit Agreement was not material in any of the periods presented. We may \
            voluntarily prepay borrowings under the Revolving Credit Agreement without premium or \
            penalty, subject to customary “breakage” costs. The Revolving Credit Agreement includes \
            certain customary mandatory prepayment provisions. Interest on Debt Interest expense for \
            the three and nine months ended December 31, 2022 was $13.5 million and $32.5 million, \
            compared to $9 million and $25 million for the three and nine months 
            ended December 31, 2021. ";
    
    let system_prompt = "Extract the line of credit facility maximum borrowing capacity from the 10-K filing. Only show the output value.";
    
    let sample_cases = vec![
        SampleCase {
            text: "input: The credit agreement also provides that up to $250 million in commitments may be used for letters of credit.",
        },
        SampleCase {
            text: "output: $250M",
        },
        SampleCase {
            text: "input: In March 2021, we upsized the Credit Agreement by $50 million, which matures July 2023, to $1.725 billion.         ",
        },
        SampleCase {
            text: "output: $1.725B",
        },
    ];

    let mut parts: Vec<Part> = Vec::new();
    for case in sample_cases {
        let part = Part {
            text: Some(case.text.to_string()),
            function_call: None,
            function_response: None,
            inline_data: None,
            file_data: None,
        };
        parts.push(part);
    }

    let response = llm
        .with_system_prompt(system_prompt)
        .with_multiple_parts(parts)    
        .invoke(prompt)
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