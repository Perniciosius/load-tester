mod cli;
mod metrics;
mod request;
use metrics::{ResponseType, show_metrics};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arc::new(cli::Cli::get_arguments());
    let metrics = Arc::new(Mutex::new(Vec::<ResponseType>::new()));

    let mut thread_handler: Vec<std::thread::JoinHandle<()>> = Vec::new();
    for i in 0..args.clients {
        let args = args.clone();
        let metrics = metrics.clone();
        thread_handler.push(
            thread::Builder::new()
                .name(String::from(format!("Client {}", i)))
                .spawn(move || {
                    request::create_request(&args, &metrics).unwrap();
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
