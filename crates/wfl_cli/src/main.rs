use std::env;
use wfl_cli::run_cli;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Err(e) = run_cli(args).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
