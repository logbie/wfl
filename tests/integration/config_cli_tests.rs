use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_config_check_valid() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join(".wflcfg");
    
    let config_content = r#"
# Valid configuration
timeout_seconds = 30
logging_enabled = true
log_level = info
max_line_length = 80
"#;
    
    fs::write(&config_path, config_content).expect("Failed to write config file");
    
    let output = Command::new(env!("CARGO_BIN_EXE_wfl"))
        .args(&["--configCheck", temp_dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "Command failed: {:?}", output);
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("✅ No configuration issues found!"), 
            "Output doesn't contain success message: {}", output_str);
}

#[test]
fn test_config_check_invalid() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join(".wflcfg");
    
    let config_content = r#"
# Invalid configuration
timeout_seconds = potato
unknown_key = value
"#;
    
    fs::write(&config_path, config_content).expect("Failed to write config file");
    
    let output = Command::new(env!("CARGO_BIN_EXE_wfl"))
        .args(&["--configCheck", temp_dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success(), "Command should have failed");
    assert_eq!(output.status.code(), Some(1), "Expected exit code 1");
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("❌ Error: Invalid type for timeout_seconds"), 
            "Output doesn't contain error message: {}", output_str);
    assert!(output_str.contains("⚠️ Warning: Unknown configuration key"), 
            "Output doesn't contain warning message: {}", output_str);
}

#[test]
fn test_config_fix() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join(".wflcfg");
    
    let config_content = r#"
# Invalid configuration
timeout_seconds = potato
"#;
    
    fs::write(&config_path, config_content).expect("Failed to write config file");
    
    let output = Command::new(env!("CARGO_BIN_EXE_wfl"))
        .args(&["--configFix", temp_dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "Command failed: {:?}", output);
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("✅ Fixed value for 'timeout_seconds'"), 
            "Output doesn't contain fix message: {}", output_str);
    
    let fixed_content = fs::read_to_string(&config_path).expect("Failed to read fixed config");
    assert!(fixed_content.contains("timeout_seconds = 60"), 
            "Config file wasn't fixed correctly: {}", fixed_content);
    
    let check_output = Command::new(env!("CARGO_BIN_EXE_wfl"))
        .args(&["--configCheck", temp_dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");
    
    assert!(check_output.status.success(), "Check command failed after fix: {:?}", check_output);
    
    let check_output_str = String::from_utf8_lossy(&check_output.stdout);
    assert!(check_output_str.contains("✅ No configuration issues found!"), 
            "Output doesn't contain success message after fix: {}", check_output_str);
}

#[test]
fn test_config_check_no_args() {
    let output = Command::new(env!("CARGO_BIN_EXE_wfl"))
        .args(&["--configCheck"])
        .output()
        .expect("Failed to execute command");
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("Checking WFL configuration"), 
            "Output doesn't contain expected message: {}", output_str);
}

#[test]
fn test_config_flags_mutually_exclusive() {
    let output = Command::new(env!("CARGO_BIN_EXE_wfl"))
        .args(&["--configCheck", "--configFix"])
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success(), "Command should have failed");
    assert_eq!(output.status.code(), Some(2), "Expected exit code 2");
    
    let error_str = String::from_utf8_lossy(&output.stderr);
    assert!(error_str.contains("cannot be combined with"), 
            "Error doesn't contain expected message: {}", error_str);
}
