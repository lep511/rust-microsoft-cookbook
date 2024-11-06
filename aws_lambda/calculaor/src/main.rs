// This example requires the following input to succeed:
// { "op": "do something", "i": 2, "j": 5 }

use lambda_runtime::{service_fn, tracing, Error, LambdaEvent};
use serde::{ Serialize, Deserialize };
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;
use anyhow::anyhow;

#[derive(Debug, Deserialize)]
struct InvokeArgs {
    op: Operation,
    i: f64,
    j: f64,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
enum Operation {
    #[serde(rename = "plus")]
    Plus,
    #[serde(rename = "minus")]
    Minus,
    #[serde(rename = "times")]
    Times,
    #[serde(rename = "divided-by")]
    DividedBy,
}

impl std::str::FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plus" => Ok(Operation::Plus),
            "minus" => Ok(Operation::Minus),
            "times" => Ok(Operation::Times),
            "divided-by" => Ok(Operation::DividedBy),
            _ => Err(anyhow!("Unknown operation {s}")),
        }
    }
}

fn calculate(args: InvokeArgs) -> Result<f64, anyhow::Error> {
    let result = match args.op {
        Operation::Plus => add(args.i, args.j),
        Operation::Minus => subtract(args.i, args.j),
        Operation::Times => multiply(args.i, args.j),
        Operation::DividedBy => divide(args.i, args.j),
    }?;

    debug!(?args, ?result, "Full event data",);
    info!("The result of the calculation: {}", result);

    Ok(result)
}

fn add(num1: f64, num2: f64) -> Result<f64, anyhow::Error> {
    Ok(num1 + num2)
}

fn subtract(num1: f64, num2: f64) -> Result<f64, anyhow::Error> {
    Ok(num1 - num2)
}

fn multiply(num1: f64, num2: f64) -> Result<f64, anyhow::Error> {
    Ok(num1 * num2)
}

fn divide(num1: f64, num2: f64) -> Result<f64, anyhow::Error> {
    if num2 == 0.0 {
        warn!("Attempted to divide by zero");
        return Err(anyhow::anyhow!("Cannot divide by zero"));
    }

    Ok(num1 / num2)
}

pub(crate) async fn my_handler(event: LambdaEvent<InvokeArgs>) -> Result<Response, Error> {
    // extract some useful info from the event
    let invoke_args:InvokeArgs = event.payload;
    //info!(?event, "Event data");

    let result = calculate(invoke_args)?;

    // prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("The result is {}", result),
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
