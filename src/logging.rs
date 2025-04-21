use crate::config::LogLevel;
use chrono::Local;
use log::{LevelFilter, SetLoggerError};
use once_cell::sync::Lazy;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use time;
use time::format_description::FormatItem;

static LOGGER_INITIALIZED: AtomicBool = AtomicBool::new(false);
static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);
static TIME_FORMAT: Lazy<Vec<FormatItem>> =
    Lazy::new(|| time::format_description::parse("[hour]:[minute]:[second].[subsecond]").unwrap());

pub fn init_logger(log_level: LogLevel, file_path: &Path) -> Result<(), SetLoggerError> {
    if LOGGER_INITIALIZED.load(Ordering::Relaxed) {
        return Ok(());
    }

    let level_filter = log_level.to_level_filter();

    let config = ConfigBuilder::new()
        .set_time_format_custom(&TIME_FORMAT)
        .set_location_level(LevelFilter::Debug) // Include file:line for all levels
        .build();

    let file_logger_result = File::create(file_path).map(|file| {
        WriteLogger::new(
            LevelFilter::Debug, // Log everything to file
            config.clone(),
            file,
        )
    });

    if let Err(e) = &file_logger_result {
        eprintln!(
            "Warning: Could not create log file at {}: {}",
            file_path.display(),
            e
        );

        let term_logger = TermLogger::new(
            level_filter.min(LevelFilter::Info), // Terminal gets info and above
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        );

        CombinedLogger::init(vec![term_logger])?;
        LOGGER_INITIALIZED.store(true, Ordering::Relaxed);

        log::info!(
            "WFL logging initialized at {} (console-only mode)",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        return Ok(());
    }

    let file_logger = file_logger_result.unwrap();

    let term_logger = TermLogger::new(
        level_filter.min(LevelFilter::Info), // Terminal gets info and above
        config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    CombinedLogger::init(vec![file_logger, term_logger])?;
    LOGGER_INITIALIZED.store(true, Ordering::Relaxed);

    log::info!(
        "WFL logging initialized at {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
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
    use tempfile::tempdir;

    #[test]
    #[ignore = "Cannot reset logger state between tests"]
    fn test_logger_initialization() {
        let temp_dir = tempdir().unwrap();
        let log_path = temp_dir.path().join("test.log");

        let result = init_logger(LogLevel::Debug, &log_path);
        assert!(result.is_ok());

        log::info!("Test log message");
    }

    #[test]
    fn test_logger_fallback_when_file_creation_fails() {
        let temp_dir = tempdir().unwrap();
        let _dir_path = temp_dir.path();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(dir_path, std::fs::Permissions::from_mode(0o555)).unwrap();
        }

        #[cfg(not(unix))]
        let dir_path = temp_dir.path().join("non_existent_directory");

        let log_path = dir_path.join("test.log");

        let result = init_logger(LogLevel::Debug, &log_path);
        assert!(result.is_ok());

        log::info!("This should not panic");
    }
}
