use cli_table::{format::Justify, Table};

#[derive(Debug, Table)]
pub struct Metrics {
    #[table(title = "5xx", justify = "Justify::Center")]
    pub count_5xx: u32,
    #[table(title = "4xx", justify = "Justify::Center")]
    pub count_4xx: u32,
    #[table(title = "3xx", justify = "Justify::Center")]
    pub count_3xx: u32,
    #[table(title = "2xx", justify = "Justify::Center")]
    pub count_2xx: u32,
    #[table(title = "Timeout", justify = "Justify::Center")]
    pub count_timeout: u32,
    #[table(skip)]
    pub response_time: Vec<u128>,
    #[table(title = "Average Response Time (ms)", justify = "Justify::Center")]
    pub average_response_time: f64,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            count_5xx: 0,
            count_4xx: 0,
            count_3xx: 0,
            count_2xx: 0,
            count_timeout: 0,
            response_time: Vec::new(),
            average_response_time: 0.0,
        }
    }
}