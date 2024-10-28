use std::fs;
use wfl::lexer::{Lexer, TokenType};
use yansi::Paint;

fn main() {
    // Read the file content
    let file_content = match fs::read_to_string("hello.wfl") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}: {}", Paint::red("Error reading file").bold(), e);
            return;
        }
    };
    
    println!("{}", Paint::green("Lexical Analysis of hello.wfl:").bold());
    println!("----------------------------------------\n");
    
    // Create lexer with file content
    let mut lexer = Lexer::new(&file_content);
    let mut last_line = 1;
    let mut errors: Vec<(TokenLocation, String)> = Vec::new();
    
    // Process all tokens
    loop {
        let token = lexer.next_token();
        
        // Add blank line between different source lines for readability
        if token.location.line > last_line {
            println!();
            last_line = token.location.line;
        }
        
        // Format the output based on token type
        let token_info = match &token.token_type {
            TokenType::StringLiteral(s) => format!("{:<12} -> \"{}\"", Paint::cyan("String"), s),
            TokenType::NumberLiteral(n) => format!("{:<12} -> {}", Paint::yellow("Number"), n),
            TokenType::Identifier(id) => format!("{:<12} -> {}", Paint::blue("ID"), id),
            TokenType::Property(prop) => format!("{:<12} -> {}", Paint::magenta("Property"), prop),
            TokenType::TruthLiteral(b) => format!("{:<12} -> {}", Paint::green("Truth"), b),
            TokenType::Invalid(e) => {
                errors.push((token.location.clone(), e.clone()));
                format!("{:<12} -> {}", Paint::red("ERROR").bold(), e)
            },
            TokenType::Newline => format!("{}", Paint::new("NEWLINE").dim()),
            TokenType::EOF => {
                println!("\n{}", Paint::green("End of file reached"));
                break;
            },
            _ => format!("{:<12}", Paint::new(format!("{:?}", token.token_type))),
        };
        
        // Print token information with location
        println!("Line {:>4}, Col {:>2} ({}): {}", 
            token.location.line,
            token.location.column,
            token.location.length,
            token_info
        );
        
        // For errors, show the line context
        if matches!(token.token_type, TokenType::Invalid(_)) {
            println!("  {}", Paint::new(&token.location.line_content).dim());
            println!("  {}{}",
                " ".repeat(token.location.column - 1),
                Paint::red(&"^".repeat(token.location.length))
            );
        }
    }
    
    // Print summary of errors if any
    if !errors.is_empty() {
        println!("\n{}", Paint::red("Errors found:").bold());
        for (location, error) in errors {
            println!("Line {}, Column {}: {}", 
                location.line,
                location.column,
                Paint::red(error)
            );
        }
    }
}
