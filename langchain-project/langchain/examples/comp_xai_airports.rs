#[allow(dead_code)]
use langchain::compatible::chat::ChatCompatible;
use langchain::compatible::libs::ChatResponse;
use env_logger::Env;

async fn generate_ranom_cities() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let endpoint_url = "https://api.x.ai/v1";
    let model = "grok-2-latest";
    let llm = ChatCompatible::new(endpoint_url, model);

    let prompt = "Generate 5 random cities from america, europe and asia \
                  Provide the cities in the format: CITY, separated by commas. \
                  Only include the city names, without any additional text.";

    let response: ChatResponse = llm.invoke(prompt).await?;

    let mut cities = Vec::new();

    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                if let Some(message) = candidate.message {
                    if let Some(content) = message.content {
                        let city_list = content.split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
                        cities.extend(city_list);
                    }
                }
            }
        }
        None => println!("No response choices available"),
    };

    Ok(cities)
}

async fn airport_code_extractor(
    city_one: &str,
    city_two: &str,
    city_three: &str,
    city_four: &str,
    city_five: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    let endpoint_url = "https://api.x.ai/v1";
    let model = "grok-2-latest";
    let llm = ChatCompatible::new(endpoint_url, model);

    let system_prompt = "You are an advanced AI assistant specialized in travel information. \
                    Your task is to analyze a given travel itinerary and identify the airport \
                    codes for the cities mentioned.";

    let prompt = format!(
        "Given the following travel itinerary, identify the airport codes for the cities: \
        {}, {}, {}, {}, and {}.\n \
        Provide the airport codes in the format: CITY:CODE, separated by commas. Before providing \
        the final list, wrap your thought process in <thinking> tags. In this section:\n\n \
        1. List each city mentioned in the itinerary with a number.\n \
        2. For each city, write down potential airport codes and choose the most likely one.\n \
        3. Explain the reasoning behind each choice.",
        city_one, 
        city_two, 
        city_three, 
        city_four, 
        city_five,
    );

    let response: ChatResponse = llm
        .with_system_prompt(system_prompt)
        .invoke(&prompt)
        .await?;

    match response.choices {
        Some(candidates) => {
            for candidate in candidates {
                #[allow(irrefutable_let_patterns)]
                if let Some(message) = candidate.message {
                    if let Some(content) = message.content {
                        println!("{}", content);
                    }
                }
            }
        }
        None => println!("No response choices available"),
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cities = generate_ranom_cities().await?;

    let [city_one, city_two, city_three, city_four, city_five] = cities.as_slice() else {
        println!("Cities: {:?}", cities);
        panic!("Vector length mismatch");
    };

    airport_code_extractor(
        city_one,
        city_two,
        city_three,
        city_four,
        city_five,
    ).await?;

    Ok(())
}