use lambda_http::{Body, Error, Request, RequestExt, Response};
use aws_sdk_dynamodb::{Client, Error as DynamodbError};
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use serde_json::json;
use std::env;

fn format_player(item: &HashMap<String, AttributeValue>) -> String {
    let player = json!({
        "pk": item.get("pk").and_then(|v| v.as_s().ok()).map_or("null", |v| v),
        "sk": item.get("sk").and_then(|v| v.as_s().ok()).map_or("null", |v| v),
        "name": item.get("name").and_then(|v| v.as_s().ok()).map_or("null", |v| v),
        "position": item.get("position").and_then(|v| v.as_s().ok()).map_or("null", |v| v),
        "birth_date": item.get("birth_date").and_then(|v| v.as_s().ok()).map_or("null", |v| v),
        "height": item.get("height").and_then(|v| v.as_s().ok()).map_or("null", |v| v),
        "weight": item.get("weight").and_then(|v| v.as_s().ok()).map_or("null", |v| v),
        "college": item.get("college").and_then(|v| v.as_s().ok()).map_or("null", |v| v),
    });

    player.to_string()
}

pub async fn list_items(client: &Client, table: &str) -> Result<Vec<String>, DynamodbError> {
    let page_size = 20;
    let resp = client
        .scan()
        .limit(page_size)
        .table_name(table)
        .send()
        .await?;

    let mut all_items: Vec<String> = vec![];

    if let Some(items) = resp.items {
        for item in items {
            let item_result = format_player(&item);
            all_items.push(item_result);
        }
    }
    
    println!("{:?}", all_items);
    Ok(all_items)
}

pub async fn get_item(client: &Client, table: &str, id: &str) -> Result<String, DynamodbError> {
    let resp = client
        .get_item()
        .table_name(table)
        .key(
            "pk", 
            AttributeValue::S(id.to_string()),
        )
        .key(
            "sk",
            AttributeValue::S(id.to_string()),
        )
        .send()
        .await?;

    let item = match resp.item {
        Some(item) => item,
        None => {
            println!("Item not found");
            return Ok("".to_string());
        }
    };

    let item_result = format_player(&item);

    println!("{:?}", item_result);
    
    Ok(item_result)
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    println!("{:?}", event);
    // Extract some useful information from the request
    let item_id = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("item_id"))
        .unwrap_or("none");

    let table_name = match env::var("TABLE_NAME") {
        Ok(value) => value,
        Err(_) => {
            let message = "Environment variable TABLE_NAME must be set.";
            let resp = Response::builder()
                .status(404)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    println!("Uri path: {}", event.uri().path());

    let mut response_player = String::from("Not found data.");

    if event.uri().path() == "/player/all_players" {
        let all_items = match list_items(&client, &table_name).await {
            Ok(items) => {
                let json_string = match serde_json::to_string(&items) {
                    Ok(json) => json,
                    Err(_) => "Error in deserialize".to_string(),
                };
                response_player = json_string
            }
            Err(e) => {
                let message = format!("Error getting items from DynamoDB. {}", e);
                let resp = Response::builder()
                    .status(400)
                    .header("content-type", "text/html")
                    .body(message.into())
                    .map_err(Box::new)?;
                return Ok(resp);
            }
        };
    } else if event.uri().path() == "/player/detail" {
        let item = match get_item(&client, &table_name, item_id).await {
            Ok(item) => response_player = item,
            Err(e) => {
                let message = format!("Error getting item from DynamoDB. {}", e);
                let resp = Response::builder()
                    .status(400)
                    .header("content-type", "text/html")
                    .body(message.into())
                    .map_err(Box::new)?;
                return Ok(resp);
            }
        };
    } else {
        let message = "Not found path.";
        let resp = Response::builder()
            .status(404)
            .header("content-type", "text/html")
            .body(message.into())
            .map_err(Box::new)?;
        return Ok(resp);
    };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(response_player.into())
        .map_err(Box::new)?;
    Ok(resp)
}