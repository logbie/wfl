use super::*;
use crate::parser::ast::ParseError;
use crate::typechecker::TypeError;

#[test]
fn test_parse_error_conversion() {
    let mut reporter = DiagnosticReporter::new();
    let source = "store x as 42\nstore y as \"hello\"\nstore z as";
    let file_id = reporter.add_file("test.wfl", source);
    
    let error = ParseError::new(
        "Expected expression after 'as'".to_string(),
        3,
        11,
    );
    
    let diagnostic = reporter.convert_parse_error(file_id, &error);
    assert_eq!(diagnostic.severity, Severity::Error);
    assert_eq!(diagnostic.message, "Expected expression after 'as'");
    assert_eq!(diagnostic.labels.len(), 1);
}

#[test]
fn test_type_error_conversion() {
    let mut reporter = DiagnosticReporter::new();
    let source = "store x as 42\nstore y as \"hello\"\ndisplay x plus y";
    let file_id = reporter.add_file("test.wfl", source);
    
    let error = TypeError::new(
        "Cannot add number and text".to_string(),
        Some(crate::parser::ast::Type::Number),
        Some(crate::parser::ast::Type::Text),
        3,
        12,
    );
    
    let diagnostic = reporter.convert_type_error(file_id, &error);
    assert_eq!(diagnostic.severity, Severity::Error);
    assert!(diagnostic.message.contains("Cannot add number and text"));
    assert_eq!(diagnostic.labels.len(), 1);
    assert!(!diagnostic.notes.is_empty());
}

#[test]
fn test_line_col_to_offset() {
    let mut reporter = DiagnosticReporter::new();
    let source = "line 1\nline 2\nline 3";
    let file_id = reporter.add_file("test.wfl", source);
    
    assert_eq!(reporter.line_col_to_offset(file_id, 1, 1), Some(0));
    assert_eq!(reporter.line_col_to_offset(file_id, 1, 2), Some(1));
    assert_eq!(reporter.line_col_to_offset(file_id, 2, 1), Some(7));
    assert_eq!(reporter.line_col_to_offset(file_id, 3, 1), Some(14));
}
