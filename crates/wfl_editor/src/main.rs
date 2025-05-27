use std::env;

#[cfg(not(headless))]
use wfl_editor::launch;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simplelog::SimpleLogger::init(log::LevelFilter::Info, simplelog::Config::default()).ok();

    let _telemetry_enabled = std::env::var("WFL_EDITOR_TELEMETRY")
        .map(|val| val == "1" || val.to_lowercase() == "true")
        .unwrap_or_else(|_| false);

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "--version" || args[1] == "-v") {
        println!("{}", wfl_editor::full_version());
        return Ok(());
    }

    let file_path = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    #[cfg(not(headless))]
    {
        if let Err(e) = launch(file_path) {
            eprintln!("Error launching editor: {}", e);
            return Err(e.into());
        }
    }

    #[cfg(headless)]
    {
        println!("WFL Editor cannot be launched in headless environment.");
        println!("Version: {}", wfl_editor::full_version());
        if let Some(path) = file_path {
            println!("Would open file: {}", path);
        }
    }

    Ok(())
}
