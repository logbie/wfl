use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{Duration, Instant};

/// Test runner for WFL test suite
/// Usage: cargo run --bin test_runner -- [all|unit|integration|e2e]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let test_type = args.get(1).cloned().unwrap_or_else(|| "all".to_string());
    
    match test_type.as_str() {
        "all" => run_all_tests(),
        "unit" => run_unit_tests(),
        "integration" => run_integration_tests(),
        "e2e" => run_e2e_tests(),
        _ => {
            println!("Unknown test type: {}", test_type);
            println!("Usage: cargo run --bin test_runner -- [all|unit|integration|e2e]");
        }
    }
}

/// Run all test types
fn run_all_tests() {
    println!("=========================================");
    println!("Running all WFL tests");
    println!("=========================================");
    
    let start = Instant::now();
    
    println!("\nUnit Tests:");
    run_unit_tests();
    
    println!("\nIntegration Tests:");
    run_integration_tests();
    
    println!("\nEnd-to-End Tests:");
    run_e2e_tests();
    
    let duration = start.elapsed();
    
    println!("\n=========================================");
    println!("All tests completed in {:.2}s", duration.as_secs_f64());
    println!("=========================================");
}

/// Run unit tests using cargo
fn run_unit_tests() {
    println!("Running unit tests...");
    let output = Command::new("cargo")
        .args(["test", "--lib", "--", "--nocapture", "unit::"])
        .output()
        .expect("Failed to run unit tests");
    
    print_test_output(&output);
}

/// Run integration tests using cargo
fn run_integration_tests() {
    println!("Running integration tests...");
    let output = Command::new("cargo")
        .args(["test", "--lib", "--", "--nocapture", "integration::"])
        .output()
        .expect("Failed to run integration tests");
    
    print_test_output(&output);
}

/// Run end-to-end tests by executing WFL programs
fn run_e2e_tests() {
    println!("Running end-to-end tests...");
    
    // First run the built-in E2E test cases
    let output = Command::new("cargo")
        .args(["test", "--lib", "--", "--nocapture", "end_to_end::"])
        .output()
        .expect("Failed to run E2E tests");
    
    print_test_output(&output);
    
    // Then run all WFL test programs
    run_test_programs();
}

/// Run all WFL test programs in the Test Programs directory
fn run_test_programs() {
    println!("Running WFL test programs...");
    
    let test_dir = Path::new("Test Programs");
    if !test_dir.exists() || !test_dir.is_dir() {
        println!("Test Programs directory not found!");
        return;
    }
    
    let mut passed = 0;
    let mut failed = 0;
    let mut total_duration = Duration::from_secs(0);
    
    // Get all .wfl files from the test directory
    let test_files = collect_test_files(test_dir);
    println!("Found {} test programs", test_files.len());
    
    for test_file in test_files {
        let start = Instant::now();
        let result = run_wfl_program(&test_file);
        let duration = start.elapsed();
        total_duration += duration;
        
        let file_name = test_file.file_name().unwrap().to_string_lossy();
        
        match result {
            Ok(_) => {
                println!("✅ {} passed ({:.2}s)", file_name, duration.as_secs_f64());
                passed += 1;
            },
            Err(err) => {
                println!("❌ {} failed: {}", file_name, err);
                failed += 1;
            }
        }
    }
    
    println!("\nTest Program Results:");
    println!("Total: {}", passed + failed);
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    println!("Duration: {:.2}s", total_duration.as_secs_f64());
}

/// Collect all .wfl files in a directory and its subdirectories
fn collect_test_files(dir: &Path) -> Vec<PathBuf> {
    let mut test_files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_dir() {
                // Recursively collect test files from subdirectories
                let sub_test_files = collect_test_files(&path);
                test_files.extend(sub_test_files);
            } else if path.extension().map_or(false, |ext| ext == "wfl") {
                test_files.push(path);
            }
        }
    }
    
    test_files
}

/// Run a single WFL program and check its output against expected output
fn run_wfl_program(program_path: &Path) -> Result<(), String> {
    // Get the expected output file path
    let expected_path = get_expected_output_path(program_path);
    
    // Check if expected output file exists
    if !expected_path.exists() {
        return Err(format!("Expected output file not found: {:?}", expected_path));
    }
    
    // Run the WFL program
    let output = Command::new("cargo")
        .args(["run", "--", program_path.to_str().unwrap()])
        .output()
        .map_err(|e| format!("Failed to run program: {}", e))?;
    
    if !output.status.success() {
        return Err(format!("Program execution failed with status: {}", output.status));
    }
    
    // Get program output
    let program_output = String::from_utf8_lossy(&output.stdout).to_string();
    
    // Get expected output
    let expected_output = fs::read_to_string(&expected_path)
        .map_err(|e| format!("Failed to read expected output: {}", e))?;
    
    // Compare outputs
    if normalize_output(&program_output) == normalize_output(&expected_output) {
        Ok(())
    } else {
        Err(format!(
            "Output does not match expected output.\nExpected:\n{}\n\nActual:\n{}",
            expected_output, program_output
        ))
    }
}

/// Get the expected output file path for a test program
fn get_expected_output_path(program_path: &Path) -> PathBuf {
    let stem = program_path.file_stem().unwrap();
    let parent = program_path.parent().unwrap();
    parent.join(format!("{}.expected", stem.to_string_lossy()))
}

/// Normalize output for comparison (trim whitespace, standardize line endings)
fn normalize_output(output: &str) -> String {
    output
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}

/// Print test output
fn print_test_output(output: &Output) {
    if output.status.success() {
        println!("Tests passed!");
    } else {
        println!("Tests failed!");
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if !stdout.is_empty() {
        println!("Output:");
        println!("{}", stdout);
    }
    
    if !stderr.is_empty() {
        println!("Errors:");
        println!("{}", stderr);
    }
} 