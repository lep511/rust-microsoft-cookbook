use lambda_runtime::{Error, LambdaEvent};
use aws_lambda_events::event::apigw::{
    ApiGatewayV2httpRequest, ApiGatewayV2httpResponse,
};
use http::header::{HeaderMap, HeaderValue};
use aws_lambda_events::encodings::Body;
use serde::{Deserialize, Serialize};
use askama::Template;

// Define a struct with the template embedded directly in code
#[derive(Template)]
#[template(source = r#"
<!DOCTYPE html>
<html>
<head>
    <title>{{ title }}</title>
</head>
<body>
    <h1>Hello, {{ name }}!</h1>
    {% if items.len() > 0 %}
    <ul>
        {% for item in items %}
        <li>{{ item }}</li>
        {% endfor %}
    </ul>
    {% else %}
    <p>No items found.</p>
    {% endif %}
</body>
</html>
"#, ext = "html")]
pub(crate) struct HelloTemplate<'a> {
    title: &'a str,
    name: &'a str,
    items: Vec<&'a str>,
}

pub(crate) async fn function_handler(
    event: LambdaEvent<ApiGatewayV2httpRequest>,
) -> Result<ApiGatewayV2httpResponse, Error> {
    // Extract the request from the event
    let request = event.payload;

    let template = HelloTemplate {
        title: "Askama Demo",
        name: "World",
        items: vec!["Item 1", "Item 2", "Item 3"],
    };

    let html = template.render().unwrap_or_else(|e| {
        format!("Error rendering template: {}", e)
    });
    let body = Body::Text(html);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));
    headers.insert("X-Custom-Header", HeaderValue::from_static("custom-value"));

    let cookies = vec![
        "session=abc123; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=3600".to_string(),
        "user=johndoe; Path=/; Max-Age=86400".to_string(),
    ];

    // Create a response
    let resp = ApiGatewayV2httpResponse {
        status_code: 200,
        headers: headers,
        multi_value_headers: HeaderMap::new(),
        body: Some(body),
        cookies: cookies,
        is_base64_encoded: false,
    };

    Ok(resp)
}