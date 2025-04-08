use langchain::agents::financials_agents::create_advisor_agent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Create the financials agent
    let financial_agent = create_advisor_agent().await;
    // Print the agent's name and prompt
    println!("Agent Name: {}", financial_agent.name);
    println!("Agent Prompt: {}", financial_agent.instructions);

    let web_search = "Tesla in 2025 - Tesla Inc.'s stock extended losses Monday, dropping \
            below a price at which Commerce Secretary Howard Lutnick predicted they'd never \
            fall to again. \
            \
            The shares plunged as much as 9.2% to $217.41 as of 9:41 a.m. in New York, amid a \
            broader selloff in global equity markets. Lutnick said during a Fox News interview  \
            on March 19 — when Tesla closed at $235.86 — that viewers should buy the stock,  \
            saying “it'll never be this cheap again.\" Chief Executive Officer Elon Musk told  \
            Tesla employees the following day that they should hang on to their shares. \
            \
            The latest decline comes after one of Tesla's biggest bulls — Wedbush Securities  \
            analyst Daniel Ives — slashed his price target on the stock by more than 40%,  \
            citing Trump's trade policies and a brand crisis created by Musk. \
            \
            Tesla shares have fallen 55% from a record high reached in mid-December.  \
            The stock had surged following Trump's election victory, which many expected  \
            to be a boon for the company, given Musk's proximity to the then president-elect.  \
            Instead, Musk's involvement in political controversies both in the U.S. and abroad  \
            has repelled some car buyers and spurred protests against the company. \
            \
            Last week, Tesla reported first-quarter vehicle deliveries that failed to meet  \
            drastically lowered expectations, falling to the lowest level since 2022.  \
            JPMorgan Chase & Co.'s Ryan Brinkman — one of Wall Street's most bearish  \
            analysts on the stock — said that he may have underestimated the degree of  \
            consumer reaction and \"unprecedented brand damage.\" \
            \
            Several analysts have lowered their estimates for Tesla's sales and earnings  \
            in recent weeks, even before the company reported weak vehicle-delivery numbers.  \
            And while Tesla is seen as relatively insulated from the 25% tariffs announced  \
            by Trump on imported autos, Musk has warned the company won't be unscathed. \
            \
            “The tariffs in their current form will disrupt Tesla, the overall supply chain,  \
            and its global footprint which has been a clear advantage over the years versus  \
            rising competitors like BYD,\" Wedbush's Ives said in a note to clients on Sunday. \
            \
            The bigger worry, according to Ives, is Tesla's position in China. \
            \
            “The backlash from Trump tariff policies in China and Musk's association will \
            be hard to understate, and this will further drive Chinese consumers to buy \
            domestic such as BYD, Nio, Xpeng, and others,\" Ives wrote.";

    financial_agent
        .run(web_search)
        .await
        .map(|response| println!("Agent Response: {}", response))
        .unwrap_or_else(|err| println!("Error invoking agent: {}", err));
    
    Ok(())
}