use std::env;
use std::fs;
use std::io;
use std::path::Path;
use wfl::Interpreter;
use wfl::analyzer::Analyzer;
use wfl::config;
use wfl::debug_report;
use wfl::diagnostics::DiagnosticReporter;
use wfl::lexer::{lex_wfl, lex_wfl_with_positions, token::Token};
use wfl::logging;
use wfl::parser::Parser;
use wfl::repl;
use wfl::typechecker::TypeChecker;
use wfl::{debug, info, warn, error};

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        if let Err(e) = repl::run_repl().await {
            eprintln!("REPL error: {}", e);
        }
        return Ok(());
    }

    let input = fs::read_to_string(&args[1])?;

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
                            
                            let config = config::load_config(script_dir);
                            println!("Timeout seconds: {}", config.timeout_seconds);
                            
                            if config.logging_enabled {
                                let log_path = script_dir.join("wfl.log");
                                if let Err(e) = logging::init_logger(config.log_level, &log_path) {
                                    eprintln!("Failed to initialize logging: {}", e);
                                } else {
                                    info!("WFL started with script: {}", &args[1]);
                                }
                            }
                            
                            let mut interpreter = Interpreter::with_timeout(config.timeout_seconds);
                            let interpret_result = interpreter.interpret(&program).await;
                            match interpret_result {
                                Ok(result) => {
                                    if config.logging_enabled {
                                        info!("Program executed successfully");
                                    }
                                    println!(
                                        "Execution completed successfully. Result: {:?}",
                                        result
                                    )
                                },
                                Err(errors) => {
                                    if config.logging_enabled {
                                        error!("Runtime errors occurred");
                                    }
                                    
                                    eprintln!("Runtime errors:");

                                    let mut reporter = DiagnosticReporter::new();
                                    let file_id = reporter.add_file(&args[1], &input);
                                    
                                    if config.debug_report_enabled && !errors.is_empty() {
                                        let error = &errors[0]; // Take the first error
                                        let call_stack = interpreter.get_call_stack();
                                        let report_path = debug_report::create_report(
                                            error,
                                            &call_stack,
                                            &input,
                                            &args[1],
                                        );
                                        
                                        let report_msg = format!("Debug report created: {}", report_path.display());
                                        eprintln!("{}", report_msg);
                                        
                                        if config.logging_enabled {
                                            info!("{}", report_msg);
                                        }
                                    }

                                    for error in errors {
                                        let diagnostic =
                                            reporter.convert_runtime_error(file_id, &error);
                                        if let Err(e) =
                                            reporter.report_diagnostic(file_id, &diagnostic)
                                        {
                                            eprintln!("Error displaying diagnostic: {}", e);
                                            eprintln!("{}", error); // Fallback to simple error display
                                        }
                                    }
                                }
                            }
                        }
                        Err(errors) => {
                            eprintln!("Type errors:");

                            let mut reporter = DiagnosticReporter::new();
                            let file_id = reporter.add_file(&args[1], &input);

                            for error in errors {
                                let diagnostic = reporter.convert_type_error(file_id, &error);
                                if let Err(e) = reporter.report_diagnostic(file_id, &diagnostic) {
                                    eprintln!("Error displaying diagnostic: {}", e);
                                    eprintln!("{}", error); // Fallback to simple error display
                                }
                            }
                        }
                    }
                }
                Err(errors) => {
                    eprintln!("Semantic errors:");

                    let mut reporter = DiagnosticReporter::new();
                    let file_id = reporter.add_file(&args[1], &input);

                    for error in errors {
                        let diagnostic = reporter.convert_semantic_error(file_id, &error);
                        if let Err(e) = reporter.report_diagnostic(file_id, &diagnostic) {
                            eprintln!("Error displaying diagnostic: {}", e);
                            eprintln!("{}", error); // Fallback to simple error display
                        }
                    }
                }
            }
        }
        Err(errors) => {
            eprintln!("Parse errors:");

            let mut reporter = DiagnosticReporter::new();
            let file_id = reporter.add_file(&args[1], &input);

            for error in errors {
                let diagnostic = reporter.convert_parse_error(file_id, &error);
                if let Err(e) = reporter.report_diagnostic(file_id, &diagnostic) {
                    eprintln!("Error displaying diagnostic: {}", e);
                    eprintln!("Error: {}", error); // Fallback to simple error display
                }
            }
        }
    }

    Ok(())
}
