mod gemini_op;

use gemini_op::get_gemini_response;
use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderState {
    pub thought: String,
    pub move1: Option<String>,
    pub move2: Option<String>,
    pub move3: Option<String>,
    pub move4: Option<String>,
    #[serde(rename = "orderType")]
    pub order_type: Option<String>,
    pub response: Option<String>,
    #[serde(rename = "currentOrder")]
    pub current_order: Option<Vec<Order>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    drink: String,
    modifiers: Vec<Modifier>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modifier {
    #[serde(rename = "mod")]
    modifier: String,
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let prompt = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("prompt"))
        .unwrap_or("None");

    if prompt == "None" {
        let resp = Response::builder()
            .status(400)
            .header("content-type", "text/html")
            .body("Please provide a prompt in string parameters".into())
            .map_err(Box::new)?;
        return Ok(resp)
    }

    match get_gemini_response(&prompt).await {
        Ok(content) => {
            let message = content.gemini_response.candidates[0].content.parts[0].text.clone();
            let order_state: OrderState = serde_json::from_str(&message).unwrap_or(OrderState {
                thought: String::from(""),
                move1: None,
                move2: None,
                move3: None,
                move4: None,
                order_type: None,
                response: None,
                current_order: None,
            });

            if order_state.move1.is_some() {
                let resp = Response::builder()
                    .status(200)
                    .header("content-type", "text/html")
                    .body(order_state.response.unwrap().into())
                    .map_err(Box::new)?;
                return Ok(resp)
            } else {
                let message = "Error getting response from Gemini".to_string();
                let resp = Response::builder()
                    .status(500)
                    .header("content-type", "text/html")
                    .body(message.into())
                    .map_err(Box::new)?;
                return Ok(resp)
            }
        }
        Err(e) => {
            let message = format!("Error getting response from Gemini - {}", e);
            let resp = Response::builder()
                .status(500)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
    }
}
