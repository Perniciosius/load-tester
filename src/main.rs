mod cli;
mod metrics;
mod request;

use cli_table::{print_stdout, WithTitle};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use textplots::{Chart, Plot, Shape};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arc::new(cli::Cli::get_arguments());
    let metrics = Arc::new(Mutex::new(metrics::Metrics::new()));

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
    let mut metrics_lock = metrics.lock().unwrap();
    metrics_lock.average_response_time = metrics_lock.response_time.iter().sum::<u128>() as f64
        / metrics_lock.response_time.len() as f64;
    assert!(print_stdout([metrics_lock.deref()].with_title()).is_ok());
    let res_time = &metrics_lock.response_time;
    let mut points = Vec::<(f32, f32)>::new();
    for (i, &v) in res_time.iter().enumerate() {
        points.push((i as f32, v as f32));
    }
    println!("\n Response Time:");
    Chart::new(80, 40, 0.0, res_time.len() as f32)
        .lineplot(&Shape::Lines(&points))
        .lineplot(&Shape::Continuous(Box::new(|_| metrics_lock.average_response_time as f32)))
        .nice();
    Ok(())
}
