use url::Url;
use std::error::Error;

fn extract_query_params(url_str: &str) -> Result<(String, String), Box<dyn Error>> {
    // Parse the URL
    let url = Url::parse(url_str)?;

    // Get the query pairs as a HashMap-like structure
    let query_pairs: Vec<_> = url.query_pairs().collect();

    // Extract 'iss' and 'launch' values
    let iss = query_pairs
        .iter()
        .find(|(key, _)| key == "iss")
        .map(|(_, value)| value.to_string())
        .ok_or("Missing 'iss' parameter")?;

    let launch = query_pairs
        .iter()
        .find(|(key, _)| key == "launch")
        .map(|(_, value)| value.to_string())
        .ok_or("Missing 'launch' parameter")?;

    Ok((iss, launch))
}

fn main() -> Result<(), Box<dyn Error>> {
    let url_str = "https://7tyg9r9mt8.execute-api.us-east-1.amazonaws.com/medical/launch?iss=https://app.meldrx.com/api/fhir/396412f4-286b-4e44-8bae-28c1357e5695&launch=CfDJ8HpS9eSTt9RIg12_TfU5L5f7JjyQrjTlZCdj76Dvh7CFkpdYJcHdLbi7Lys8pWs61eXkwgvntSHwgIRFitqmLKgbX1Blbdca8XzLlmKo4NrrO4sQ0s0yp72F8s3ovL6ZZxZaG4ZP8yqz2aovEyixbZjyiv2w5d7s6pAgRbwYXImzpM0NytkvYF6xIzTmSqaX8YHS1UwQrZMUllE5616Aj7iosCau4iWFzPnAOkECfgF03Fnf8lSLvPWT4dbxZ-OZni9yJcLPV6mYf8d9P4Ew03pX01casy-Vh-9SI6GNo2UmxfqJQkP8XSMtumMf152JCgX0qBD2_JJK2Ws463Us4c5M6JtHODB1znGCT3k4qfIdEPaUdfZDUQv_czuWpcyQCYcYbQ0p-pyEne4SfFAdCFZfx6Uu9akcSzTrz1zpEgjhJuctEJGY-2Y4EV1dzH7r-J_H5NzzW48d0YwhwIJwQkDbMjkct7bW7NXASA1iQ4apv-RemrQANEQifx_nUf6frFrCRW1UWjYcRU9oGE9UCvERUEgG7Cz-dVayEao6OHCx";

    let (iss, launch) = extract_query_params(url_str)?;

    println!("iss: {}", iss);
    println!("launch: {}", launch);

    Ok(())
}