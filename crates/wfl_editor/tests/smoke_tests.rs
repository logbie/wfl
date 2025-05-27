#[cfg(test)]
mod tests {
    use std::process::Command;
    
    #[test]
    #[cfg(feature = "editor")]
    fn test_editor_version() {
        let output = Command::new("cargo")
            .args(["run", "--features", "editor", "--", "editor", "--version"])
            .output()
            .expect("Failed to execute editor command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        println!("STDOUT: {}", stdout);
        println!("STDERR: {}", stderr);
        
        assert!(output.status.success(), "Editor command failed: {}", stderr);
        assert!(stdout.contains("WebFirst Language"), "Version output doesn't contain expected string");
    }
}
