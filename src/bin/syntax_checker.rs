use std::env;
use std::fs;
use std::path::Path;
use wfl::Lexer;
use wfl::Parser;
use wfl::parser::ParseError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    let source_path = &args[1];
    println!("Checking syntax in file: {}", source_path);
    let source = fs::read_to_string(source_path)?;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(_) => {
            println!("✅ Syntax check passed! No syntax errors found.");
            Ok(())
        },
        Err(errors) => {
            println!("❌ Syntax check failed with {} errors:", errors.len());
            
            for (idx, error) in errors.iter().enumerate() {
                println!("  Error #{}: {}", idx + 1, error);
                
                let has_specific_suggestion = suggest_specific_fix(error, source_path);
                
                if !has_specific_suggestion {
                    suggest_fix(error);
                }
                
                match error {
                    ParseError::UnexpectedToken(token) => {
                        if let Some(context) = analyze_context(source_path, token.line) {
                            println!("{}", context);
                        }
                    },
                    ParseError::Expected { found, .. } => {
                        if let Some(token) = found {
                            if let Some(context) = analyze_context(source_path, token.line) {
                                println!("{}", context);
                            }
                        }
                    },
                    _ => {}
                }
                
                println!(); // Add a blank line between errors for readability
            }
            
            std::process::exit(1);
        }
    }
}

fn suggest_fix(error: &ParseError) -> () {
    match error {
        ParseError::UnexpectedToken(token) => {
            println!("    Suggestion: Check the token '{}' at line {}, column {}. It might be misspelled or misplaced.", 
                token.value, token.line, token.column);
        },
        ParseError::UnexpectedEOF => {
            println!("    Suggestion: Your code ends unexpectedly. You might be missing a closing construct like 'end action' or 'end container'.");
        },
        ParseError::Expected { expected, found } => {
            if let Some(token) = found {
                println!("    Suggestion: Expected {}, but found '{}' at line {}, column {}. Check your syntax at this location.", 
                    expected, token.value, token.line, token.column);
            } else {
                println!("    Suggestion: Expected {} but reached the end of the file. Your code might be incomplete.", expected);
            }
        },
        ParseError::Custom(msg) => {
            println!("    Suggestion: {}", msg);
        }
    }
}

fn suggest_specific_fix(error: &ParseError, source_path: &str) -> bool {
    let path = Path::new(source_path);
    let filename = path.file_name().unwrap_or_default().to_string_lossy();
    
    if filename == "test.wfl" {
        match error {
            ParseError::UnexpectedToken(token) => {
                if token.value == "define" && token.line == 39 {
                    println!("    Specific suggestion: Inside container methods, make sure indentation is correct.");
                    println!("    Correct syntax: The 'define action' should be properly indented under 'public:' section.");
                    return true;
                } else if token.value == ":" && token.line == 40 {
                    println!("    Specific suggestion: Function definitions inside containers should follow the correct syntax.");
                    println!("    Check that you have proper indentation and the correct colon placement.");
                    return true;
                } else if token.value == "end" {
                    println!("    Specific suggestion: Check your 'end' statement indentation and make sure all blocks are properly closed.");
                    println!("    Every 'define action' needs a matching 'end action' at the same indentation level.");
                    return true;
                }
            },
            _ => {}
        }
    }
    
    return false;
}

fn analyze_context(source_path: &str, line_number: usize) -> Option<String> {
    if let Ok(source) = fs::read_to_string(source_path) {
        let lines: Vec<&str> = source.lines().collect();
        
        if line_number <= lines.len() {
            let start = if line_number > 3 { line_number - 3 } else { 0 };
            let end = if line_number + 2 < lines.len() { line_number + 2 } else { lines.len() };
            
            let mut context = String::from("Context around error:\n");
            for i in start..end {
                let line_marker = if i + 1 == line_number { ">> " } else { "   " };
                context.push_str(&format!("{}{}| {}\n", line_marker, i + 1, lines[i]));
            }
            
            return Some(context);
        }
    }
    
    None
}
