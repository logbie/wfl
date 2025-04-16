// Test utilities for WFL test suite

use std::fs;
use std::path::Path;

/// Reads a test program from a file
pub fn read_test_program(filename: &str) -> String {
    fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Failed to read test file: {}", filename))
}

/// Compares actual output with expected output
pub fn compare_output(actual: &str, expected: &str) -> bool {
    // Normalize line endings and whitespace
    let actual_normalized = normalize_output(actual);
    let expected_normalized = normalize_output(expected);
    
    if actual_normalized != expected_normalized {
        println!("Expected output:");
        println!("{}", expected_normalized);
        println!("Actual output:");
        println!("{}", actual_normalized);
        false
    } else {
        true
    }
}

/// Normalizes output by trimming whitespace and standardizing line endings
fn normalize_output(output: &str) -> String {
    output
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}

/// Creates an expected output file path from a test program path
pub fn expected_output_path(test_program_path: &str) -> String {
    let path = Path::new(test_program_path);
    let parent = path.parent().unwrap_or(Path::new(""));
    let stem = path.file_stem().unwrap().to_str().unwrap();
    
    parent.join(format!("{}.expected", stem))
        .to_str()
        .unwrap()
        .to_string()
}

/// Reads expected output for a test program
pub fn read_expected_output(test_program_path: &str) -> String {
    let expected_path = expected_output_path(test_program_path);
    fs::read_to_string(&expected_path)
        .unwrap_or_else(|_| panic!("Failed to read expected output file: {}", expected_path))
}

/// Test helper for running lexer tests
pub fn test_lexer(input: &str) -> Vec<String> {
    // This is a placeholder - implement actual lexer test functionality
    // Should return a vector of token strings
    vec![]
}

/// Test helper for running parser tests
pub fn test_parser(input: &str) -> String {
    // This is a placeholder - implement actual parser test functionality
    // Should return a string representation of the AST
    String::new()
}

/// Test helper for running bytecode compiler tests
pub fn test_bytecode_compiler(input: &str) -> Vec<u8> {
    // This is a placeholder - implement actual bytecode compiler test functionality
    // Should return bytecode as a vector of bytes
    vec![]
}

// Add more test utilities as needed 