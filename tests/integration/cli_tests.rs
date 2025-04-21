use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_lint_fix_diff_combined() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test_file.wfl");
    
    let test_content = r#"
store myVariable as 42
display myVariable
"#;
    
    fs::write(&file_path, test_content).expect("Failed to write test file");
    
    let output = Command::new(env!("CARGO_BIN_EXE_wfl"))
        .args(&["--lint", "--fix", "--diff", file_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "Command failed: {:?}", output);
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    assert!(output_str.contains("-store myVariable as 42"), 
            "Diff doesn't contain the original line: {}", output_str);
    assert!(output_str.contains("+store my_variable as 42"), 
            "Diff doesn't contain the fixed line: {}", output_str);
}
