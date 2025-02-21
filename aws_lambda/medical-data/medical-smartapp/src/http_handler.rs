use lambda_http::{Body, Error, Request, RequestExt, Response};
use crate::http_page::get_http_page;
use lambda_http::tracing::{error, info};

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("Event: {:?}", event);
    // let who = event
    //     .query_string_parameters_ref()
    //     .and_then(|params| params.first("name"))
    //     .unwrap_or("world");

    let message = get_http_page();

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}