use crate::config::WflConfig;
use crate::interpreter::environment::Environment;
use crate::interpreter::error::{ErrorKind, RuntimeError};
use crate::interpreter::value::Value;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{self};
use std::fs::File;
use std::io::{BufWriter, Write as IoWrite};
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub struct SafeDebug<'a> {
    value: &'a Value,
    depth: usize,
    seen: Rc<RefCell<HashSet<*const ()>>>,
}

impl<'a> SafeDebug<'a> {
    pub fn new(value: &'a Value, depth: usize) -> Self {
        Self {
            value,
            depth,
            seen: Rc::new(RefCell::new(HashSet::new())),
        }
    }
}

impl fmt::Debug for SafeDebug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.depth == 0 {
            return write!(f, "...");
        }

        match self.value {
            Value::List(list) => {
                let ptr = Rc::as_ptr(list) as *const ();

                if !self.seen.borrow_mut().insert(ptr) {
                    return write!(f, "[<cycle>]");
                }

                let borrowed = list.borrow();
                write!(f, "[")?;

                for (i, v) in borrowed.iter().take(16).enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    SafeDebug {
                        value: v,
                        depth: self.depth - 1,
                        seen: Rc::clone(&self.seen),
                    }
                    .fmt(f)?;
                }

                if borrowed.len() > 16 {
                    write!(f, ", ... ({} more items)", borrowed.len() - 16)?;
                }

                self.seen.borrow_mut().remove(&ptr);
                write!(f, "]")
            }
            Value::Object(obj) => {
                let ptr = Rc::as_ptr(obj) as *const ();

                if !self.seen.borrow_mut().insert(ptr) {
                    return write!(f, "{{<cycle>}}");
                }

                let borrowed = obj.borrow();
                write!(f, "{{")?;

                for (i, (k, v)) in borrowed.iter().take(16).enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: ", k)?;
                    SafeDebug {
                        value: v,
                        depth: self.depth - 1,
                        seen: Rc::clone(&self.seen),
                    }
                    .fmt(f)?;
                }

                if borrowed.len() > 16 {
                    write!(f, ", ... ({} more items)", borrowed.len() - 16)?;
                }

                self.seen.borrow_mut().remove(&ptr);
                write!(f, "}}")
            }
            v => fmt::Debug::fmt(v, f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub func_name: String,
    pub call_line: usize,
    pub call_col: usize,
    pub locals: Option<HashMap<String, Captured>>,
}

#[derive(Debug, Clone)]
pub enum Captured {
    Primitive(Value),
    Truncated(String),
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
        let env_ref = env.borrow();
        let env_values = &env_ref.values;
        
        let mut captured_locals = HashMap::with_capacity(env_values.len());
        let mut total_bytes = 0;
        const MAX_CAPTURE_BYTES: usize = 32 * 1024; // 32 KiB limit per frame

        for (name, value) in env_values {
            let captured = match value {
                Value::Number(_) | Value::Bool(_) | Value::Null => Captured::Primitive(value),

                Value::Text(s) if s.len() < 256 => Captured::Primitive(value),

                Value::List(rc) => {
                    let approx_size = std::mem::size_of_val(&value) + rc.borrow().len() * 8;
                    total_bytes += approx_size;

                    if total_bytes > MAX_CAPTURE_BYTES {
                        Captured::Truncated(format!(
                            "<list with {} items truncated...>",
                            rc.borrow().len()
                        ))
                    } else {
                        Captured::Truncated(format!("<list with {} items>", rc.borrow().len()))
                    }
                }
                Value::Object(rc) => {
                    let approx_size = std::mem::size_of_val(&value) + rc.borrow().len() * 16;
                    total_bytes += approx_size;

                    if total_bytes > MAX_CAPTURE_BYTES {
                        Captured::Truncated(format!(
                            "<object with {} fields truncated...>",
                            rc.borrow().len()
                        ))
                    } else {
                        Captured::Truncated(format!("<object with {} fields>", rc.borrow().len()))
                    }
                }
                Value::Function(_rc) => {
                    total_bytes += std::mem::size_of_val(&value) + 64; // Rough estimate for function

                    if total_bytes > MAX_CAPTURE_BYTES {
                        Captured::Truncated(format!("<{} truncated...>", value.type_name()))
                    } else {
                        Captured::Truncated(format!("<{}>", value.type_name()))
                    }
                }
                Value::NativeFunction(_) => {
                    total_bytes += std::mem::size_of_val(&value) + 64; // Rough estimate for function

                    if total_bytes > MAX_CAPTURE_BYTES {
                        Captured::Truncated(format!("<{} truncated...>", value.type_name()))
                    } else {
                        Captured::Truncated(format!("<{}>", value.type_name()))
                    }
                }
                Value::Future(_) => {
                    total_bytes += std::mem::size_of_val(&value) + 32; // Rough estimate for future

                    if total_bytes > MAX_CAPTURE_BYTES {
                        Captured::Truncated(format!("<future truncated...>"))
                    } else {
                        Captured::Truncated(format!("<future>"))
                    }
                }
                Value::Text(s) => {
                    let approx_size = std::mem::size_of_val(&value) + s.len();
                    total_bytes += approx_size;

                    if total_bytes > MAX_CAPTURE_BYTES {
                        Captured::Truncated(format!("<text of length {} truncated...>", s.len()))
                    } else {
                        if s.len() > 256 {
                            let truncated =
                                format!("{}... (truncated, {} chars total)", &s[..250], s.len());
                            Captured::Truncated(truncated)
                        } else {
                            Captured::Primitive(value)
                        }
                    }
                }
            };

            captured_locals.insert(name, captured);
        }

        self.locals = Some(captured_locals);
    }
}

pub fn create_report(
    error: &RuntimeError,
    call_stack: &[CallFrame],
    source: &str,
    script_path: &str,
    config: &WflConfig,
) -> Result<PathBuf, std::io::Error> {
    if matches!(error.kind, ErrorKind::OutOfMemory) && config.debug_report_enabled {
        log::warn!("Skipping debug report for OutOfMemory error to avoid recursive OOM");
        return Ok(PathBuf::new());
    }

    let debug_file_path = generate_debug_filename(script_path);

    let file = File::create(&debug_file_path)?;
    let mut writer = BufWriter::new(file);

    generate_report_content(error, call_stack, source, script_path, config, &mut writer)?;

    writer.flush()?;

    Ok(debug_file_path)
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
    config: &WflConfig,
    writer: &mut impl IoWrite,
) -> Result<(), std::io::Error> {
    writeln!(writer, "=== WFL Debug Report ===")?;
    writeln!(writer, "Script: {}", script_path)?;
    writeln!(
        writer,
        "Time: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;

    writeln!(writer, "\n=== Error Summary ===")?;
    writeln!(
        writer,
        "Runtime error at line {}, column {}: {}",
        error.line, error.column, error.message
    )?;

    writeln!(writer, "\n=== Stack Trace ===")?;
    if call_stack.is_empty() {
        writeln!(
            writer,
            "In main script at line {}, column {}",
            error.line, error.column
        )?;
    } else {
        let start_idx = if config.debug_full_report {
            0
        } else {
            call_stack.len().saturating_sub(5)
        };

        if start_idx > 0 && !config.debug_full_report {
            writeln!(
                writer,
                "... ({} earlier frames truncated, use debug_full_report=true to see all)",
                start_idx
            )?;
        }

        for (i, frame) in call_stack.iter().enumerate().skip(start_idx).rev() {
            if i == call_stack.len() - 1 {
                writeln!(
                    writer,
                    "At action \"{}\" (line {}, column {})",
                    frame.func_name, error.line, error.column
                )?;
            } else {
                writeln!(
                    writer,
                    "Called from action \"{}\" (line {}, column {})",
                    frame.func_name, frame.call_line, frame.call_col
                )?;
            }
        }
    }

    writeln!(writer, "\n=== Source Code ===")?;
    add_source_snippet(writer, source, error.line)?;

    if config.debug_full_report && !call_stack.is_empty() {
        let frame = &call_stack[call_stack.len() - 1];
        writeln!(writer, "\n=== Action Body ===")?;
        extract_function_body(writer, source, &frame.func_name)?;
    }

    writeln!(writer, "\n=== Local Variables ===")?;
    if let Some(frame) = call_stack.last() {
        if let Some(locals) = &frame.locals {
            let locals_iter = if config.debug_full_report {
                locals.iter().collect::<Vec<_>>()
            } else {
                locals.iter().take(10).collect::<Vec<_>>()
            };

            for (name, captured) in locals_iter {
                write!(writer, "{} = ", name)?;
                match captured {
                    Captured::Primitive(value) => {
                        writeln!(writer, "{:?}", SafeDebug::new(value, 4))?;
                    }
                    Captured::Truncated(summary) => {
                        writeln!(writer, "{}", summary)?;
                    }
                }
            }

            if !config.debug_full_report && locals.len() > 10 {
                writeln!(
                    writer,
                    "... ({} more variables truncated, use debug_full_report=true to see all)",
                    locals.len() - 10
                )?;
            }
        } else {
            writeln!(writer, "(No local variables captured)")?;
        }
    } else {
        writeln!(writer, "(No local variables in global scope)")?;
    }

    Ok(())
}

fn add_source_snippet(
    writer: &mut impl IoWrite,
    source: &str,
    error_line: usize,
) -> Result<(), std::io::Error> {
    let lines: Vec<&str> = source.lines().collect();
    let err_line_index = error_line.saturating_sub(1); // 0-based index

    let start_line = err_line_index.saturating_sub(2);
    let end_line = std::cmp::min(err_line_index + 2, lines.len().saturating_sub(1));

    for (i, line) in lines
        .iter()
        .enumerate()
        .skip(start_line)
        .take(end_line - start_line + 1)
    {
        let line_marker = if i == err_line_index { ">> " } else { "   " };
        writeln!(writer, "{}{}: {}", line_marker, i + 1, line)?;
    }

    Ok(())
}

fn extract_function_body(
    writer: &mut impl IoWrite,
    source: &str,
    func_name: &str,
) -> Result<(), std::io::Error> {
    let lines: Vec<&str> = source.lines().collect();

    let mut start_line = None;
    let mut end_line = None;

    for (i, line) in lines.iter().enumerate() {
        if line.contains(&format!("define action called {}", func_name))
            || line.contains(&format!("action called {}", func_name))
        {
            start_line = Some(i);
        } else if start_line.is_some() && line.contains("end action") {
            end_line = Some(i);
            break;
        }
    }

    if let (Some(start), Some(end)) = (start_line, end_line) {
        for (i, line) in lines.iter().enumerate().skip(start).take(end - start + 1) {
            writeln!(writer, "{}: {}", i + 1, line)?;
        }
    } else {
        writeln!(writer, "(Could not locate action body)")?;
    }

    Ok(())
}

#[deprecated(
    since = "0.2.0",
    note = "Use generate_report_content with a writer instead"
)]
fn write_report_to_file(file_path: &Path, content: &str) -> Result<(), std::io::Error> {
    let mut file = BufWriter::new(File::create(file_path)?);
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::environment::Environment;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_debug_report_generation_simple() {
        let error = RuntimeError::new("Test error".to_string(), 1, 1);
        let mut call_frame = CallFrame::new("test_function".to_string(), 1, 1);

        let env = Environment::new_global();

        {
            let mut env_mut = env.borrow_mut();
            env_mut.define("x", Value::Number(42.0));
            env_mut.define("y", Value::Text("hello".into()));
        }

        call_frame.capture_locals(&env);

        let call_stack = vec![call_frame];

        let script_content = "store x as 42\nstore y as \"hello\"";

        let temp_dir = tempdir().unwrap();
        let script_path = temp_dir.path().join("test_script.wfl");
        fs::write(&script_path, script_content).unwrap();

        let config = WflConfig::default();

        let report_path = create_report(
            &error,
            &call_stack,
            script_content,
            script_path.to_str().unwrap(),
            &config,
        )
        .unwrap();

        assert!(report_path.exists());

        let report_content = fs::read_to_string(report_path).unwrap();

        assert!(report_content.contains("=== WFL Debug Report ==="));
        assert!(report_content.contains("=== Error Summary ==="));
        assert!(report_content.contains("Test error"));
        assert!(report_content.contains("=== Stack Trace ==="));
        assert!(report_content.contains("test_function"));
        assert!(report_content.contains("=== Source Code ==="));
        assert!(report_content.contains("=== Local Variables ==="));
        assert!(report_content.contains("x = 42"));
        assert!(report_content.contains("y = \"hello\""));
    }

    #[test]
    fn test_safe_debug_with_cyclic_list() {
        use crate::interpreter::value::Value;
        use std::cell::RefCell;
        use std::rc::Rc;

        let list = Rc::new(RefCell::new(Vec::<Value>::new()));
        let list_value = Value::List(Rc::clone(&list));

        list.borrow_mut().push(list_value.clone());

        let safe_debug = SafeDebug::new(&list_value, 4);
        let debug_output = format!("{:?}", safe_debug);

        assert!(debug_output.contains("<cycle>"));

        assert!(
            debug_output.len() < 1000,
            "Debug output size: {} bytes exceeds 1000 byte limit",
            debug_output.len()
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_report_failure_message() {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;
        use tempfile::tempdir;

        let error = RuntimeError::new("Test error".to_string(), 1, 1);
        let call_frame = CallFrame::new("test_function".to_string(), 1, 1);
        let call_stack = vec![call_frame];
        let script_content = "store x as 42";
        let config = WflConfig::default();

        let temp_dir = tempdir().unwrap();
        let script_path = temp_dir.path().join("test_script.wfl");
        fs::write(&script_path, script_content).unwrap();

        let mut perms = temp_dir.path().metadata().unwrap().permissions();
        perms.set_mode(0o444); // read-only
        fs::set_permissions(temp_dir.path(), perms).unwrap();

        let result = create_report(
            &error,
            &call_stack,
            script_content,
            script_path.to_str().unwrap(),
            &config,
        );

        assert!(result.is_err());

        let mut perms = temp_dir.path().metadata().unwrap().permissions();
        perms.set_mode(0o755); // rwx for owner
        fs::set_permissions(temp_dir.path(), perms).unwrap();
    }

    #[test]
    #[cfg(windows)]
    #[ignore]
    fn test_report_failure_message_windows() {
        println!("Skipping write permission test on Windows");
    }
}
