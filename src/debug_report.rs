use crate::interpreter::error::RuntimeError;
use crate::interpreter::environment::Environment;
use crate::interpreter::value::Value;
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub func_name: String,
    pub call_line: usize,
    pub call_col: usize,
    pub locals: Option<HashMap<String, Value>>,
}

impl CallFrame {
    pub fn new(func_name: String, call_line: usize, call_col: usize) -> Self {
        Self {
            func_name,
            call_line,
            call_col,
            locals: None,
        }
    }

    pub fn capture_locals(&mut self, env: &Rc<RefCell<Environment>>) {
        self.locals = Some(env.borrow().values.clone());
    }
}

pub fn create_report(
    error: &RuntimeError,
    call_stack: &[CallFrame],
    source: &str,
    script_path: &str,
) -> PathBuf {
    let debug_file_path = generate_debug_filename(script_path);
    
    let report = generate_report_content(error, call_stack, source, script_path);
    
    write_report_to_file(&debug_file_path, &report);
    
    debug_file_path
}

fn generate_debug_filename(script_path: &str) -> PathBuf {
    let path = Path::new(script_path);
    let stem = path.file_stem().unwrap_or_default();
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    
    parent.join(format!("{}_debug.txt", stem.to_string_lossy()))
}

fn generate_report_content(
    error: &RuntimeError,
    call_stack: &[CallFrame],
    source: &str,
    script_path: &str,
) -> String {
    let mut report = String::new();
    
    writeln!(&mut report, "=== WFL Debug Report ===").unwrap();
    writeln!(&mut report, "Script: {}", script_path).unwrap();
    writeln!(&mut report, "Time: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")).unwrap();
    writeln!(&mut report, "\n=== Error Summary ===").unwrap();
    writeln!(&mut report, "Runtime error at line {}, column {}: {}", 
        error.line, error.column, error.message).unwrap();
    
    writeln!(&mut report, "\n=== Stack Trace ===").unwrap();
    if call_stack.is_empty() {
        writeln!(&mut report, "In main script at line {}, column {}", error.line, error.column).unwrap();
    } else {
        for (i, frame) in call_stack.iter().enumerate().rev() {
            if i == call_stack.len() - 1 {
                writeln!(&mut report, "At action \"{}\" (line {}, column {})", 
                    frame.func_name, error.line, error.column).unwrap();
            } else {
                writeln!(&mut report, "Called from action \"{}\" (line {}, column {})", 
                    frame.func_name, frame.call_line, frame.call_col).unwrap();
            }
        }
    }
    
    writeln!(&mut report, "\n=== Source Code ===").unwrap();
    add_source_snippet(&mut report, source, error.line);
    
    if !call_stack.is_empty() {
        let frame = &call_stack[call_stack.len() - 1];
        writeln!(&mut report, "\n=== Action Body ===").unwrap();
        
        extract_function_body(&mut report, source, &frame.func_name);
    }
    
    writeln!(&mut report, "\n=== Local Variables ===").unwrap();
    if let Some(frame) = call_stack.last() {
        if let Some(locals) = &frame.locals {
            for (name, value) in locals {
                writeln!(&mut report, "{} = {:?}", name, value).unwrap();
            }
        } else {
            writeln!(&mut report, "(No local variables captured)").unwrap();
        }
    } else {
        writeln!(&mut report, "(No local variables in global scope)").unwrap();
    }
    
    report
}

fn add_source_snippet(report: &mut String, source: &str, error_line: usize) {
    let lines: Vec<&str> = source.lines().collect();
    let err_line_index = error_line.saturating_sub(1); // 0-based index
    
    let start_line = err_line_index.saturating_sub(2);
    let end_line = std::cmp::min(err_line_index + 2, lines.len().saturating_sub(1));
    
    for i in start_line..=end_line {
        if i < lines.len() {
            let line_marker = if i == err_line_index { ">> " } else { "   " };
            writeln!(report, "{}{}: {}", line_marker, i + 1, lines[i]).unwrap();
        }
    }
}

fn extract_function_body(report: &mut String, source: &str, func_name: &str) {
    let lines: Vec<&str> = source.lines().collect();
    
    let mut start_line = None;
    let mut end_line = None;
    
    for (i, line) in lines.iter().enumerate() {
        if line.contains(&format!("define action called {}", func_name)) ||
           line.contains(&format!("action called {}", func_name)) {
            start_line = Some(i);
        } else if start_line.is_some() && line.contains("end action") {
            end_line = Some(i);
            break;
        }
    }
    
    if let (Some(start), Some(end)) = (start_line, end_line) {
        for i in start..=end {
            writeln!(report, "{}: {}", i + 1, lines[i]).unwrap();
        }
    } else {
        writeln!(report, "(Could not locate action body)").unwrap();
    }
}

fn write_report_to_file(file_path: &Path, content: &str) {
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            log::error!("Failed to create debug report file: {}", e);
            return;
        }
    };
    
    if let Err(e) = file.write_all(content.as_bytes()) {
        log::error!("Failed to write debug report: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::{Interpreter, error::RuntimeError};
    use crate::lexer::lex_wfl_with_positions;
    use crate::parser::Parser;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_debug_report_generation() {
        let temp_dir = tempdir().unwrap();
        let script_path = temp_dir.path().join("test_script.wfl");
        
        let script_content = r#"
        define action called divide:
            store x as 10
            store y as 0
            store result as x divided by y
            give back result
        end action

        divide
        "#;
        
        fs::write(&script_path, script_content).unwrap();
        
        let tokens = lex_wfl_with_positions(script_content);
        let mut parser = Parser::new(&tokens);
        let program = parser.parse().unwrap();
        
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program).await;
        
        assert!(result.is_err());
        let errors = result.err().unwrap();
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Division by zero"));
        
        let call_stack = interpreter.get_call_stack();
        let report_path = create_report(
            &errors[0],
            &call_stack,
            script_content,
            script_path.to_str().unwrap(),
        );
        
        assert!(report_path.exists());
        
        let report_content = fs::read_to_string(report_path).unwrap();
        
        assert!(report_content.contains("=== WFL Debug Report ==="));
        assert!(report_content.contains("=== Error Summary ==="));
        assert!(report_content.contains("Division by zero"));
        assert!(report_content.contains("=== Stack Trace ==="));
        assert!(report_content.contains("divide"));
        assert!(report_content.contains("=== Source Code ==="));
        assert!(report_content.contains("=== Action Body ==="));
        assert!(report_content.contains("=== Local Variables ==="));
    }
}
