#[cfg(feature = "dhat-heap")]
mod tests {
    use std::fs::{self, File};
    use std::io::{Read, Write};
    use std::path::Path;
    use wfl::interpreter::Interpreter;
    use wfl::lexer::lex_wfl_with_positions;
    use wfl::parser::Parser;

    #[tokio::test]
    async fn test_log_message_memory_usage() {
        let _profiler = dhat::Profiler::builder().testing().build();

        let log_path = "temp_nexus.log";
        let _ = fs::remove_file(log_path); // Remove if exists

        let source = r#"
        open file at "temp_nexus.log" as logHandle
        
        define action called log_message needs message_text:
            wait for append message_text with "\n" into logHandle
        end action
        
        log_message with "Starting Nexus WFL Integration Test Suite..."
        log_message with "Second log message"
        log_message with "Third log message"
        "#;

        let tokens = lex_wfl_with_positions(source);
        let mut parser = Parser::new(&tokens);
        let program = parser.parse().unwrap();

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program).await;
        assert!(result.is_ok());

        let mut log_content = String::new();
        File::open(log_path)
            .unwrap()
            .read_to_string(&mut log_content)
            .unwrap();
        assert!(log_content.contains("Starting Nexus WFL Integration Test Suite..."));
        assert!(log_content.contains("Second log message"));
        assert!(log_content.contains("Third log message"));

        let stats = dhat::HeapStats::get();
        assert!(
            stats.max_bytes < 15 * 1024 * 1024, // 15 MB max
            "Max memory usage exceeded limit: {} bytes >= 15 MB",
            stats.max_bytes
        );

        let _ = fs::remove_file(log_path);

        drop(interpreter);
    }
}
