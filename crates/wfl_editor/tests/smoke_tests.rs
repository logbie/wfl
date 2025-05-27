#[cfg(test)]
mod tests {
    

    #[test]
    fn test_editor_version() {
        #[cfg(feature = "gui")]
        {
            let output = Command::new(env!("CARGO_BIN_EXE_wfl"))
                .args(["editor", "--version"])
                .output();

            if output.is_err() {
                assert!(
                    wfl_core::version::VERSION.len() > 0,
                    "Version should be defined"
                );
                return;
            }

            let output = output.unwrap();
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            println!("STDOUT: {}", stdout);
            println!("STDERR: {}", stderr);

            if stdout.contains("cannot be launched in headless environment") {
                assert!(
                    stdout.contains("Version:"),
                    "Version information should be displayed"
                );
                assert!(
                    stdout.contains("WebFirst Language Editor"),
                    "Should show editor name"
                );
            } else {
                assert!(
                    stdout.contains("WebFirst Language"),
                    "Version output should contain WebFirst Language"
                );
                assert!(
                    stdout.contains("Editor"),
                    "Version output should contain Editor"
                );
                assert!(
                    output.status.success(),
                    "Editor command should exit successfully"
                );
            }
        }

        #[cfg(not(feature = "gui"))]
        {}
    }

    #[test]
    fn test_editor_config_env_override() {
        unsafe {
            ::std::env::set_var("WFL_EDITOR_TELEMETRY", "1");
        }
        let config = wfl_editor::EditorConfig::load();
        assert!(
            config.telemetry_enabled,
            "Telemetry should be enabled with env var set to 1"
        );

        unsafe {
            ::std::env::set_var("WFL_EDITOR_TELEMETRY", "0");
        }
        let config = wfl_editor::EditorConfig::load();
        assert!(
            !config.telemetry_enabled,
            "Telemetry should be disabled with env var set to 0"
        );

        unsafe {
            ::std::env::remove_var("WFL_EDITOR_TELEMETRY");
        }
    }
}
