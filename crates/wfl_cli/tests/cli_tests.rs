#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use tempfile::tempdir;
    use wfl_cli::create_new_project;

    #[test]
    fn test_cli_version() {
        let output = Command::new("../../../target/debug/wfl")
            .arg("--version")
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

        assert!(output.status.success(), "Version command failed");
        assert!(
            stdout.contains("WebFirst Language"),
            "Version output should contain WebFirst Language"
        );
    }

    #[test]
    fn test_project_scaffolding() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let project_name = "test_project";

        let current_dir = env::current_dir().expect("Failed to get current directory");

        env::set_current_dir(temp_dir.path()).expect("Failed to change to temp directory");

        let result = create_new_project(project_name, false);

        let project_path = temp_dir.path().join(project_name);
        assert!(result.is_ok(), "Project creation failed: {:?}", result);
        assert!(project_path.exists(), "Project directory was not created");
        assert!(
            project_path.join("src").exists(),
            "src directory was not created"
        );
        assert!(
            project_path.join("src/main.wfl").exists(),
            "main.wfl was not created"
        );
        assert!(
            project_path.join("README.md").exists(),
            "README.md was not created"
        );
        assert!(
            project_path.join(".gitignore").exists(),
            ".gitignore was not created"
        );

        env::set_current_dir(current_dir).expect("Failed to restore original directory");
    }

    #[cfg(feature = "editor")]
    #[test]
    fn test_project_scaffolding_with_editor() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let project_name = "test_editor_project";

        let current_dir = env::current_dir().expect("Failed to get current directory");

        env::set_current_dir(temp_dir.path()).expect("Failed to change to temp directory");

        let result = create_new_project(project_name, true);

        let project_path = temp_dir.path().join(project_name);
        assert!(
            result.is_ok(),
            "Project creation with editor failed: {:?}",
            result
        );
        assert!(project_path.exists(), "Project directory was not created");
        assert!(
            project_path.join("src").exists(),
            "src directory was not created"
        );
        assert!(
            project_path.join("src/main.wfl").exists(),
            "main.wfl was not created"
        );
        assert!(
            project_path.join("README.md").exists(),
            "README.md was not created"
        );
        assert!(
            project_path.join(".gitignore").exists(),
            ".gitignore was not created"
        );
        assert!(
            project_path.join("wfl-editor.toml").exists(),
            "wfl-editor.toml was not created"
        );

        env::set_current_dir(current_dir).expect("Failed to restore original directory");
    }
}
