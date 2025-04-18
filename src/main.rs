use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use wfl::Interpreter;
use wfl::analyzer::Analyzer;
use wfl::config;
use wfl::lexer::{lex_wfl, lex_wfl_with_positions, token::Token};
use wfl::parser::Parser;
use wfl::typechecker::TypeChecker;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        fs::read_to_string(&args[1])?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let tokens = lex_wfl(&input);
    let tokens_with_pos = lex_wfl_with_positions(&input);

    println!("Lexer output:");
    for (i, token) in tokens.iter().enumerate() {
        println!("{}: {:?}", i, token);
    }

    println!("\nTotal tokens: {}", tokens.len());

    let keyword_count = tokens
        .iter()
        .filter(|t| {
            matches!(
                t,
                Token::KeywordStore
                    | Token::KeywordCreate
                    | Token::KeywordDisplay
                    | Token::KeywordIf
                    | Token::KeywordCheck
                    | Token::KeywordOtherwise
                    | Token::KeywordThen
                    | Token::KeywordEnd
                    | Token::KeywordAs
                    | Token::KeywordTo
                    | Token::KeywordFrom
                    | Token::KeywordWith
                    | Token::KeywordAnd
                    | Token::KeywordOr
                    | Token::KeywordCount
                    | Token::KeywordFor
                    | Token::KeywordEach
                    | Token::KeywordIn
                    | Token::KeywordReversed
                    | Token::KeywordRepeat
                    | Token::KeywordWhile
                    | Token::KeywordUntil
                    | Token::KeywordForever
                    | Token::KeywordSkip
                    | Token::KeywordContinue
                    | Token::KeywordBreak
                    | Token::KeywordExit
                    | Token::KeywordLoop
                    | Token::KeywordDefine
                    | Token::KeywordAction
                    | Token::KeywordCalled
                    | Token::KeywordNeeds
                    | Token::KeywordGive
                    | Token::KeywordBack
                    | Token::KeywordReturn
                    | Token::KeywordOpen
                    | Token::KeywordClose
                    | Token::KeywordFile
                    | Token::KeywordUrl
                    | Token::KeywordDatabase
                    | Token::KeywordAt
                    | Token::KeywordRead
                    | Token::KeywordWrite
                    | Token::KeywordContent
                    | Token::KeywordInto
                    | Token::KeywordPlus
                    | Token::KeywordMinus
                    | Token::KeywordTimes
                    | Token::KeywordDivided
                    | Token::KeywordBy
                    | Token::KeywordContains
                    | Token::KeywordAbove
                    | Token::KeywordBelow
                    | Token::KeywordEqual
                    | Token::KeywordGreater
                    | Token::KeywordLess
                    | Token::KeywordNot
                    | Token::KeywordIs
            )
        })
        .count();
    println!("Keywords: {}", keyword_count);

    let identifier_count = tokens
        .iter()
        .filter(|t| matches!(t, Token::Identifier(_)))
        .count();
    println!("Identifiers: {}", identifier_count);

    let literal_count = tokens
        .iter()
        .filter(|t| {
            matches!(
                t,
                Token::StringLiteral(_)
                    | Token::IntLiteral(_)
                    | Token::FloatLiteral(_)
                    | Token::BooleanLiteral(_)
                    | Token::NothingLiteral
            )
        })
        .count();
    println!("Literals: {}", literal_count);

    println!("\nParser output:");
    let mut parser = Parser::new(&tokens_with_pos);
    match parser.parse() {
        Ok(program) => {
            println!("AST:\n{:#?}", program);

            let mut analyzer = Analyzer::new();
            match analyzer.analyze(&program) {
                Ok(_) => {
                    println!("Semantic analysis passed.");

                    let mut type_checker = TypeChecker::new();
                    match type_checker.check_types(&program) {
                        Ok(_) => {
                            println!("Type checking passed.");

                            let script_dir = args
                                .get(1)
                                .map(|path| Path::new(path).parent().unwrap_or(Path::new(".")))
                                .unwrap_or_else(|| Path::new("."));

                            println!("Script directory: {:?}", script_dir);
                            let timeout_secs = config::load_timeout(script_dir);
                            println!("Timeout seconds: {}", timeout_secs);
                            let mut interpreter = Interpreter::with_timeout(timeout_secs);
                            match interpreter.interpret(&program) {
                                Ok(result) => println!(
                                    "Execution completed successfully. Result: {:?}",
                                    result
                                ),
                                Err(errors) => {
                                    eprintln!("Runtime errors:");
                                    for error in errors {
                                        eprintln!("{}", error);
                                    }
                                }
                            }
                        }
                        Err(errors) => {
                            eprintln!("Type errors:");
                            for error in errors {
                                eprintln!("{}", error);
                            }
                        }
                    }
                }
                Err(errors) => {
                    eprintln!("Semantic errors:");
                    for error in errors {
                        eprintln!("{}", error);
                    }
                }
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("Error: {}", error);
            }
        }
    }

    Ok(())
}
