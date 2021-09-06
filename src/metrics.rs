use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use textplots::{Chart, Shape, ColorPlot};
use rgb::RGB8;

const CYAN: RGB8 = RGB8 { r: 49, g: 193, b: 131 };
const YELLOW_GREEN: RGB8 = RGB8 { r: 176, g: 220, b: 100 };
const YELLOW: RGB8 = RGB8 { r: 250, g: 207, b: 83 };
const RED: RGB8 = RGB8 { r: 249, g: 80, b: 83 };


pub enum ResponseType {
    Success(u128),
    Redirect(u128),
    ClientError(u128),
    ServerError(u128),
    Timeout,
}

pub fn show_metrics(mutex: std::sync::Arc<std::sync::Mutex<std::vec::Vec<ResponseType>>>) {
    let mutex_lock = mutex.lock().unwrap();
    let mut success_response_time: Vec<u128> = Vec::new();
    let mut redirect_response_time: Vec<u128> = Vec::new();
    let mut server_error_response_time: Vec<u128> = Vec::new();
    let mut client_error_response_time: Vec<u128> = Vec::new();
    let mut timeout_error_count: u32 = 0;
    let mut response_time: Vec<u128> = Vec::new();
    for i in mutex_lock.iter() {
        match i {
            ResponseType::Success(value) => {
                response_time.push(*value);
                success_response_time.push(*value);
            }
            ResponseType::Redirect(value) => {
                response_time.push(*value);
                redirect_response_time.push(*value);
            }
            ResponseType::ServerError(value) => {
                response_time.push(*value);
                server_error_response_time.push(*value);
            }
            ResponseType::ClientError(value) => {
                response_time.push(*value);
                client_error_response_time.push(*value);
            }
            ResponseType::Timeout => {
                timeout_error_count += 1;
            }
        }
    }

    let total_response_count = mutex_lock.len();


    let table = vec![
        vec![
        "Response Count".cell(),
        success_response_time.len().cell(),
        redirect_response_time.len().cell(),
        client_error_response_time.len().cell(),
        server_error_response_time.len().cell(),
        timeout_error_count.cell(),
        total_response_count.cell(),
        ],
        vec![
            "Average Response Time (ms)".cell(),
            avg_calc(&success_response_time).cell(),
            avg_calc(&redirect_response_time).cell(),
            avg_calc(&client_error_response_time).cell(),
            avg_calc(&server_error_response_time).cell(),
            "-".cell(),
            avg_calc(&response_time).cell()
        ]
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

    if success_response_time.len() > 0 {
        println!("\n2xx Response:");
        Chart::new(150, 80, 0.0, success_response_time.len() as f32)
            .linecolorplot(&Shape::Lines(&get_line(&success_response_time)), CYAN)
            .nice();
    }
    
    if redirect_response_time.len() > 0 {
        println!("\n3xx Response:");
        Chart::new(150, 80, 0.0, redirect_response_time.len() as f32)
            .linecolorplot(&Shape::Lines(&get_line(&redirect_response_time)), YELLOW_GREEN)
            .nice();
    }

    if client_error_response_time.len() > 0 {
        println!("\n4xx Response:");
        Chart::new(150, 80, 0.0, client_error_response_time.len() as f32)
            .linecolorplot(&Shape::Lines(&get_line(&client_error_response_time)), RED)
            .nice();
    }

    if server_error_response_time.len() > 0 {
        println!("\n5xx Response:");
        Chart::new(150, 80, 0.0, server_error_response_time.len() as f32)
            .linecolorplot(&Shape::Lines(&get_line(&server_error_response_time)), YELLOW)
            .nice();
    }
}

fn avg_calc(response_time: &Vec<u128>) -> u128 {
    match response_time.len() {
        0 => 0,
        _ => response_time.iter().sum::<u128>() / response_time.len() as u128
    }
}

fn get_line(vector: &Vec<u128>) -> Vec<(f32, f32)> {
    let mut points = Vec::new();
    for (i, &v) in vector.iter().enumerate() {
        points.push((i as f32, v as f32));
    }
    points
}