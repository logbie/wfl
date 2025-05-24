use std::fs;
use std::path::Path;
use wfl::interpreter::Interpreter;
use wfl::lexer::lex_wfl_with_positions;
use wfl::parser::Parser;

#[tokio::test]
async fn test_nexus_fixture_runs() {
    // Path to the test fixture
    let fixture_path = "tests/fixtures/nexus.wfl";
    let log_path = "test.log";
    
    // Remove log file if it exists
    let _ = fs::remove_file(log_path);
    
    // Read the fixture file
    let source = fs::read_to_string(fixture_path).expect("Failed to read fixture file");
    
    // Parse and interpret
    let tokens = lex_wfl_with_positions(&source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().expect("Failed to parse program");
    
    // Run the interpreter
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program).await;
    assert!(result.is_ok(), "Failed to execute program: {:?}", result);
    
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Verify log file was created and contains expected content
    assert!(Path::new(log_path).exists(), "Log file was not created");
    
    let log_content = fs::read_to_string(log_path).expect("Failed to read log file");
    assert!(
        log_content.contains("Starting Nexus WFL Integration Test Suite..."),
        "Log file does not contain expected content"
    );
}
