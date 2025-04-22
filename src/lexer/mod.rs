pub mod token;

use crate::parser::intern::intern;
use logos::Logos;
use token::{Token, TokenWithPosition};

pub fn normalize_line_endings(input: &str) -> String {
    input.replace("\r\n", "\n")
}

pub fn lex_wfl(input: &str) -> Vec<Token> {
    let input = normalize_line_endings(input);
    let mut lexer = Token::lexer(&input);
    let mut tokens = Vec::new();
    let mut current_id: Option<String> = None; // Temporary string for building multi-word identifiers

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(Token::Error) => {
                eprintln!(
                    "Lexing error at position {}: unexpected input `{}`",
                    lexer.span().start,
                    lexer.slice()
                );
            }
            Ok(Token::Identifier(word)) => {
                if let Some(ref mut id) = current_id {
                    id.push(' ');
                    id.push_str(&word);
                } else {
                    current_id = Some(word.to_string());
                }
            }
            Ok(other) => {
                if let Some(id) = current_id.take() {
                    tokens.push(Token::Identifier(intern(&id)));
                }
                tokens.push(other);
            }
            Err(_) => {
                eprintln!(
                    "Lexing error at position {}: unexpected input `{}`",
                    lexer.span().start,
                    lexer.slice()
                );
            }
        }
    }

    if let Some(id) = current_id.take() {
        tokens.push(Token::Identifier(intern(&id)));
    }
    tokens
}

pub fn lex_wfl_with_positions(input: &str) -> Vec<TokenWithPosition> {
    let input = normalize_line_endings(input);
    let mut lexer = Token::lexer(&input);
    let mut tokens = Vec::new();
    let mut current_id: Option<String> = None; // Temporary string for building multi-word identifiers// Temporary string for building multi-word identifiers
    let mut current_id_start_line = 0;
    let mut current_id_start_column = 0;
    let mut current_id_length = 0;

    let mut _line = 1;
    let mut _column = 1;
    let mut line_starts = vec![0];

    for (i, c) in input.char_indices() {
        if c == '\n' {
            _line += 1;
            _column = 1;
            line_starts.push(i + 1);
        } else {
            _column += 1;
        }
    }

    let position = |offset: usize| -> (usize, usize) {
        let line_idx = line_starts.binary_search(&offset).unwrap_or_else(|i| i - 1);
        let line = line_idx + 1;
        let column = offset - line_starts[line_idx] + 1;
        (line, column)
    };

    while let Some(token_result) = lexer.next() {
        let span = lexer.span();
        let (token_line, token_column) = position(span.start);
        let token_length = span.end - span.start;

        match token_result {
            Ok(Token::Error) => {
                eprintln!(
                    "Lexing error at position {}: unexpected input `{}`",
                    span.start,
                    lexer.slice()
                );
            }
            Ok(Token::Identifier(word)) => {
                if let Some(ref mut id) = current_id {
                    id.push(' ');
                    id.push_str(&word);
                    current_id_length = span.end - current_id_start_column;
                } else {
                    current_id = Some(word.to_string());
                    current_id_start_line = token_line;
                    current_id_start_column = span.start;
                    current_id_length = token_length;
                }
            }
            Ok(other) => {
                if let Some(id) = current_id.take() {
                    tokens.push(TokenWithPosition::new(
                        Token::Identifier(intern(&id)),
                        current_id_start_line,
                        current_id_start_column,
                        current_id_length,
                    ));
                }
                tokens.push(TokenWithPosition::new(
                    other,
                    token_line,
                    token_column,
                    token_length,
                ));
            }
            Err(_) => {
                eprintln!(
                    "Lexing error at position {}: unexpected input `{}`",
                    span.start,
                    lexer.slice()
                );
            }
        }
    }

    if let Some(id) = current_id.take() {
        tokens.push(TokenWithPosition::new(
            Token::Identifier(intern(&id)),
            current_id_start_line,
            current_id_start_column,
            current_id_length,
        ));
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_ending_normalization() {
        let input = "store x as 1\r\ndisplay x\r\n";
        let normalized = normalize_line_endings(input);
        assert!(!normalized.contains('\r'));
        assert_eq!(normalized.matches('\n').count(), 2);
    }

    #[test]
    fn test_multi_word_identifier() {
        let input = r#"
            store user name as "Alice"
            display user name with " is logged in."
        "#;
        let tokens = lex_wfl(input);
        assert_eq!(
            tokens,
            vec![
                Token::KeywordStore,
                Token::Identifier(intern("user name")),
                Token::KeywordAs,
                Token::StringLiteral("Alice".to_string()),
                Token::KeywordDisplay,
                Token::Identifier(intern("user name")),
                Token::KeywordWith,
                Token::StringLiteral(" is logged in.".to_string()),
            ]
        );
    }

    #[test]
    fn test_literals_and_comments() {
        let input = r#"
            create count as 42
            create is active as no  // boolean false
            display greeting as "Hello"
            display greeting with " world!"
            open file at "data.txt" as file handle
            display file handle
            "#;
        let tokens = lex_wfl(input);

        println!("Tokens: {:?}", tokens);

        assert!(tokens.contains(&Token::KeywordCreate));
        assert!(tokens.contains(&Token::KeywordCount)); // "count" is recognized as a keyword
        assert!(tokens.contains(&Token::KeywordAs));
        assert!(tokens.contains(&Token::IntLiteral(42)));

        assert!(tokens.contains(&Token::KeywordIs));
        assert!(tokens.contains(&Token::Identifier(intern("active"))));

        assert!(tokens.contains(&Token::StringLiteral("Hello".to_string())));
        assert!(tokens.contains(&Token::KeywordWith));
        assert!(tokens.contains(&Token::StringLiteral(" world!".to_string())));

        assert!(tokens.contains(&Token::KeywordOpen));
        assert!(tokens.contains(&Token::KeywordFile));
        assert!(tokens.contains(&Token::KeywordAt));
        assert!(tokens.contains(&Token::StringLiteral("data.txt".to_string())));
        assert!(tokens.contains(&Token::KeywordAs));
        assert!(tokens.contains(&Token::KeywordFile));
        assert!(tokens.contains(&Token::Identifier(intern("handle"))));
    }

    #[test]
    fn test_hello_world_program() {
        let input = r#"

            define action called main:
                display "Hello, World!"
            end action
        "#;
        let tokens = lex_wfl(input);
        assert_eq!(
            tokens,
            vec![
                Token::KeywordDefine,
                Token::KeywordAction,
                Token::KeywordCalled,
                Token::Identifier(intern("main")),
                Token::Colon,
                Token::KeywordDisplay,
                Token::StringLiteral("Hello, World!".to_string()),
                Token::KeywordEnd,
                Token::KeywordAction,
            ]
        );
    }

    #[test]
    fn test_conditional_statement() {
        let input = r#"
            check if user name is "Alice":
                display "Special greeting for Alice!"
            otherwise:
                display "Hello, " with user name
            end check
        "#;
        let tokens = lex_wfl(input);
        assert_eq!(
            tokens,
            vec![
                Token::KeywordCheck,
                Token::KeywordIf,
                Token::Identifier(intern("user name")),
                Token::KeywordIs,
                Token::StringLiteral("Alice".to_string()),
                Token::Colon,
                Token::KeywordDisplay,
                Token::StringLiteral("Special greeting for Alice!".to_string()),
                Token::KeywordOtherwise,
                Token::Colon,
                Token::KeywordDisplay,
                Token::StringLiteral("Hello, ".to_string()),
                Token::KeywordWith,
                Token::Identifier(intern("user name")),
                Token::KeywordEnd,
                Token::KeywordCheck,
            ]
        );
    }

    #[test]
    fn test_loop_statement() {
        let input = r#"
            count from 1 to 5:
                display "Count: " with count
            end count
        "#;
        let tokens = lex_wfl(input);
        assert_eq!(
            tokens,
            vec![
                Token::KeywordCount,
                Token::KeywordFrom,
                Token::IntLiteral(1),
                Token::KeywordTo,
                Token::IntLiteral(5),
                Token::Colon,
                Token::KeywordDisplay,
                Token::StringLiteral("Count: ".to_string()),
                Token::KeywordWith,
                Token::KeywordCount,
                Token::KeywordEnd,
                Token::KeywordCount,
            ]
        );
    }

    #[test]
    fn test_identifiers_with_underscores() {
        let input = r#"
            store user_name as "Alice"
            display user_name with " is logged in."
        "#;

        let tokens = lex_wfl(input);

        assert_eq!(
            tokens,
            vec![
                Token::KeywordStore,
                Token::Identifier(intern("user_name")),
                Token::KeywordAs,
                Token::StringLiteral("Alice".to_string()),
                Token::KeywordDisplay,
                Token::Identifier(intern("user_name")),
                Token::KeywordWith,
                Token::StringLiteral(" is logged in.".to_string()),
            ]
        );
    }
}
