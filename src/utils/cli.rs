use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// Host URL of WebApp
    #[structopt(short, long)]
    pub host: String,
    
    /// Scheme
    #[structopt(short, long, possible_values = &["http", "https"])]
    pub scheme: String,
    /// List of URL paths (space separated)
    #[structopt(short, long, default_value = "/")]
    pub paths: Vec<String>,
    
    /// Request method
    #[structopt(short, long, default_value = "get", possible_values = &["get", "post", "put", "delete"])]
    pub method: String,

    /// Total no of clients
    #[structopt(short, long)]
    pub clients: i64,

    /// Total time (seconds) (Not required if -n, --number is used)
    #[structopt(short, long, required_unless = "number")]
    pub time: Option<f64>,

    /// Number of requests to be sent by each client (Not required if -t, --time is used)
    #[structopt(short, long, required_unless = "time")]
    pub number: Option<u64>,

    /// Request body (path to *.json file)
    #[structopt(short, long, parse(from_os_str))]
    pub body: Option<PathBuf>,

    /// List of Request header (space separated) (format: "key=value")
    #[structopt(long)]
    pub headers: Option<Vec<String>>,

    // /// Show response for each path separately
    // #[structopt(long)]
    // pub response_per_path: bool
}

impl Cli {
    pub fn get_arguments() -> Self {
        Cli::from_args()
    }
}