use std::path::Path;
use std::str::FromStr;

#[cfg(windows)]
const DEFAULT_GLOBAL_CONFIG_PATH: &str = "C:\\wfl\\config";

#[cfg(not(windows))]
const DEFAULT_GLOBAL_CONFIG_PATH: &str = "/etc/wfl/wfl.cfg";

fn get_global_config_path() -> &'static str {
    std::env::var("WFL_GLOBAL_CONFIG_PATH")
        .ok()
        .map(|path| Box::leak(path.into_boxed_str()))
        .map_or(DEFAULT_GLOBAL_CONFIG_PATH, |v| v)
}

#[derive(Debug, Clone)]
pub struct WflConfig {
    pub timeout_seconds: u64,
    pub logging_enabled: bool,
    pub debug_report_enabled: bool,
    pub log_level: LogLevel,
    // Code quality suite settings
    pub max_line_length: usize,
    pub max_nesting_depth: usize,
    pub indent_size: usize,
    pub snake_case_variables: bool,
    pub trailing_whitespace: bool,
    pub consistent_keyword_case: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for WflConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 60,
            logging_enabled: false,
            debug_report_enabled: true,
            log_level: LogLevel::Info,
            // Code quality suite defaults - strict by default
            max_line_length: 100,
            max_nesting_depth: 5,
            indent_size: 4,
            snake_case_variables: true,
            trailing_whitespace: false, // false means no trailing whitespace allowed
            consistent_keyword_case: true,
        }
    }
}

// For the FromStr trait implementation
#[derive(Debug)]
pub struct ParseLogLevelError;

impl std::fmt::Display for ParseLogLevelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse log level")
    }
}

impl std::error::Error for ParseLogLevelError {}

impl FromStr for LogLevel {
    type Err = ParseLogLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_lowercase().as_str() {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" | "warning" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info, // Default to Info for unrecognized values
        })
    }
}

impl LogLevel {
    // Keep this for backward compatibility
    pub fn parse_str(s: &str) -> Self {
        s.parse().unwrap_or(LogLevel::Info)
    }

    pub fn to_level_filter(&self) -> log::LevelFilter {
        match self {
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Error => log::LevelFilter::Error,
        }
    }
}

fn parse_config_text(config: &mut WflConfig, text: &str, file: &Path) {
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, rest)) = line.split_once('=') {
            let key = key.trim();
            let value = rest.trim();

            match key {
                "timeout_seconds" => {
                    if let Ok(timeout) = value.parse::<u64>() {
                        if config.timeout_seconds != WflConfig::default().timeout_seconds {
                            log::debug!(
                                "Overriding timeout_seconds: {} -> {} from {}",
                                config.timeout_seconds,
                                timeout.max(1),
                                file.display()
                            );
                        }
                        config.timeout_seconds = timeout.max(1);
                        log::debug!(
                            "Loaded timeout override: {} s from {}",
                            config.timeout_seconds,
                            file.display()
                        );
                    }
                }
                "logging_enabled" => {
                    if let Ok(enabled) = value.parse::<bool>() {
                        if config.logging_enabled != WflConfig::default().logging_enabled {
                            log::debug!(
                                "Overriding logging_enabled: {} -> {} from {}",
                                config.logging_enabled,
                                enabled,
                                file.display()
                            );
                        }
                        config.logging_enabled = enabled;
                        log::debug!(
                            "Loaded logging_enabled: {} from {}",
                            config.logging_enabled,
                            file.display()
                        );
                    }
                }
                "debug_report_enabled" => {
                    if let Ok(enabled) = value.parse::<bool>() {
                        if config.debug_report_enabled != WflConfig::default().debug_report_enabled
                        {
                            log::debug!(
                                "Overriding debug_report_enabled: {} -> {} from {}",
                                config.debug_report_enabled,
                                enabled,
                                file.display()
                            );
                        }
                        config.debug_report_enabled = enabled;
                        log::debug!(
                            "Loaded debug_report_enabled: {} from {}",
                            config.debug_report_enabled,
                            file.display()
                        );
                    }
                }
                "log_level" => {
                    if config.log_level != WflConfig::default().log_level {
                        log::debug!(
                            "Overriding log_level: {:?} -> {:?} from {}",
                            config.log_level,
                            LogLevel::parse_str(value),
                            file.display()
                        );
                    }
                    config.log_level = LogLevel::parse_str(value);
                    log::debug!(
                        "Loaded log_level: {:?} from {}",
                        config.log_level,
                        file.display()
                    );
                }
                // Code quality suite settings
                "max_line_length" => {
                    if let Ok(length) = value.parse::<usize>() {
                        if config.max_line_length != WflConfig::default().max_line_length {
                            log::debug!(
                                "Overriding max_line_length: {} -> {} from {}",
                                config.max_line_length,
                                length,
                                file.display()
                            );
                        }
                        config.max_line_length = length;
                        log::debug!(
                            "Loaded max_line_length: {} from {}",
                            config.max_line_length,
                            file.display()
                        );
                    }
                }
                "max_nesting_depth" => {
                    if let Ok(depth) = value.parse::<usize>() {
                        if config.max_nesting_depth != WflConfig::default().max_nesting_depth {
                            log::debug!(
                                "Overriding max_nesting_depth: {} -> {} from {}",
                                config.max_nesting_depth,
                                depth,
                                file.display()
                            );
                        }
                        config.max_nesting_depth = depth;
                        log::debug!(
                            "Loaded max_nesting_depth: {} from {}",
                            config.max_nesting_depth,
                            file.display()
                        );
                    }
                }
                "indent_size" => {
                    if let Ok(size) = value.parse::<usize>() {
                        if config.indent_size != WflConfig::default().indent_size {
                            log::debug!(
                                "Overriding indent_size: {} -> {} from {}",
                                config.indent_size,
                                size,
                                file.display()
                            );
                        }
                        config.indent_size = size;
                        log::debug!(
                            "Loaded indent_size: {} from {}",
                            config.indent_size,
                            file.display()
                        );
                    }
                }
                "snake_case_variables" => {
                    if let Ok(enabled) = value.parse::<bool>() {
                        if config.snake_case_variables != WflConfig::default().snake_case_variables
                        {
                            log::debug!(
                                "Overriding snake_case_variables: {} -> {} from {}",
                                config.snake_case_variables,
                                enabled,
                                file.display()
                            );
                        }
                        config.snake_case_variables = enabled;
                        log::debug!(
                            "Loaded snake_case_variables: {} from {}",
                            config.snake_case_variables,
                            file.display()
                        );
                    }
                }
                "trailing_whitespace" => {
                    if let Ok(enabled) = value.parse::<bool>() {
                        if config.trailing_whitespace != WflConfig::default().trailing_whitespace {
                            log::debug!(
                                "Overriding trailing_whitespace: {} -> {} from {}",
                                config.trailing_whitespace,
                                enabled,
                                file.display()
                            );
                        }
                        config.trailing_whitespace = enabled;
                        log::debug!(
                            "Loaded trailing_whitespace: {} from {}",
                            config.trailing_whitespace,
                            file.display()
                        );
                    }
                }
                "consistent_keyword_case" => {
                    if let Ok(enabled) = value.parse::<bool>() {
                        if config.consistent_keyword_case
                            != WflConfig::default().consistent_keyword_case
                        {
                            log::debug!(
                                "Overriding consistent_keyword_case: {} -> {} from {}",
                                config.consistent_keyword_case,
                                enabled,
                                file.display()
                            );
                        }
                        config.consistent_keyword_case = enabled;
                        log::debug!(
                            "Loaded consistent_keyword_case: {} from {}",
                            config.consistent_keyword_case,
                            file.display()
                        );
                    }
                }
                _ => {
                    log::warn!("Unknown configuration key: {} in {}", key, file.display());
                }
            }
        }
    }
}

pub fn load_config(dir: &Path) -> WflConfig {
    // Start with default configuration
    let mut config = WflConfig::default();

    // Try to load global configuration
    let global_config = Path::new(get_global_config_path());
    let mut loaded_global = false;

    if global_config.exists() {
        if let Ok(text) = std::fs::read_to_string(global_config) {
            loaded_global = true;
            log::debug!(
                "Loading global configuration from {}",
                global_config.display()
            );
            parse_config_text(&mut config, &text, global_config);
        }
    }

    if !loaded_global {
        let old_system_config = Path::new("/etc/wfl/.wflcfg");
        if old_system_config.exists() {
            if let Ok(text) = std::fs::read_to_string(old_system_config) {
                log::debug!(
                    "Loading global configuration from {} (legacy path)",
                    old_system_config.display()
                );
                parse_config_text(&mut config, &text, old_system_config);
            }
        }
    }

    let local_config = dir.join(".wflcfg");
    if local_config.exists() {
        if let Ok(text) = std::fs::read_to_string(&local_config) {
            log::debug!(
                "Loading local configuration from {}",
                local_config.display()
            );
            parse_config_text(&mut config, &text, &local_config);
        }
    }

    config
}

pub fn load_config_with_global(script_dir: &Path) -> WflConfig {
    // Start with default configuration
    let mut config = WflConfig::default();

    // Try to load global configuration
    let global_config = Path::new(get_global_config_path());
    let mut loaded_global = false;

    if global_config.exists() {
        if let Ok(text) = std::fs::read_to_string(global_config) {
            loaded_global = true;
            log::debug!(
                "Loading global configuration from {}",
                global_config.display()
            );
            parse_config_text(&mut config, &text, global_config);
        }
    }

    if !loaded_global {
        let old_system_config = Path::new("/etc/wfl/.wflcfg");
        if old_system_config.exists() {
            if let Ok(text) = std::fs::read_to_string(old_system_config) {
                log::debug!(
                    "Loading global configuration from {} (legacy path)",
                    old_system_config.display()
                );
                parse_config_text(&mut config, &text, old_system_config);
            }
        }
    }

    // Load local configuration
    let local_config = script_dir.join(".wflcfg");
    if local_config.exists() {
        if let Ok(text) = std::fs::read_to_string(&local_config) {
            log::debug!(
                "Loading local configuration from {}",
                local_config.display()
            );
            parse_config_text(&mut config, &text, &local_config);
        }
    }

    config
}

pub fn load_timeout(dir: &Path) -> u64 {
    load_config(dir).timeout_seconds
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    fn with_test_global_path<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        f()
    }

    #[test]
    fn test_load_timeout_default() {
        let temp_dir = tempfile::tempdir().unwrap();
        let timeout = load_timeout(temp_dir.path());
        assert_eq!(timeout, 60);
    }

    #[test]
    fn test_load_timeout_custom() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join(".wflcfg");

        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(b"timeout_seconds = 120").unwrap();

        let timeout = load_timeout(temp_dir.path());
        assert_eq!(timeout, 120);
    }

    #[test]
    fn test_load_timeout_with_comments() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join(".wflcfg");

        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(b"# This is a comment\ntimeout_seconds = 45\n# Another comment")
            .unwrap();

        let timeout = load_timeout(temp_dir.path());
        assert_eq!(timeout, 45);
    }

    #[test]
    fn test_load_timeout_invalid() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join(".wflcfg");

        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(b"timeout_seconds = invalid").unwrap();

        unsafe {
            std::env::set_var("WFL_GLOBAL_CONFIG_PATH", "/non/existent/path");
        }

        let timeout = with_test_global_path(|| load_timeout(temp_dir.path()));
        assert_eq!(timeout, 60); // Should fall back to default
    }

    #[test]
    fn test_load_config_defaults() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = load_config(temp_dir.path());

        assert_eq!(config.timeout_seconds, 60);
        assert!(!config.logging_enabled);
        assert!(config.debug_report_enabled);
        assert_eq!(config.log_level, LogLevel::Info);
    }

    #[test]
    fn test_load_config_custom() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join(".wflcfg");

        let config_content = r#"
        # WFL Configuration
        timeout_seconds = 120
        logging_enabled = true
        debug_report_enabled = false
        log_level = debug
        "#;

        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(config_content.as_bytes()).unwrap();

        let config = load_config(temp_dir.path());

        assert_eq!(config.timeout_seconds, 120);
        assert!(config.logging_enabled);
        assert!(!config.debug_report_enabled);
        assert_eq!(config.log_level, LogLevel::Debug);
    }

    #[test]
    fn test_load_config_partial() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join(".wflcfg");

        let config_content = r#"
        # Only specify some settings
        timeout_seconds = 30
        log_level = error
        "#;

        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(config_content.as_bytes()).unwrap();

        unsafe {
            std::env::set_var("WFL_GLOBAL_CONFIG_PATH", "/non/existent/path");
        }

        let config = with_test_global_path(|| load_config(temp_dir.path()));

        assert_eq!(config.timeout_seconds, 30);
        assert!(!config.logging_enabled); // Default
        assert!(config.debug_report_enabled); // Default
        assert_eq!(config.log_level, LogLevel::Error);
    }

    #[test]
    fn test_log_level_parsing() {
        assert_eq!("debug".parse::<LogLevel>().unwrap(), LogLevel::Debug);
        assert_eq!("INFO".parse::<LogLevel>().unwrap(), LogLevel::Info);
        assert_eq!("Warning".parse::<LogLevel>().unwrap(), LogLevel::Warn);
        assert_eq!("warn".parse::<LogLevel>().unwrap(), LogLevel::Warn);
        assert_eq!("ERROR".parse::<LogLevel>().unwrap(), LogLevel::Error);
        assert_eq!("unknown".parse::<LogLevel>().unwrap(), LogLevel::Info); // Default
    }

    #[test]
    fn test_load_config_global_only() {
        let temp_dir = tempdir().unwrap();
        let global_config_path = temp_dir.path().join("wfl.cfg");

        let global_config_content = r#"
        # Global WFL Configuration
        timeout_seconds = 180
        logging_enabled = true
        max_line_length = 120
        "#;

        let mut file = fs::File::create(&global_config_path).unwrap();
        file.write_all(global_config_content.as_bytes()).unwrap();

        unsafe {
            std::env::set_var(
                "WFL_GLOBAL_CONFIG_PATH",
                global_config_path.to_str().unwrap(),
            );
        }

        let script_dir = tempdir().unwrap();

        let config = with_test_global_path(|| load_config_with_global(script_dir.path()));

        assert_eq!(config.timeout_seconds, 60);
        assert!(config.logging_enabled);
        assert_eq!(config.max_line_length, 120);
        assert!(config.debug_report_enabled); // Default
    }

    #[test]
    fn test_load_config_local_only() {
        let script_dir = tempdir().unwrap();
        let local_config_path = script_dir.path().join(".wflcfg");

        let local_config_content = r#"
        # Local WFL Configuration
        timeout_seconds = 90
        log_level = debug
        snake_case_variables = false
        "#;

        let mut file = fs::File::create(&local_config_path).unwrap();
        file.write_all(local_config_content.as_bytes()).unwrap();

        unsafe {
            std::env::set_var("WFL_GLOBAL_CONFIG_PATH", "/non/existent/path");
        }

        let config = with_test_global_path(|| load_config(script_dir.path()));

        assert_eq!(config.timeout_seconds, 90);
        assert!(!config.logging_enabled); // Default
        assert_eq!(config.log_level, LogLevel::Debug);
        assert!(!config.snake_case_variables);
    }

    #[test]
    fn test_load_config_local_override() {
        let temp_dir = tempdir().unwrap();
        let global_config_path = temp_dir.path().join("wfl.cfg");

        let global_config_content = r#"
        # Global WFL Configuration
        timeout_seconds = 180
        logging_enabled = true
        max_line_length = 120
        snake_case_variables = true
        "#;

        let mut file = fs::File::create(&global_config_path).unwrap();
        file.write_all(global_config_content.as_bytes()).unwrap();

        unsafe {
            std::env::set_var(
                "WFL_GLOBAL_CONFIG_PATH",
                global_config_path.to_str().unwrap(),
            );
        }

        let script_dir = tempdir().unwrap();
        let local_config_path = script_dir.path().join(".wflcfg");

        let local_config_content = r#"
        # Local WFL Configuration (overrides global)
        timeout_seconds = 60
        log_level = debug
        snake_case_variables = false
        "#;

        let mut file = fs::File::create(&local_config_path).unwrap();
        file.write_all(local_config_content.as_bytes()).unwrap();

        let config = load_config(script_dir.path());

        unsafe {
            std::env::remove_var("WFL_GLOBAL_CONFIG_PATH");
        }

        assert_eq!(config.timeout_seconds, 60); // Local override
        assert!(config.logging_enabled); // From global
        assert_eq!(config.max_line_length, 120); // From global
        assert_eq!(config.log_level, LogLevel::Debug); // Local override
        assert!(!config.snake_case_variables); // Local override
    }
}
