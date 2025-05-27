#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use tempfile::tempdir;
    
    #[test]
    fn test_cli_version() {
        let output = Command::new("cargo")
            .args(["run", "--", "--version"])
            .output()
            .expect("Failed to execute version command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        assert!(output.status.success());
        assert!(stdout.contains("WebFirst Language"));
    }
    
    #[test]
    fn test_project_scaffolding() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let project_name = "test_project";
        let project_path = temp_dir.path().join(project_name);
        
        let output = Command::new("cargo")
            .args(["run", "--", "new", project_name])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to execute new project command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        println!("STDOUT: {}", stdout);
        println!("STDERR: {}", stderr);
        
        assert!(output.status.success(), "Project creation failed: {}", stderr);
        
        assert!(project_path.exists(), "Project directory was not created");
        assert!(project_path.join("src").exists(), "src directory was not created");
        assert!(project_path.join("src/main.wfl").exists(), "main.wfl was not created");
        assert!(project_path.join("README.md").exists(), "README.md was not created");
        assert!(project_path.join(".gitignore").exists(), ".gitignore was not created");
    }
    
    #[cfg(feature = "editor")]
    #[test]
    fn test_project_scaffolding_with_editor() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let project_name = "test_editor_project";
        let project_path = temp_dir.path().join(project_name);
        
        let output = Command::new("cargo")
            .args(["run", "--features", "editor", "--", "new", project_name, "--with-editor"])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to execute new project with editor command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        println!("STDOUT: {}", stdout);
        println!("STDERR: {}", stderr);
        
        assert!(output.status.success(), "Project creation with editor failed: {}", stderr);
        
        assert!(project_path.exists(), "Project directory was not created");
        assert!(project_path.join("src").exists(), "src directory was not created");
        assert!(project_path.join("src/main.wfl").exists(), "main.wfl was not created");
        assert!(project_path.join("README.md").exists(), "README.md was not created");
        assert!(project_path.join(".gitignore").exists(), ".gitignore was not created");
        assert!(project_path.join("wfl-editor.toml").exists(), "wfl-editor.toml was not created");
    }
}
