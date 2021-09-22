use crate::utils::cli::Cli;
use reqwest::{
    blocking::Client,
    blocking::RequestBuilder,
    header::{HeaderMap, HeaderName, HeaderValue},
    Url,
};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::Instant;
use crate::models::{response::Response, response_type::ResponseType};

pub fn create_request(
    args: &Cli,
    metrics: &Mutex<Vec<Response>>,
    json: &Option<HashMap<String, serde_json::Value>>,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.number {
        Some(mut n) => {
            n = (n as f64 / args.paths.len() as f64).round() as u64;
            for path in &args.paths {
                for _ in 0..n {
                    send_request(args, metrics, json, path)?;
                }
            }
        }
        None => {
            let request_timer = Instant::now();
            let arg_time = args.time.unwrap() / args.paths.len() as f64;
            for path in &args.paths {
                while request_timer.elapsed().as_secs_f64() < arg_time {
                    send_request(args, metrics, json, path)?;
                }
            }
        }
    }
    Ok(())
}

fn send_request(
    args: &Cli,
    metrics: &Mutex<Vec<Response>>,
    json: &Option<HashMap<String, serde_json::Value>>,
    path: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(format!("{}://{}", args.scheme, args.host).as_str())?.join(&path)?;
    let mut req = request_builder(&args.method, url);
    if let Some(value) = json {
        req = req.header("Content-Type", "application/json").json(&value);
    }
    if let Some(value) = &args.headers {
        let mut headers = HeaderMap::new();
        for i in value {
            let split = i
                .split("=")
                .map(|x| String::from(x))
                .collect::<Vec<String>>();
            headers.insert(
                HeaderName::from_str(&split[0]).unwrap(),
                HeaderValue::from_str(&split[1]).unwrap(),
            );
        }
        req = req.headers(headers);
    }
    let timer = Instant::now();
    let res = req.send();
    let response_time = timer.elapsed().as_millis();
    let mut metrics_lock = metrics.lock().unwrap();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                metrics_lock.push(Response {
                    res_type: ResponseType::Success,
                    res_time: response_time,
                    path: path.clone(),
                });
            } else if response.status().is_redirection() {
                metrics_lock.push(Response {
                    res_type: ResponseType::Redirect,
                    res_time: response_time,
                    path: path.clone(),
                });
            } else if response.status().is_client_error() {
                metrics_lock.push(Response {
                    res_type: ResponseType::ClientError,
                    res_time: response_time,
                    path: path.clone(),
                });
            } else if response.status().is_server_error() {
                metrics_lock.push(Response {
                    res_type: ResponseType::ServerError,
                    res_time: response_time,
                    path: path.clone(),
                });
            }
        }
        Err(e) => {
            if e.is_timeout() {
                metrics_lock.push(Response {
                    res_type: ResponseType::Timeout,
                    res_time: 0,
                    path: path.clone(),
                });
            } else {
                eprintln!("Error: {}", e.to_string());
                return Ok(());
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
