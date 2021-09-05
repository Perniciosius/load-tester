use crate::cli::Cli;
use crate::metrics::Metrics;
use reqwest::blocking::Client;
use reqwest::blocking::RequestBuilder;
use reqwest::Url;
use std::sync::Mutex;
use std::time::Instant;

pub fn create_request(
    args: &Cli,
    metrics: &Mutex<Metrics>,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.number {
        Some(n) => {
            for _ in 0..n {
                send_request(args, metrics)?;
            }
        }
        None => {
            let request_timer = Instant::now();
            let arg_time = args.time.unwrap();
            while request_timer.elapsed().as_secs_f64() < arg_time {
                send_request(args, metrics)?;
            }
        }
    }
    Ok(())
}

fn send_request(args: &Cli, metrics: &Mutex<Metrics>) -> Result<(), Box<dyn std::error::Error>> {
    for path in &args.paths {
        let url = Url::parse(format!("{}://{}", args.scheme, args.host).as_str())?.join(&path)?;
        let req = request_builder(&args.method, url);
        let timer = Instant::now();
        let res = req.send();
        let response_time = timer.elapsed().as_millis();
        let mut metrics_lock = metrics.lock().unwrap();

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    metrics_lock.count_2xx += 1;
                } else if response.status().is_redirection() {
                    metrics_lock.count_3xx += 1;
                } else if response.status().is_client_error() {
                    metrics_lock.count_4xx += 1;
                } else if response.status().is_server_error() {
                    metrics_lock.count_5xx += 1;
                }
                metrics_lock.response_time.push(response_time);
            }
            Err(e) => {
                if e.is_timeout() {
                    metrics_lock.count_timeout += 1;
                } else {
                    eprintln!("Error: {}", e.to_string());
                    // return Ok(());
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