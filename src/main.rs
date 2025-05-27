use std::io;
use std::env;

fn main() -> io::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(async {
        wfl_cli::run_cli(env::args().collect()).await
    })
}
