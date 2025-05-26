use super::*;
use crate::lexer::lex_wfl_with_positions;
use crate::parser::Parser;
use crate::diagnostics::{WflDiagnostic, Severity};

#[test]
fn test_unused_variable_detection() {
    let input = "store x as 10\nstore y as 20\ndisplay x";
    let tokens = lex_wfl_with_positions(input);
    let program = Parser::new(&tokens).parse().unwrap();
    
    let mut analyzer = Analyzer::new();
    let diagnostics = analyzer.check_unused_variables(&program, 0);
    
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].message.contains("y"));
    assert_eq!(diagnostics[0].code, "ANALYZE-UNUSED");
    assert_eq!(diagnostics[0].severity, Severity::Warning);
}

#[test]
fn test_unreachable_code_detection() {
    let input = "define action called test:\n  give back 10\n  display \"This is unreachable\"\nend action";
    let tokens = lex_wfl_with_positions(input);
    let program = Parser::new(&tokens).parse().unwrap();
    
    let mut analyzer = Analyzer::new();
    let diagnostics = analyzer.check_unreachable_code(&program, 0);
    
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].message.contains("Unreachable"));
    assert_eq!(diagnostics[0].code, "ANALYZE-UNREACHABLE");
}

#[test]
fn test_shadowing_detection() {
    let input = "store x as 10\ndefine action called test:\n  store x as 20\n  display x\nend action";
    let tokens = lex_wfl_with_positions(input);
    let program = Parser::new(&tokens).parse().unwrap();
    
    let mut analyzer = Analyzer::new();
    let diagnostics = analyzer.check_shadowing(&program, 0);
    
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].message.contains("shadows"));
    assert_eq!(diagnostics[0].code, "ANALYZE-SHADOW");
}

#[test]
fn test_inconsistent_returns() {
    let input = "define action called test returns number:\n  if x > 0 then\n    give back 10\n  end\nend action";
    let tokens = lex_wfl_with_positions(input);
    let program = Parser::new(&tokens).parse().unwrap();
    
    let mut analyzer = Analyzer::new();
    let diagnostics = analyzer.check_inconsistent_returns(&program, 0);
    
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].message.contains("inconsistent return"));
    assert_eq!(diagnostics[0].code, "ANALYZE-RETURN");
}

#[test]
fn test_static_analyzer_integration() {
    let input = "store x as 10\nstore unused as 20\ndefine action called test returns number:\n  if x > 0 then\n    give back 10\n  end\nend action";
    let tokens = lex_wfl_with_positions(input);
    let program = Parser::new(&tokens).parse().unwrap();
    
    let mut analyzer = Analyzer::new();
    let diagnostics = analyzer.analyze_static(&program, 0);
    
    assert!(diagnostics.len() >= 2);
    assert!(diagnostics.iter().any(|d| d.code == "ANALYZE-UNUSED"));
    assert!(diagnostics.iter().any(|d| d.code == "ANALYZE-RETURN"));
}

#[test]
fn test_wait_for_variable_definition() {
    let input = "wait for open file \"test.txt\" as file1 and read content into currentLog\ndisplay currentLog";
    let tokens = lex_wfl_with_positions(input);
    let program = Parser::new(&tokens).parse().unwrap();
    
    let mut analyzer = Analyzer::new();
    analyzer.analyze(&program);
    
    assert_eq!(analyzer.errors.len(), 0);
    
    assert!(analyzer.current_scope.resolve("currentLog").is_some());
}
