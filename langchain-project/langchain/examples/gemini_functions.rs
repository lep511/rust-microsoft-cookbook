use langchain::gemini::chatgemini::ChatGemini;
use serde_json::json;
use std::fs;

async fn example_chat() -> Result<(), Box<dyn std::error::Error>> {
    let file_txt = "tests/files/note.txt";
    let contents = fs::read_to_string(file_txt)?;

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let base_prompt = "Please return JSON describing the the people, places, \
        things and relationships from this story using the following schema: \
        \
        {\"people\": list[PERSON], \"places\":list[PLACE], \"things\":list[THING], \"relationships\": list[RELATIONSHIP]} \
        \
        PERSON = {\"name\": str, \"description\": str, \"start_place_name\": str, \"end_place_name\": str} \
        PLACE = {\"name\": str, \"description\": str} \
        THING = {\"name\": str, \"description\": str, \"start_place_name\": str, \"end_place_name\": str} \
        RELATIONSHIP = {\"person_1_name\": str, \"person_2_name\": str, \"relationship\": str} \
        \
        All fields are required. \
        \
        Important: Only return a single piece of valid JSON text. \
        \
        Here is the story:
        \
        ";
    
    let prompt = format!("{} {}", base_prompt, contents);
    let response = llm
        .invoke(&prompt)
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

async fn example_tools() -> Result<(), Box<dyn std::error::Error>> {

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;
    
    let function1 = json!({
        "name":"find_movies",
        "description":"find movie titles currently playing in theaters based on any description, genre, title words, etc.",
        "parameters":{
            "type":"OBJECT",
            "properties":{
                "location":{
                    "type":"STRING",
                    "description":"The city and state, e.g. San Francisco, CA or a zip code e.g. 95616"
                },
                "description":{
                    "type":"STRING",
                    "description":"Any kind of description including category or genre, title words, attributes, etc."
                }
            },
            "required":[
                "description"
            ]
        }
    });

    let function2 = json!({
        "name":"find_theaters",
        "description":"find theaters based on location and optionally movie title which is currently playing in theaters",
        "parameters":{
            "type":"OBJECT",
            "properties":{
                "location":{
                    "type":"STRING",
                    "description":"The city and state, e.g. San Francisco, CA or a zip code e.g. 95616"
                },
                "movie":{
                    "type":"STRING",
                    "description":"Any movie title"
                }
            },
            "required":[
                "location"
            ]
        }
    });

    let function3 = json!({
        "name":"get_showtimes",
        "description":"Find the start times for movies playing in a specific theater",
        "parameters":{
            "type":"OBJECT",
            "properties":{
                "location":{
                    "type":"STRING",
                    "description":"The city and state, e.g. San Francisco, CA or a zip code e.g. 95616"
                },
                "movie":{
                    "type":"STRING",
                    "description":"Any movie title"
                },
                "theater":{
                    "type":"STRING",
                    "description":"Name of the theater"
                },
                "date":{
                    "type":"STRING",
                    "description":"Date for requested showtime"
                }
            },
            "required":[
                "location",
                "movie",
                "theater",
                "date"
            ]
        }
    });

    let function_dec = vec![json!({
        "functionDeclarations":[
            function1,
            function2,
            function3
        ]
    })];

    let tool_config = json!({
        "function_calling_config":{
            "mode":"ANY",
            "allowed_function_names":[
                "find_theaters",
                "get_showtimes"
            ]
        }
    });

    let question = "What movies are showing in North Seattle tonight?";
    
    let response = llm
        .with_tools(function_dec)
        .with_tool_config(tool_config)
        .invoke(question)
        .await?;

    println!("Question: {}", question);
    if let Some(candidates) = &response.candidates {
        for candidate in candidates {
            if let Some(content) = &candidate.content {
                for part in &content.parts {
                    if let Some(function_call) = &part.function_call {
                        println!("Function name: {}", function_call.name);
                        println!("Location: {}", function_call.args.get("location").unwrap_or(&json!("{}")));
                    }
                }
            }
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // example_chat().await?;
    example_tools().await?;
    Ok(())
}