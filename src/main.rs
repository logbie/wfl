use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process;
use wfl::Interpreter;
use wfl::analyzer::{Analyzer, StaticAnalyzer};
use wfl::config;
use wfl::debug_report;
use wfl::diagnostics::DiagnosticReporter;
use wfl::fixer::{CodeFixer, FixerOutputMode};
use wfl::lexer::{lex_wfl, lex_wfl_with_positions, token::Token};
use wfl::linter::Linter;
use wfl::logging;
use wfl::parser::Parser;
use wfl::repl;
use wfl::typechecker::TypeChecker;
use wfl::{error, info};

fn print_help() {
    println!("WebFirst Language (WFL) Compiler and Interpreter");
    println!();
    println!("USAGE:");
    println!("    wfl [FLAGS] [file]");
    println!();
    println!("FLAGS:");
    println!("    --help       Prints this help information");
    println!("    --lint       Run the linter on the specified file");
    println!("    --analyze    Run the static analyzer on the specified file");
    println!("    --fix        Fix code style issues in the specified file");
    println!("        --in-place    Modify the file in place");
    println!("        --diff        Show a diff of the changes");
    println!();
    println!("NOTES:");
    println!("    All runs are now type‑checked and semantically analyzed by default.");
    println!("    This ensures that scripts are validated for semantic correctness");
    println!("    and type safety before execution, preventing many common runtime errors.");
    println!();
    println!("If no file is specified, the REPL will be started.");
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        if let Err(e) = repl::run_repl().await {
            eprintln!("REPL error: {}", e);
        }
        return Ok(());
    }

    if args.len() >= 2 && args[1] == "--help" {
        print_help();
        return Ok(());
    }

    let mut lint_mode = false;
    let mut analyze_mode = false;
    let mut fix_mode = false;
    let mut fix_in_place = false;
    let mut fix_diff = false;
    let mut file_path = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--lint" => {
                if lint_mode || analyze_mode || fix_mode {
                    eprintln!("Error: --lint, --analyze, and --fix flags are mutually exclusive");
                    process::exit(2);
                }
                lint_mode = true;
                i += 1;
                if i < args.len() && !args[i].starts_with("--") {
                    file_path = args[i].clone();
                    i += 1;
                } else {
                    eprintln!("Error: --lint requires a file path");
                    process::exit(2);
                }
            }
            "--analyze" => {
                if lint_mode || analyze_mode || fix_mode {
                    eprintln!("Error: --lint, --analyze, and --fix flags are mutually exclusive");
                    process::exit(2);
                }
                analyze_mode = true;
                i += 1;
                if i < args.len() && !args[i].starts_with("--") {
                    file_path = args[i].clone();
                    i += 1;
                } else {
                    eprintln!("Error: --analyze requires a file path");
                    process::exit(2);
                }
            }
            "--fix" => {
                if lint_mode || analyze_mode || fix_mode {
                    eprintln!("Error: --lint, --analyze, and --fix flags are mutually exclusive");
                    process::exit(2);
                }
                fix_mode = true;
                i += 1;
                if i < args.len() && !args[i].starts_with("--") {
                    file_path = args[i].clone();
                    i += 1;
                } else {
                    eprintln!("Error: --fix requires a file path");
                    process::exit(2);
                }

                while i < args.len() && args[i].starts_with("--") {
                    match args[i].as_str() {
                        "--in-place" => {
                            if fix_diff {
                                eprintln!(
                                    "Error: --in-place and --diff flags are mutually exclusive"
                                );
                                process::exit(2);
                            }
                            fix_in_place = true;
                            i += 1;
                        }
                        "--diff" => {
                            if fix_in_place {
                                eprintln!(
                                    "Error: --in-place and --diff flags are mutually exclusive"
                                );
                                process::exit(2);
                            }
                            fix_diff = true;
                            i += 1;
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            _ => {
                if file_path.is_empty() {
                    file_path = args[i].clone();
                }
                i += 1;
            }
        }
    }

    if file_path.is_empty() {
        eprintln!("Error: No file path provided");
        process::exit(2);
    }

    let input = fs::read_to_string(&file_path)?;
    let script_dir = Path::new(&file_path).parent().unwrap_or(Path::new("."));
    let config = config::load_config(script_dir);

    if lint_mode {
        let tokens_with_pos = lex_wfl_with_positions(&input);
        match Parser::new(&tokens_with_pos).parse() {
            Ok(program) => {
                let mut linter = Linter::new();
                linter.load_config(script_dir);

                let (diagnostics, _success) = linter.lint(&program, &input, &file_path);

                if !diagnostics.is_empty() {
                    eprintln!("Lint warnings:");

                    let mut reporter = DiagnosticReporter::new();
                    let file_id = reporter.add_file(&file_path, &input);

                    for diagnostic in diagnostics {
                        if let Err(e) = reporter.report_diagnostic(file_id, &diagnostic) {
                            eprintln!("Error displaying diagnostic: {}", e);
                            eprintln!("{}", diagnostic.message);
                        }
                    }

                    process::exit(1);
                } else {
                    println!("No lint warnings found.");
                    process::exit(0);
                }
            }
            Err(errors) => {
                eprintln!("Parse errors:");

                let mut reporter = DiagnosticReporter::new();
                let file_id = reporter.add_file(&file_path, &input);

                for error in errors {
                    let diagnostic = reporter.convert_parse_error(file_id, &error);
                    if let Err(e) = reporter.report_diagnostic(file_id, &diagnostic) {
                        eprintln!("Error displaying diagnostic: {}", e);
                        eprintln!("Error: {}", error);
                    }
                }

                process::exit(2);
            }
        }
    } else if analyze_mode {
        let tokens_with_pos = lex_wfl_with_positions(&input);
        match Parser::new(&tokens_with_pos).parse() {
            Ok(program) => {
                let mut analyzer = Analyzer::new();

                let mut reporter = DiagnosticReporter::new();
                let file_id = reporter.add_file(&file_path, &input);
                let diagnostics = analyzer.analyze_static(&program, file_id);

                if !diagnostics.is_empty() {
                    eprintln!("Static analysis warnings:");

                    let mut reporter = DiagnosticReporter::new();
                    let file_id = reporter.add_file(&file_path, &input);

                    for diagnostic in diagnostics {
                        if let Err(e) = reporter.report_diagnostic(file_id, &diagnostic) {
                            eprintln!("Error displaying diagnostic: {}", e);
                            eprintln!("{}", diagnostic.message);
                        }
                    }

                    process::exit(1);
                } else {
                    println!("No static analysis warnings found.");
                    process::exit(0);
                }
            }
            Err(errors) => {
                eprintln!("Parse errors:");

                let mut reporter = DiagnosticReporter::new();
                let file_id = reporter.add_file(&file_path, &input);

                for error in errors {
                    let diagnostic = reporter.convert_parse_error(file_id, &error);
                    if let Err(e) = reporter.report_diagnostic(file_id, &diagnostic) {
                        eprintln!("Error displaying diagnostic: {}", e);
                        eprintln!("Error: {}", error);
                    }
                }

                process::exit(2);
            }
        }
    } else if fix_mode {
        let tokens_with_pos = lex_wfl_with_positions(&input);
        match Parser::new(&tokens_with_pos).parse() {
            Ok(_program) => {
                let mut fixer = CodeFixer::new();
                fixer.set_indent_size(config.indent_size);
                fixer.load_config(script_dir);

                let output_mode = if fix_in_place {
                    FixerOutputMode::InPlace
                } else if fix_diff {
                    FixerOutputMode::Diff
                } else {
                    FixerOutputMode::Stdout
                };

                match fixer.fix_file(Path::new(&file_path), output_mode) {
                    Ok(summary) => {
                        println!("Code fixing summary:");
                        println!("  Lines reformatted: {}", summary.lines_reformatted);
                        println!("  Variables renamed: {}", summary.vars_renamed);
                        println!("  Dead code removed: {}", summary.dead_code_removed);
                        process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Error fixing code: {}", e);
                        process::exit(1);
                    }
                }
            }
            Err(errors) => {
                eprintln!("Parse errors:");

                let mut reporter = DiagnosticReporter::new();
                let file_id = reporter.add_file(&file_path, &input);

                for error in errors {
                    let diagnostic = reporter.convert_parse_error(file_id, &error);
                    if let Err(e) = reporter.report_diagnostic(file_id, &diagnostic) {
                        eprintln!("Error displaying diagnostic: {}", e);
                        eprintln!("Error: {}", error);
                    }
                }

                process::exit(2);
            }
        }
    } else {
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
                let mut reporter = DiagnosticReporter::new();
                let file_id = reporter.add_file(&file_path, &input);
                let sema_diags = analyzer.analyze_static(&program, file_id);
                if !sema_diags.is_empty() {
                    for d in &sema_diags {
                        reporter.report_diagnostic(file_id, d)?;
                    }
                    process::exit(2);
                }
                println!("Semantic analysis passed.");

                let mut tc = TypeChecker::new();
                if let Err(errors) = tc.check_types(&program) {
                    for e in &errors {
                        eprintln!("{e}");
                    }
                    process::exit(2);
                }
                println!("Type checking passed.");

                let mut linter = Linter::new();
                let (_lint_diags, _clean) = linter.lint(&program, &input, &file_path);

                println!("Script directory: {:?}", script_dir);
                println!("Timeout seconds: {}", config.timeout_seconds);

                if config.logging_enabled {
                    let log_path = script_dir.join("wfl.log");
                    if let Err(e) = logging::init_logger(config.log_level, &log_path) {
                        eprintln!("Failed to initialize logging: {}", e);
                    } else {
                        info!("WebFirst Language started with script: {}", &file_path);
                    }
                }

                let mut interpreter = Interpreter::with_timeout(config.timeout_seconds);
                let interpret_result = interpreter.interpret(&program).await;
                match interpret_result {
                    Ok(result) => {
                        if config.logging_enabled {
                            info!("Program executed successfully");
                        }
                        println!("Execution completed successfully. Result: {:?}", result)
                    }
                    Err(errors) => {
                        if config.logging_enabled {
                            error!("Runtime errors occurred");
                        }

                        eprintln!("Runtime errors:");

                        let mut reporter = DiagnosticReporter::new();
                        let file_id = reporter.add_file(&file_path, &input);

                        if config.debug_report_enabled && !errors.is_empty() {
                            let error = &errors[0]; // Take the first error
                            let call_stack = interpreter.get_call_stack();
                            match debug_report::create_report(
                                error,
                                &call_stack,
                                &input,
                                &file_path,
                            ) {
                                Ok(report_path) => {
                                    let report_msg =
                                        format!("Debug report created: {}", report_path.display());
                                    eprintln!("{}", report_msg);

                                    if config.logging_enabled {
                                        info!("{}", report_msg);
                                    }
                                }
                                Err(_) => {
                                    eprintln!("Could not create debug report");

                                    if config.logging_enabled {
                                        error!("Could not create debug report");
                                    }
                                }
                            }
                        }

                        for error in errors {
                            let diagnostic = reporter.convert_runtime_error(file_id, &error);
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
                let file_id = reporter.add_file(&file_path, &input);

                for error in errors {
                    let diagnostic = reporter.convert_parse_error(file_id, &error);
                    if let Err(e) = reporter.report_diagnostic(file_id, &diagnostic) {
                        eprintln!("Error displaying diagnostic: {}", e);
                        eprintln!("Error: {}", error); // Fallback to simple error display
                    }
                }
            }
        }
    }

    Ok(())
}
