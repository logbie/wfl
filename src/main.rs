use std::env;
use std::fs;
use wfl::Lexer;
use wfl::Parser;
use wfl::Compiler;
use wfl::parser::Statement;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if a file path was provided
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    // Read the source file
    let source_path = &args[1];
    println!("Reading source file: {}", source_path);
    let source = fs::read_to_string(source_path)?;
    println!("Source content:\n{}", source);
    
    // Lexical analysis
    println!("Starting lexical analysis...");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    println!("Lexical analysis completed. Found {} tokens.", tokens.len());
    // Print the first few tokens for debugging
    for (i, token) in tokens.iter().enumerate().take(10) {
        println!("Token {}: {:?}", i, token);
    }
    
    // Parse the tokens
    println!("Starting parsing...");
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("Parsing completed successfully!");
            println!("AST contains {} top-level statements.", program.statements.len());
            
            // Print a summary of the statements
            println!("\nProgram structure:");
            for (idx, stmt) in program.statements.iter().enumerate() {
                let stmt_type: String = match stmt {
                    Statement::VariableDeclaration { .. } => "Variable Declaration".to_string(),
                    Statement::CheckStatement { .. } => "Conditional Statement".to_string(),
                    Statement::ForEachLoop { .. } => "For-Each Loop".to_string(),
                    Statement::RepeatLoop { .. } => "While/Until Loop".to_string(),
                    Statement::CountLoop { .. } => "Count Loop".to_string(),
                    Statement::TryCatch { .. } => "Try-Catch Block".to_string(),
                    Statement::ActionDefinition { name, .. } => 
                        format!("Action Definition: {}", name),
                    Statement::ContainerDefinition { name, .. } => 
                        format!("Container Definition: {}", name),
                    Statement::ExpressionStatement(_) => "Expression Statement".to_string(),
                    Statement::ReturnStatement(_) => "Return Statement".to_string(),
                    Statement::BreakStatement(_) => "Break Statement".to_string(),
                    Statement::ContinueStatement(_) => "Continue Statement".to_string(),
                };
                println!("  {}: {}", idx + 1, stmt_type);
            }
            
            // Compile to bytecode
            println!("\nStarting bytecode compilation...");
            let mut compiler = Compiler::new();
            match compiler.compile(program) {
                Ok(function) => {
                    println!("Bytecode compilation completed successfully!");
                    println!("Generated {} bytecode instructions.", function.chunk.code.len());
                    
                    // Print the bytecode for inspection
                    println!("\nBytecode:");
                    for (i, op) in function.chunk.code.iter().enumerate() {
                        println!("  {:04}: {:?}", i, op);
                    }
                    
                    // Print the constants
                    println!("\nConstants:");
                    for (i, constant) in function.chunk.constants.iter().enumerate() {
                        println!("  {:04}: {:?}", i, constant);
                    }
                },
                Err(errors) => {
                    println!("Bytecode compilation failed with {} errors:", errors.len());
                    for (idx, error) in errors.iter().enumerate() {
                        println!("  Error #{}: {}", idx + 1, error);
                    }
                    std::process::exit(1);
                }
            }
        },
        Err(errors) => {
            println!("Parsing failed with {} errors:", errors.len());
            for (idx, error) in errors.iter().enumerate() {
                println!("  Error #{}: {}", idx + 1, error);
            }
            std::process::exit(1);
        }
    }

    println!("\nWFL compilation completed successfully!");
    Ok(())
}
