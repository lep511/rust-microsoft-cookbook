use lambda_http::{Body, Error, Request, RequestExt, Response};
use rand::Rng;
use rayon::prelude::*;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    
    let mut rng = rand::thread_rng();
    
    {
        let nums: Vec<i32> = (0..10000000).map(|_| rng.gen_range(1..=100)).collect();
        let start = std::time::Instant::now();
        // 1. For loop
        let mut _nums_squared: Vec<i32> = Vec::new();
        for num in nums {
            _nums_squared.push(num * num * num);
        }
        println!("With for.. Took {:?}", start.elapsed());
    }
    {
        let nums: Vec<i32> = (0..10000000).map(|_| rng.gen_range(1..=100)).collect();
        let start = std::time::Instant::now();
        // 2. Iterator
        let _nums_squared: Vec<i32> = nums.iter()
            .map(|&x| x * x * x)
            .collect();

        println!("With iter.. Took {:?}", start.elapsed());
    }
    {
        let nums: Vec<i32> = (0..10000000).map(|_| rng.gen_range(1..=100)).collect();
        let start = std::time::Instant::now();
        // 2. Iterator
        let _nums_squared: Vec<i32> = nums.par_iter()
            .map(|&x| x * x * x)
            .collect();

        println!("With Rayon.. Took {:?}", start.elapsed());

    }

    let message = match event.query_string_parameters().first("name") {
        Some(name) => format!("Hello {}, this is an AWS Lambda HTTP request", name),
        None => "Hello world, this is an AWS Lambda HTTP request".to_string(),
    };

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use lambda_http::{Request, RequestExt};

    #[tokio::test]
    async fn test_generic_http_handler() {
        let request = Request::default();

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello world, this is an AWS Lambda HTTP request"
        );
    }

    #[tokio::test]
    async fn test_http_handler_with_query_string() {
        let mut query_string_parameters: HashMap<String, String> = HashMap::new();
        query_string_parameters.insert("name".into(), "test-128".into());

        let request = Request::default()
            .with_query_string_parameters(query_string_parameters);

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello test-128, this is an AWS Lambda HTTP request"
        );
    }
}
