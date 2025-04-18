use std::fmt;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl RuntimeError {
    pub fn new(message: String, line: usize, column: usize) -> Self {
        RuntimeError {
            message,
            line,
            column,
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Runtime error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for RuntimeError {}
