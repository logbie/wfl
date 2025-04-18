use crate::interpreter::Interpreter;
use crate::lexer::{lex_wfl_with_positions, token::TokenWithPosition};
use crate::parser::{
    Parser,
    ast::{Program, Statement},
};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RustylineResult};
use std::io::{self, Write};

pub struct ReplState {
    interpreter: Interpreter,
    input_buffer: String,
    in_multiline: bool,
    history: Vec<String>,
}

impl Default for ReplState {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplState {
    pub fn new() -> Self {
        let interpreter = Interpreter::new();

        ReplState {
            interpreter,
            input_buffer: String::new(),
            in_multiline: false,
            history: Vec::new(),
        }
    }

    pub fn process_line(&mut self, line: &str) -> Result<Option<String>, String> {
        if line.trim().starts_with('.') {
            return self.handle_repl_command(line.trim());
        }

        if !self.input_buffer.is_empty() {
            self.input_buffer.push('\n');
        }
        self.input_buffer.push_str(line);

        let input = self.input_buffer.clone();
        let tokens = lex_wfl_with_positions(&input);

        if self.is_input_incomplete(&tokens) {
            self.in_multiline = true;
            return Ok(None); // Need more input
        }

        self.in_multiline = false;

        if !input.trim().is_empty() {
            self.history.push(input.clone());
        }

        let result = self.process_complete_input(&input);

        self.input_buffer.clear();

        result
    }

    fn handle_repl_command(&mut self, command: &str) -> Result<Option<String>, String> {
        match command {
            ".exit" => std::process::exit(0),
            ".help" => Ok(Some(
                "WFL REPL Commands:\n\
                 .exit    - Exit the REPL\n\
                 .help    - Show this help message\n\
                 .history - Show command history\n\
                 .clear   - Clear the screen\n"
                    .to_string(),
            )),
            ".history" => {
                let mut result = String::new();
                for (i, cmd) in self.history.iter().enumerate() {
                    result.push_str(&format!("{}: {}\n", i + 1, cmd));
                }
                Ok(Some(result))
            }
            ".clear" => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();
                Ok(None)
            }
            _ => Ok(Some(format!("Unknown command: {}", command))),
        }
    }

    fn is_input_incomplete(&self, tokens: &[TokenWithPosition]) -> bool {
        if tokens.is_empty() {
            return false;
        }

        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Err(errors) => errors.iter().any(|e| {
                e.message.contains("Unexpected end of input")
                    || e.message.contains("expected")
                    || e.message.contains("Expected")
            }),
            Ok(_) => false, // Successfully parsed, input is complete
        }
    }

    fn process_complete_input(&mut self, input: &str) -> Result<Option<String>, String> {
        let tokens = lex_wfl_with_positions(input);

        let mut parser = Parser::new(&tokens);
        let program = match parser.parse() {
            Ok(prog) => prog,
            Err(errors) => {
                let error_messages: Vec<String> = errors
                    .iter()
                    .map(|e| {
                        format!(
                            "Parse error at line {}, column {}: {}",
                            e.line, e.column, e.message
                        )
                    })
                    .collect();
                return Ok(Some(error_messages.join("\n")));
            }
        };

        if program.statements.is_empty() {
            return Ok(None);
        }

        let mut result_output = None;

        if let Some(last_stmt) = program.statements.last() {
            match last_stmt {
                Statement::ExpressionStatement { .. } => {
                    let expr_program = Program {
                        statements: vec![last_stmt.clone()],
                    };

                    match self.interpreter.interpret(&expr_program) {
                        Ok(value) => {
                            result_output = Some(format!("{:?}", value));
                        }
                        Err(errors) => {
                            result_output = Some(format!("Runtime error: {:?}", errors));
                        }
                    }
                }
                _ => match self.interpreter.interpret(&program) {
                    Ok(_) => {}
                    Err(errors) => {
                        result_output = Some(format!("Runtime error: {:?}", errors));
                    }
                },
            }
        } else {
            match self.interpreter.interpret(&program) {
                Ok(_) => {}
                Err(errors) => {
                    result_output = Some(format!("Runtime error: {:?}", errors));
                }
            }
        }

        Ok(result_output)
    }
}

pub fn run_repl() -> RustylineResult<()> {
    let mut repl_state = ReplState::new();
    let mut rl = DefaultEditor::new()?;

    println!("WFL REPL - Type .help for commands or .exit to quit");

    loop {
        let prompt = if repl_state.in_multiline {
            "... "
        } else {
            "wfl> "
        };
        match rl.readline(prompt) {
            Ok(line) => {
                rl.add_history_entry(&line)?;

                match repl_state.process_line(&line) {
                    Ok(Some(output)) => println!("{}", output),
                    Ok(None) => {} // No output needed
                    Err(error) => println!("Error: {}", error),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
