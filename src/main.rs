use std::env;
use std::fs;
use wfl::Lexer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if a file path was provided
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    // Read the source file
    let source = fs::read_to_string(&args[1])?;
    
    // Create lexer and process the source
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    // Print the tokens
    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
