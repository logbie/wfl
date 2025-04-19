use crate::config::LogLevel;
use chrono::Local;
use log::{LevelFilter, SetLoggerError};
use simplelog::{
    ColorChoice, CombinedLogger, Config, ConfigBuilder, SharedLogger, TermLogger, TerminalMode,
    WriteLogger,
};
use std::fs::File;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

static LOGGER_INITIALIZED: AtomicBool = AtomicBool::new(false);
static START_TIME: once_cell::sync::Lazy<Instant> = once_cell::sync::Lazy::new(Instant::now);

pub fn init_logger(log_level: LogLevel, file_path: &Path) -> Result<(), SetLoggerError> {
    if LOGGER_INITIALIZED.load(Ordering::Relaxed) {
        return Ok(());
    }

    let level_filter = log_level.to_level_filter();

    let config = ConfigBuilder::new()
        .set_time_format_str("%H:%M:%S%.3f")
        .set_location_level(LevelFilter::Debug) // Include file:line for all levels
        .build();

    let file_logger = WriteLogger::new(
        LevelFilter::Debug, // Log everything to file
        config.clone(),
        File::create(file_path).unwrap(),
    );

    let term_logger = TermLogger::new(
        level_filter.min(LevelFilter::Info), // Terminal gets info and above
        config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    CombinedLogger::init(vec![file_logger, term_logger])?;
    LOGGER_INITIALIZED.store(true, Ordering::Relaxed);

    log::info!("WFL logging initialized at {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    Ok(())
}

pub fn elapsed_time() -> std::time::Duration {
    START_TIME.elapsed()
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        log::debug!("[{:?}] {}", $crate::logging::elapsed_time(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log::info!("[{:?}] {}", $crate::logging::elapsed_time(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log::warn!("[{:?}] {}", $crate::logging::elapsed_time(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log::error!("[{:?}] {}", $crate::logging::elapsed_time(), format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_logger_initialization() {
        let temp_dir = tempdir().unwrap();
        let log_path = temp_dir.path().join("test.log");
        
        let result = init_logger(LogLevel::Debug, &log_path);
        assert!(result.is_ok());
        
        log::info!("Test log message");
        
        assert!(log_path.exists());
        let log_content = fs::read_to_string(log_path).unwrap();
        assert!(log_content.contains("Test log message"));
    }
}
