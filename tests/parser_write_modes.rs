use wfl::lexer::lex_wfl_with_positions;
use wfl::parser::{
    Parser,
    ast::{Statement, WriteMode},
};

#[test]
fn test_parse_write_statement() {
    let source = r#"wait for write content "test" into logHandle"#;
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::WaitForStatement { inner, .. } = &program.statements[0] {
        if let Statement::WriteFileStatement { mode, .. } = inner.as_ref() {
            assert!(matches!(mode, WriteMode::Overwrite));
        } else {
            panic!("Expected WriteFileStatement");
        }
    } else {
        panic!("Expected WaitForStatement");
    }
}

#[test]
fn test_parse_append_statement() {
    let source = r#"wait for append content "test" into logHandle"#;
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::WaitForStatement { inner, .. } = &program.statements[0] {
        if let Statement::WriteFileStatement { mode, .. } = inner.as_ref() {
            assert!(matches!(mode, WriteMode::Append));
        } else {
            panic!("Expected WriteFileStatement");
        }
    } else {
        panic!("Expected WaitForStatement");
    }
}
