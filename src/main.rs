mod cli;
mod metrics;
mod request;
use metrics::{show_metrics, ResponseType};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arc::new(cli::Cli::get_arguments());
    let metrics = Arc::new(Mutex::new(Vec::<ResponseType>::new()));
    let json_string = Arc::new(check_json_file(&args.body)?);

    let mut thread_handler: Vec<std::thread::JoinHandle<()>> = Vec::new();
    for i in 0..args.clients {
        let args = args.clone();
        let metrics = metrics.clone();
        let json = json_string.clone();
        thread_handler.push(
            thread::Builder::new()
                .name(String::from(format!("Client {}", i)))
                .spawn(move || {
                    request::create_request(&args, &metrics, &json).unwrap();
                })
                .unwrap(),
        );
    }
    for i in thread_handler {
        i.join().unwrap();
    }

    show_metrics(metrics);
    Ok(())
}

fn check_json_file(
    body: &Option<PathBuf>,
) -> Result<Option<HashMap<String, serde_json::Value>>, Box<dyn std::error::Error>> {
    match body {
        Some(value) => {
            let json_string = read_to_string(value)?;
            let map: HashMap<String, serde_json::Value> = serde_json::from_str(&json_string)?;
            Ok(Some(map))
        }
        None => Ok(None),
    }
}
