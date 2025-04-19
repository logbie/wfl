
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::{SimpleFile, SimpleFiles};
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::fmt;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

impl From<Severity> for codespan_reporting::diagnostic::Severity {
    fn from(severity: Severity) -> Self {
        match severity {
            Severity::Error => Self::Error,
            Severity::Warning => Self::Warning,
            Severity::Note => Self::Note,
            Severity::Help => Self::Help,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct WflDiagnostic {
    pub severity: Severity,
    pub message: String,
    pub labels: Vec<(Span, String)>,
    pub notes: Vec<String>,
}

impl WflDiagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        WflDiagnostic {
            severity: Severity::Error,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        WflDiagnostic {
            severity: Severity::Warning,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub fn with_primary_label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push((span, message.into()));
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn to_codespan_diagnostic(&self, file_id: usize) -> Diagnostic<usize> {
        let mut diag = Diagnostic::new(self.severity.into())
            .with_message(self.message.clone());
        
        for (span, message) in &self.labels {
            diag = diag.with_labels(vec![
                Label::primary(file_id, span.start..span.end).with_message(message.clone())
            ]);
        }
        
        for note in &self.notes {
            diag = diag.with_notes(vec![note.clone()]);
        }
        
        diag
    }
}

pub struct DiagnosticReporter {
    pub files: SimpleFiles<String, String>,
}

impl Default for DiagnosticReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticReporter {
    pub fn new() -> Self {
        DiagnosticReporter {
            files: SimpleFiles::new(),
        }
    }

    pub fn add_file(&mut self, name: impl Into<String>, source: impl Into<String>) -> usize {
        self.files.add(name.into(), source.into())
    }

    pub fn report_diagnostic(&self, file_id: usize, diagnostic: &WflDiagnostic) -> io::Result<()> {
        let mut diag = Diagnostic::new(diagnostic.severity.into())
            .with_message(diagnostic.message.clone());

        for (span, message) in &diagnostic.labels {
            diag = diag.with_labels(vec![
                Label::primary(file_id, span.start..span.end).with_message(message.clone())
            ]);
        }

        for note in &diagnostic.notes {
            diag = diag.with_notes(vec![note.clone()]);
        }

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = term::Config::default();

        term::emit(&mut writer.lock(), &config, &self.files, &diag)
    }

    pub fn line_col_to_offset(&self, file_id: usize, line: usize, column: usize) -> Option<usize> {
        let line = line.saturating_sub(1);
        let column = column.saturating_sub(1);

        if let Ok(file) = self.files.get(file_id) {
            let source = file.source();
            let lines: Vec<&str> = source.lines().collect();

            if line < lines.len() {
                let line_start = source.lines().take(line).map(|l| l.len() + 1).sum::<usize>();
                if column <= lines[line].len() {
                    return Some(line_start + column);
                }
            }
        }
        None
    }

    pub fn convert_parse_error(&self, file_id: usize, error: &crate::parser::ast::ParseError) -> WflDiagnostic {
        let message = error.message.clone();
        
        let start_offset = self.line_col_to_offset(file_id, error.line, error.column).unwrap_or(0);
        let end_offset = start_offset + 1;
        
        WflDiagnostic::error(message)
            .with_primary_label(
                Span { start: start_offset, end: end_offset },
                "Error occurred here"
            )
    }

    pub fn convert_type_error(&self, file_id: usize, error: &crate::typechecker::TypeError) -> WflDiagnostic {
        let mut message = error.message.clone();
        
        if let (Some(expected), Some(found)) = (&error.expected, &error.found) {
            message = format!("{} - Expected {} but found {}", message, expected, found);
        }
        
        let start_offset = self.line_col_to_offset(file_id, error.line, error.column).unwrap_or(0);
        let end_offset = start_offset + 1;
        
        let mut diag = WflDiagnostic::error(message)
            .with_primary_label(
                Span { start: start_offset, end: end_offset },
                "Type error occurred here"
            );
        
        if message.contains("undefined") || message.contains("not defined") {
            diag = diag.with_note("Did you misspell the variable name or forget to declare it?");
        } else if let (Some(expected), Some(found)) = (&error.expected, &error.found) {
            if expected.to_string() == "Number" && found.to_string() == "Text" {
                diag = diag.with_note("Try converting the text to a number using 'convert to number'");
            } else if expected.to_string() == "Text" && found.to_string() == "Number" {
                diag = diag.with_note("Try converting the number to text using 'convert to text'");
            }
        }
        
        diag
    }

    pub fn convert_semantic_error(&self, file_id: usize, error: &crate::analyzer::SemanticError) -> WflDiagnostic {
        let message = error.message.clone();
        
        let start_offset = self.line_col_to_offset(file_id, error.line, error.column).unwrap_or(0);
        let end_offset = start_offset + 1;
        
        let mut diag = WflDiagnostic::error(message)
            .with_primary_label(
                Span { start: start_offset, end: end_offset },
                "Semantic error occurred here"
            );
        
        if error.message.contains("already defined") {
            diag = diag.with_note("Variables must have unique names within the same scope");
        } else if error.message.contains("not defined") {
            diag = diag.with_note("Did you misspell the variable name or forget to declare it?");
        }
        
        diag
    }

    pub fn convert_runtime_error(&self, file_id: usize, error: &crate::interpreter::error::RuntimeError) -> WflDiagnostic {
        let message = error.message.clone();
        
        let start_offset = self.line_col_to_offset(file_id, error.line, error.column).unwrap_or(0);
        let end_offset = start_offset + 1;
        
        let mut diag = WflDiagnostic::error(message)
            .with_primary_label(
                Span { start: start_offset, end: end_offset },
                "Runtime error occurred here"
            );
        
        if error.message.contains("division by zero") {
            diag = diag.with_note("Check your divisor to ensure it's never zero");
        } else if error.message.contains("index out of bounds") {
            diag = diag.with_note("Make sure your index is within the valid range of the list");
        } else if error.message.contains("file not found") {
            diag = diag.with_note("Verify that the file exists and the path is correct");
        }
        
        diag
    }
}

#[cfg(test)]
mod tests;
