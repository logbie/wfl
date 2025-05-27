use std::env;
use wfl_editor::launch;

fn main() -> Result<(), eframe::Error> {
    simplelog::SimpleLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
    ).ok();
    
    let _telemetry_enabled = std::env::var("WFL_EDITOR_TELEMETRY")
        .map(|val| val == "1" || val.to_lowercase() == "true")
        .unwrap_or_else(|_| false);
    
    
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };
    
    launch(file_path)
}
