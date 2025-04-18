use std::path::Path;

pub fn load_timeout(dir: &Path) -> u64 {
    let default = 60;
    let file = dir.join(".wflcfg");

    if let Ok(text) = std::fs::read_to_string(&file) {
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            if let Some(rest) = line.strip_prefix("timeout_seconds") {
                if let Some(value_str) = rest.split('=').nth(1) {
                    if let Ok(value) = value_str.trim().parse::<u64>() {
                        let value = value.max(1);
                        log::debug!(
                            "Loaded timeout override: {} s from {}",
                            value,
                            file.display()
                        );
                        return value;
                    }
                }
            }
        }
    }

    default
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
}
