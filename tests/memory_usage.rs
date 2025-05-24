#[cfg(feature = "dhat-heap")]
mod tests {

    use std::rc::Rc;
    use wfl::interpreter::Interpreter;
    use wfl::lexer::lex_wfl_with_positions;
    use wfl::parser::Parser;

    #[test]
    fn basic_allocations() {
        let _profiler = dhat::Profiler::builder().testing().build();

        let v = vec![1u8; 256];

        let stats = dhat::HeapStats::get();

        assert!(
            stats.max_bytes < 1024,
            "Max bytes exceeded limit: {} >= 1024",
            stats.max_bytes
        );
        drop(v);
    }

    #[tokio::test]
    async fn interpreter_small_program() {
        let _profiler = dhat::Profiler::builder().testing().build();

        let source = r#"
        store x as 42
        store y as x plus 10
        "#;

        let tokens = lex_wfl_with_positions(source);
        let mut parser = Parser::new(&tokens);
        let program = parser.parse().unwrap();

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program).await;
        assert!(result.is_ok());

        let stats = dhat::HeapStats::get();

        assert!(
            stats.max_bytes < 15 * 1024, // Increased limit to account for step mode overhead
            "Max bytes exceeded limit: {} >= {}",
            stats.max_bytes,
            15 * 1024
        );

        drop(interpreter);
    }

    #[tokio::test]
    async fn test_functions_memory_usage() {
        let _profiler = dhat::Profiler::builder().testing().build();

        let source = r#"
        define action called double needs x:
            return x times 2
        end action
        
        store result as double with 21
        "#;

        let tokens = lex_wfl_with_positions(source);
        let mut parser = Parser::new(&tokens);
        let program = parser.parse().unwrap();

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program).await;
        assert!(result.is_ok());

        let stats = dhat::HeapStats::get();

        assert!(
            stats.max_bytes < 20 * 1024,
            "Max bytes exceeded limit: {} >= {}",
            stats.max_bytes,
            20 * 1024
        );
        assert!(
            stats.total_blocks < 1000,
            "Total blocks exceeded limit: {} >= 1000",
            stats.total_blocks
        );

        drop(interpreter);
    }

    #[tokio::test]
    async fn test_environment_memory_usage() {
        let _profiler = dhat::Profiler::builder().testing().build();

        let source = r#"
        store global_var as "global"
        
        define action called create_counter:
            store counter_value as 0
            
            define action called increment:
                store counter_value as counter_value plus 1
                return counter_value
            end action
            
            return increment
        end action
        
        store counter as create_counter with nothing
        store result1 as counter with nothing
        store result2 as counter with nothing
        "#;

        let tokens = lex_wfl_with_positions(source);
        let mut parser = Parser::new(&tokens);
        let program = parser.parse().unwrap();

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program).await;
        assert!(result.is_ok());

        let global_env = interpreter.global_env();
        let result2 = global_env.borrow().get("result2").unwrap();
        assert_eq!(result2.to_string(), "2");

        let stats = dhat::HeapStats::get();

        assert!(
            stats.max_bytes < 25 * 1024,
            "Max bytes exceeded limit: {} >= {}",
            stats.max_bytes,
            25 * 1024
        );

        let rc_count = Rc::strong_count(global_env);
        assert_eq!(
            rc_count, 1,
            "Global environment should have exactly one reference"
        );

        drop(interpreter);
    }
}
