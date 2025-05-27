use std::env;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    wfl_cli::run_cli(args).await
}
