use langchain::gemini::chat::ChatGemini;
use env_logger::Env;
use langchain::gemini::libs::{Part, FunctionCall, FunctionResponse, FunctionContent};
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

    let question = "Which theaters in Mountain View show Barbie movie?";
    
    let response = llm.clone()
        .with_tools(function_dec.clone())
        .with_tool_config(tool_config)
        .invoke(question)
        .await?;

    let mut function_name = String::new();
    let mut movie = String::new();
    let mut location = String::new();

    println!("Question: {}", question);
    if let Some(candidates) = &response.candidates {
        for candidate in candidates {
            if let Some(content) = &candidate.content {
                for part in &content.parts {
                    if let Some(function_call) = &part.function_call {
                        function_name = function_call.name.clone();
                        movie = function_call.args.get("movie").unwrap().to_string();
                        location = function_call.args.get("location").unwrap().to_string();
                        println!("Function name: {}", function_name);
                        println!("Movie: {}", movie);
                        println!("Location: {}", location);
                    }
                }
            }
        }
    };

    let function_call_assistant = FunctionCall {
        name: function_name.clone(),
        args: json!({
            "location": location,
            "movie": movie
        }),
    };

    let assistant_response = Part {
        text: None,
        function_call: Some(function_call_assistant),
        function_response: None,
        inline_data: None,
        file_data: None,
    };

    let content_response = json!({
        "movie":"Barbie",
        "theaters":[
            {
                "name":"AMC Mountain View 16",
                "address":"2000 W El Camino Real, Mountain View, CA 94040"
            },
            {
                "name":"Regal Edwards 14",
                "address":"245 Castro St, Mountain View, CA 94040"
            }
        ]
    });

    let function_content = FunctionContent {
        name: function_name.clone(),
        content: content_response,
    };

    let function_response = FunctionResponse {
        name: function_name,
        response: function_content,
    };

    let response = llm
        .with_assistant_response(vec![assistant_response])
        .with_function_response(function_response)
        .with_tools(function_dec)
        .invoke(question)
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
    // example_chat().await?;
    example_tools().await?;
    Ok(())
}