#[cfg(test)]
mod bytecode_compiler_tests {
    // Importing required modules
    use crate::bytecode::{Compiler, OpCode, Chunk};
    use crate::parser::{Parser, Expression, ExpressionKind, Statement, StatementKind};
    use crate::lexer::Lexer;
    
    // Helper function to compile an expression
    fn compile_expression(input: &str) -> Chunk {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let expr = parser.parse_expression();
        
        let mut compiler = Compiler::new();
        compiler.compile_expression(&expr);
        compiler.emit_opcode(OpCode::Return);
        
        compiler.chunk()
    }
    
    // Helper function to compile a statement
    fn compile_statement(input: &str) -> Chunk {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let stmt = parser.parse_statement();
        
        let mut compiler = Compiler::new();
        compiler.compile_statement(&stmt);
        compiler.emit_opcode(OpCode::Return);
        
        compiler.chunk()
    }
    
    #[test]
    fn test_literal_compilation() {
        // Test number literal
        let chunk = compile_expression("42");
        assert_eq!(chunk.code[0], OpCode::Constant as u8);
        // Assuming the constant is at index 0 in the constant pool
        assert_eq!(chunk.code[1], 0);
        assert_eq!(chunk.code[2], OpCode::Return as u8);
        // The constant at index 0 should be 42
        assert_eq!(chunk.constants[0].to_string(), "42");
        
        // Test string literal
        let chunk = compile_expression(r#""hello""#);
        assert_eq!(chunk.code[0], OpCode::Constant as u8);
        assert_eq!(chunk.constants[0].to_string(), "hello");
        
        // Test boolean literal
        let chunk = compile_expression("true");
        assert_eq!(chunk.code[0], OpCode::True as u8);
        assert_eq!(chunk.code[1], OpCode::Return as u8);
        
        let chunk = compile_expression("false");
        assert_eq!(chunk.code[0], OpCode::False as u8);
        assert_eq!(chunk.code[1], OpCode::Return as u8);
    }
    
    #[test]
    fn test_binary_expression_compilation() {
        // Test addition
        let chunk = compile_expression("5 + 3");
        
        // Expected bytecode:
        // 1. Load constant 5
        // 2. Load constant 3
        // 3. Add
        // 4. Return
        
        assert_eq!(chunk.code[0], OpCode::Constant as u8);
        assert_eq!(chunk.constants[0].to_string(), "5");
        
        assert_eq!(chunk.code[2], OpCode::Constant as u8);
        assert_eq!(chunk.constants[1].to_string(), "3");
        
        assert_eq!(chunk.code[4], OpCode::Add as u8);
        assert_eq!(chunk.code[5], OpCode::Return as u8);
        
        // Test other binary operations
        let operations = [
            ("10 - 2", OpCode::Subtract),
            ("7 * 4", OpCode::Multiply),
            ("20 / 5", OpCode::Divide),
            ("x > y", OpCode::Greater),
            ("a < b", OpCode::Less),
            ("p >= q", OpCode::GreaterEqual),
            ("m <= n", OpCode::LessEqual),
            ("foo == bar", OpCode::Equal),
            ("x != y", OpCode::NotEqual),
            ("true and false", OpCode::And),
            ("a or b", OpCode::Or),
        ];
        
        for (input, op_code) in operations.iter() {
            let chunk = compile_expression(input);
            
            // Find the operation in the chunk
            let mut found = false;
            for i in 0..chunk.code.len() {
                if chunk.code[i] == *op_code as u8 {
                    found = true;
                    break;
                }
            }
            
            assert!(found, "Expected OpCode::{:?} in bytecode for expression {}", op_code, input);
        }
    }
    
    #[test]
    fn test_unary_expression_compilation() {
        // Test negation
        let chunk = compile_expression("-42");
        
        // Expected bytecode:
        // 1. Load constant 42
        // 2. Negate
        // 3. Return
        
        assert_eq!(chunk.code[0], OpCode::Constant as u8);
        assert_eq!(chunk.constants[0].to_string(), "42");
        
        assert_eq!(chunk.code[2], OpCode::Negate as u8);
        assert_eq!(chunk.code[3], OpCode::Return as u8);
        
        // Test logical not
        let chunk = compile_expression("not true");
        
        // Expected bytecode:
        // 1. True
        // 2. Not
        // 3. Return
        
        assert_eq!(chunk.code[0], OpCode::True as u8);
        assert_eq!(chunk.code[1], OpCode::Not as u8);
        assert_eq!(chunk.code[2], OpCode::Return as u8);
    }
    
    #[test]
    fn test_variable_declaration_compilation() {
        let chunk = compile_statement("define variable x = 10");
        
        // Expected bytecode:
        // 1. Load constant 10
        // 2. DefineGlobal "x"
        // 3. Return
        
        assert_eq!(chunk.code[0], OpCode::Constant as u8);
        assert_eq!(chunk.constants[0].to_string(), "10");
        
        assert_eq!(chunk.code[2], OpCode::DefineGlobal as u8);
        // The next byte should be the index of "x" in the constant pool
        assert_eq!(chunk.constants[chunk.code[3] as usize].to_string(), "x");
        
        assert_eq!(chunk.code[4], OpCode::Return as u8);
    }
    
    #[test]
    fn test_variable_access_compilation() {
        let chunk = compile_expression("myVar");
        
        // Expected bytecode:
        // 1. GetGlobal "myVar"
        // 2. Return
        
        assert_eq!(chunk.code[0], OpCode::GetGlobal as u8);
        // The next byte should be the index of "myVar" in the constant pool
        assert_eq!(chunk.constants[chunk.code[1] as usize].to_string(), "myVar");
        
        assert_eq!(chunk.code[2], OpCode::Return as u8);
    }
    
    #[test]
    fn test_assignment_compilation() {
        let chunk = compile_statement("x = 42");
        
        // Expected bytecode:
        // 1. Load constant 42
        // 2. SetGlobal "x"
        // 3. Return
        
        assert_eq!(chunk.code[0], OpCode::Constant as u8);
        assert_eq!(chunk.constants[0].to_string(), "42");
        
        assert_eq!(chunk.code[2], OpCode::SetGlobal as u8);
        // The next byte should be the index of "x" in the constant pool
        assert_eq!(chunk.constants[chunk.code[3] as usize].to_string(), "x");
        
        assert_eq!(chunk.code[4], OpCode::Return as u8);
    }
    
    #[test]
    fn test_if_statement_compilation() {
        let chunk = compile_statement("if x > 5\ndefine variable result = \"greater\"\nend if");
        
        // Expected bytecode should include:
        // 1. Load variable x
        // 2. Load constant 5
        // 3. Greater
        // 4. JumpIfFalse to after the if block
        // ... if block code ...
        // 5. Jump to after the else block (if there is one)
        
        // Just check for a few key opcodes
        let mut found_get_global = false;
        let mut found_greater = false;
        let mut found_jump_if_false = false;
        
        for i in 0..chunk.code.len() {
            match chunk.code[i] {
                x if x == OpCode::GetGlobal as u8 => found_get_global = true,
                x if x == OpCode::Greater as u8 => found_greater = true,
                x if x == OpCode::JumpIfFalse as u8 => found_jump_if_false = true,
                _ => {}
            }
        }
        
        assert!(found_get_global, "Expected GetGlobal in if statement bytecode");
        assert!(found_greater, "Expected Greater in if statement bytecode");
        assert!(found_jump_if_false, "Expected JumpIfFalse in if statement bytecode");
    }
    
    // Add more tests as needed for:
    // - while loops
    // - function calls
    // - container definitions
    // - action definitions
    // - collection operations
} 