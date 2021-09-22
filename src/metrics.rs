use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use std::ops::Deref;
use textplots::{Chart, ColorPlot, Shape};
use crate::utils::colors::{*};
use crate::models::{response_type::ResponseType, response::Response};


pub fn show_metrics(mutex: std::sync::Arc<std::sync::Mutex<std::vec::Vec<Response>>>) {
    let mutex_lock = mutex.lock().unwrap();
    let response = mutex_lock.deref();
    let mut success_response: Vec<Response> = Vec::new();
    let mut redirect_response: Vec<Response> = Vec::new();
    let mut server_error_response: Vec<Response> = Vec::new();
    let mut client_error_response: Vec<Response> = Vec::new();
    let normal_response: Vec<Response> = response
        .iter()
        .filter(|x| x.res_type != ResponseType::Timeout)
        .map(|x| x.clone())
        .collect();
    let mut timeout_error_count: u32 = 0;
    for i in response.iter() {
        match i.res_type {
            ResponseType::Success => success_response.push(i.clone()),
            ResponseType::Redirect => redirect_response.push(i.clone()),
            ResponseType::ServerError => server_error_response.push(i.clone()),
            ResponseType::ClientError => client_error_response.push(i.clone()),
            ResponseType::Timeout => timeout_error_count += 1,
        }
    }

    let total_response_count = response.len();

    let table = vec![
        vec![
            "Response Count".cell(),
            success_response.len().cell(),
            redirect_response.len().cell(),
            client_error_response.len().cell(),
            server_error_response.len().cell(),
            timeout_error_count.cell(),
            total_response_count.cell(),
        ],
        vec![
            "Average Response Time (ms)".cell(),
            avg_calc(&success_response).cell(),
            avg_calc(&redirect_response).cell(),
            avg_calc(&client_error_response).cell(),
            avg_calc(&server_error_response).cell(),
            "-".cell(),
            avg_calc(&normal_response).cell(),
        ],
        vec![
            "Maximum Response Time (ms)".cell(),
            max_calc(&success_response).cell(),
            max_calc(&redirect_response).cell(),
            max_calc(&client_error_response).cell(),
            max_calc(&server_error_response).cell(),
            "-".cell(),
            max_calc(&normal_response).cell(),
        ],
        vec![
            "Minimum Response Time (ms)".cell(),
            min_calc(&success_response).cell(),
            min_calc(&redirect_response).cell(),
            min_calc(&client_error_response).cell(),
            min_calc(&server_error_response).cell(),
            "-".cell(),
            min_calc(&response).cell(),
        ],
    ]
    .table()
    .title(vec![
        "Parameters".cell().bold(true).justify(Justify::Center),
        "2xx".cell().bold(true).justify(Justify::Center),
        "3xx".cell().bold(true).justify(Justify::Center),
        "4xx".cell().bold(true).justify(Justify::Center),
        "5xx".cell().bold(true).justify(Justify::Center),
        "Timeout".cell().bold(true).justify(Justify::Center),
        "Total".cell().bold(true).justify(Justify::Center),
    ])
    .bold(true);

    assert!(print_stdout(table).is_ok());

    if success_response.len() > 0 {
        println!("\n2xx Response:");
        Chart::new(150, 80, 0.0, success_response.len() as f32)
            .linecolorplot(&Shape::Lines(&get_line(&success_response)), CYAN)
            .nice();
    }
    if redirect_response.len() > 0 {
        println!("\n3xx Response:");
        Chart::new(150, 80, 0.0, redirect_response.len() as f32)
            .linecolorplot(&Shape::Lines(&get_line(&redirect_response)), YELLOW_GREEN)
            .nice();
    }

    if client_error_response.len() > 0 {
        println!("\n4xx Response:");
        Chart::new(150, 80, 0.0, client_error_response.len() as f32)
            .linecolorplot(&Shape::Lines(&get_line(&client_error_response)), RED)
            .nice();
    }

    if server_error_response.len() > 0 {
        println!("\n5xx Response:");
        Chart::new(150, 80, 0.0, server_error_response.len() as f32)
            .linecolorplot(&Shape::Lines(&get_line(&server_error_response)), YELLOW)
            .nice();
    }
}

fn avg_calc(response: &Vec<Response>) -> u128 {
    match response.len() {
        0 => 0,
        _ => response.iter().map(|x| x.res_time).sum::<u128>() / response.len() as u128,
    }
}

fn min_calc(response: &Vec<Response>) -> u128 {
    match response.len() {
        0 => 0,
        _ => response.iter().map(|x| x.res_time).min().unwrap(),
    }
}

fn max_calc(response: &Vec<Response>) -> u128 {
    match response.len() {
        0 => 0,
        _ => response.iter().map(|x| x.res_time).max().unwrap(),
    }
}

fn get_line(vector: &Vec<Response>) -> Vec<(f32, f32)> {
    let mut points = Vec::new();
    for (i, v) in vector.iter().map(|x| x.res_time).enumerate() {
        points.push((i as f32, v as f32));
    }
    points
}
