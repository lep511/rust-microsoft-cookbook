use lambda_http::{Body, Error, Request, RequestExt};
use aws_lambda_events::event::apigw::ApiGatewayV2httpResponse as Response;
use http::header::{HeaderMap, HeaderValue};
use askama::Template;
use serde::{Deserialize, Serialize};

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
struct HelloTemplate<'a> {
    title: &'a str,
    name: &'a str,
    items: Vec<&'a str>,
}

#[derive(Deserialize)]
struct QueryParams {
    name: Option<String>,
}

pub(crate) async fn function_handler(event: Request) -> Result<Response, Error> {

    // Render template
    let template = HelloTemplate {
        title: "Askama Demo",
        name: "World",
        items: vec!["Item 1", "Item 2", "Item 3"],
    };
    
    let html = template.render()?;
    let body = Some(Body::Text(html));

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));
    headers.insert("X-Custom-Header", HeaderValue::from_static("custom-value"));

    let cookies = vec![
        "session=abc123; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=3600".to_string(),
        "user=johndoe; Path=/; Max-Age=86400".to_string(),
    ];
    
    // Return response
    let resp = Response {
        status_code: 200,
        headers: headers,
        multi_value_headers: HeaderMap::new(),
        body: body,
        is_base64_encoded: false,
        cookies: cookies,
    };
    
    Ok(resp)
}