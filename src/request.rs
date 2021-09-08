use crate::cli::Cli;
use crate::metrics::ResponseType;
use reqwest::{blocking::Client, blocking::RequestBuilder, Url};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

pub fn create_request(
    args: &Cli,
    metrics: &Mutex<Vec<ResponseType>>,
    json: &Option<HashMap<String, serde_json::Value>>,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.number {
        Some(n) => {
            for _ in 0..n {
                send_request(args, metrics, json)?;
            }
        }
        None => {
            let request_timer = Instant::now();
            let arg_time = args.time.unwrap();
            while request_timer.elapsed().as_secs_f64() < arg_time {
                send_request(args, metrics, json)?;
            }
        }
    }
    Ok(())
}

fn send_request(
    args: &Cli,
    metrics: &Mutex<Vec<ResponseType>>,
    json: &Option<HashMap<String, serde_json::Value>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for path in &args.paths {
        let url = Url::parse(format!("{}://{}", args.scheme, args.host).as_str())?.join(&path)?;
        let mut req = request_builder(&args.method, url);
        if let Some(value) = json {
            req = req.header("Content-Type", "application/json").json(&value);
        }
        let timer = Instant::now();
        let res = req.send();
        let response_time = timer.elapsed().as_millis();
        let mut metrics_lock = metrics.lock().unwrap();

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    metrics_lock.push(ResponseType::Success(response_time));
                } else if response.status().is_redirection() {
                    metrics_lock.push(ResponseType::Redirect(response_time));
                } else if response.status().is_client_error() {
                    metrics_lock.push(ResponseType::ClientError(response_time));
                } else if response.status().is_server_error() {
                    metrics_lock.push(ResponseType::ServerError(response_time));
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    metrics_lock.push(ResponseType::Timeout);
                } else {
                    eprintln!("Error: {}", e.to_string());
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

fn request_builder(method: &String, url: Url) -> RequestBuilder {
    let client = Client::new();
    match method.as_str() {
        "get" => client.get(url),
        "post" => client.post(url),
        "put" => client.put(url),
        "delete" => client.delete(url),
        _ => panic!("Unexpected condition"),
    }
}
