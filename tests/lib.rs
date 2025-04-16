// Main test entry point for WFL test suite

// Include test utility functions
pub mod test_utils;

// Unit tests
mod unit {
    // Lexer unit tests
    pub mod lexer {
        pub mod token_tests;
        // Add more lexer test modules as needed
    }
    
    // Parser unit tests
    pub mod parser {
        pub mod expression_tests;
        pub mod statement_tests;
        pub mod container_tests;
        pub mod collection_tests;
        // Add more parser test modules as needed
    }
    
    // Bytecode compiler unit tests
    pub mod bytecode {
        pub mod compiler_tests;
        // Add more bytecode test modules as needed
    }
    
    // VM unit tests (future)
    pub mod vm {
        // VM test modules will be added here
    }
}

// Integration tests
mod integration {
    pub mod lexer_parser_tests;
    // Add more integration test modules as needed
}

// End-to-end tests
mod end_to_end {
    use std::fs;
    use std::path::Path;
    use crate::test_utils::{read_test_program, read_expected_output, compare_output};
    
    // Helper function to run a test program and check its output
    fn run_test_program(program_path: &str) {
        let program = read_test_program(program_path);
        
        // This part would need to be updated to use the actual WFL interpreter/VM
        // For now, it's just a placeholder
        let output = ""; // run_wfl_program(&program);
        
        let expected_output = read_expected_output(program_path);
        assert!(compare_output(output, &expected_output), 
                "Program output does not match expected output");
    }
    
    // Test suite for running all test programs
    #[test]
    #[ignore] // Ignore until VM is implemented
    fn test_all_programs() {
        let test_dir = Path::new("Test Programs");
        
        if test_dir.exists() && test_dir.is_dir() {
            let entries = fs::read_dir(test_dir).unwrap();
            
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                
                if path.is_file() && path.extension().map_or(false, |ext| ext == "wfl") {
                    println!("Running test program: {:?}", path);
                    run_test_program(path.to_str().unwrap());
                }
            }
        } else {
            panic!("Test Programs directory not found");
        }
    }
    
    // Tests for specific categories of programs
    mod simple_programs {
        // Tests for simple language features
    }
    
    mod control_flow {
        // Tests for control flow constructs
    }
    
    mod containers {
        // Tests for container definitions and usage
    }
    
    mod actions {
        // Tests for action definitions and calls
    }
    
    mod collections {
        // Tests for collection operations
    }
} 