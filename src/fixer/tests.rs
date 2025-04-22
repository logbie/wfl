use super::*;
use crate::lexer::lex_wfl_with_positions;
use crate::parser::Parser;

#[test]
fn test_fix_variable_naming() {
    let input = "store Counter as 5";
    let tokens = lex_wfl_with_positions(input);
    let program = Parser::new(&tokens).parse().unwrap();

    let config = crate::config::WflConfig::default();
    let interpreter = std::rc::Rc::new(crate::interpreter::Interpreter::with_config(&config));
    let fixer = CodeFixer::with_interpreter(interpreter);
    let (fixed_code, summary) = fixer.fix(&program, input);

    assert_eq!(fixed_code.trim(), "store counter as 5");
    assert_eq!(summary.vars_renamed, 1);
}

#[test]
#[ignore = "Temporarily disabled due to memory issues"]
fn test_fix_indentation() {
    let input = "define action called test:\ndisplay \"Hello\"\nend action";
    let tokens = lex_wfl_with_positions(input);
    let program = Parser::new(&tokens).parse().unwrap();

    let config = crate::config::WflConfig::default();
    let interpreter = std::rc::Rc::new(crate::interpreter::Interpreter::with_config(&config));
    let fixer = CodeFixer::with_interpreter(interpreter);
    let (_fixed_code, summary) = fixer.fix(&program, input);

    assert!(summary.lines_reformatted > 0);
}

#[test]
fn test_idempotence() {
    let input = "store counter as 5";
    let tokens = lex_wfl_with_positions(input);
    let program = Parser::new(&tokens).parse().unwrap();

    let config = crate::config::WflConfig::default();
    let interpreter = std::rc::Rc::new(crate::interpreter::Interpreter::with_config(&config));
    let fixer = CodeFixer::with_interpreter(interpreter);
    let (fixed_code, _) = fixer.fix(&program, input);

    let tokens2 = lex_wfl_with_positions(&fixed_code);
    let program2 = Parser::new(&tokens2).parse().unwrap();
    let (fixed_code2, summary2) = fixer.fix(&program2, &fixed_code);

    assert_eq!(fixed_code.trim(), fixed_code2.trim());
    assert_eq!(summary2.vars_renamed, 0);
}
