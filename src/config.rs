use std::path::Path;

#[derive(Debug, Clone)]
pub struct WflConfig {
    pub timeout_seconds: u64,
    pub logging_enabled: bool,
    pub debug_report_enabled: bool,
    pub log_level: LogLevel,
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
        }
    }
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" | "warning" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info, // Default to Info for unrecognized values
        }
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

pub fn load_config(dir: &Path) -> WflConfig {
    let mut config = WflConfig::default();
    let file = dir.join(".wflcfg");

    if let Ok(text) = std::fs::read_to_string(&file) {
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
                            config.timeout_seconds = timeout.max(1);
                            log::debug!(
                                "Loaded timeout override: {} s from {}",
                                config.timeout_seconds,
                                file.display()
                            );
                        }
                    },
                    "logging_enabled" => {
                        if let Ok(enabled) = value.parse::<bool>() {
                            config.logging_enabled = enabled;
                            log::debug!(
                                "Loaded logging_enabled: {} from {}",
                                config.logging_enabled,
                                file.display()
                            );
                        }
                    },
                    "debug_report_enabled" => {
                        if let Ok(enabled) = value.parse::<bool>() {
                            config.debug_report_enabled = enabled;
                            log::debug!(
                                "Loaded debug_report_enabled: {} from {}",
                                config.debug_report_enabled,
                                file.display()
                            );
                        }
                    },
                    "log_level" => {
                        config.log_level = LogLevel::from_str(value);
                        log::debug!(
                            "Loaded log_level: {:?} from {}",
                            config.log_level,
                            file.display()
                        );
                    },
                    _ => {}
                }
            }
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

        let timeout = load_timeout(temp_dir.path());
        assert_eq!(timeout, 60); // Should fall back to default
    }
    
    #[test]
    fn test_load_config_defaults() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = load_config(temp_dir.path());
        
        assert_eq!(config.timeout_seconds, 60);
        assert_eq!(config.logging_enabled, false);
        assert_eq!(config.debug_report_enabled, true);
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
        assert_eq!(config.logging_enabled, true);
        assert_eq!(config.debug_report_enabled, false);
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
        
        let config = load_config(temp_dir.path());
        
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.logging_enabled, false); // Default
        assert_eq!(config.debug_report_enabled, true); // Default
        assert_eq!(config.log_level, LogLevel::Error);
    }

    #[test]
    fn test_log_level_parsing() {
        assert_eq!(LogLevel::from_str("debug"), LogLevel::Debug);
        assert_eq!(LogLevel::from_str("INFO"), LogLevel::Info);
        assert_eq!(LogLevel::from_str("Warning"), LogLevel::Warn);
        assert_eq!(LogLevel::from_str("warn"), LogLevel::Warn);
        assert_eq!(LogLevel::from_str("ERROR"), LogLevel::Error);
        assert_eq!(LogLevel::from_str("unknown"), LogLevel::Info); // Default
    }
}
