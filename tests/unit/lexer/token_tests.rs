#[cfg(test)]
mod lexer_token_tests {
    // Importing required modules
    use crate::lexer::token::{Token, TokenType};
    use crate::lexer::Lexer;
    
    #[test]
    fn test_keywords() {
        // Test recognizing all keywords
        let input = "define container action if else while end check and or not true false";
        let mut lexer = Lexer::new(&input);
        
        // These assertions need to be updated based on actual lexer implementation
        assert_eq!(lexer.next_token().token_type, TokenType::Define);
        assert_eq!(lexer.next_token().token_type, TokenType::Container);
        assert_eq!(lexer.next_token().token_type, TokenType::Action);
        assert_eq!(lexer.next_token().token_type, TokenType::If);
        assert_eq!(lexer.next_token().token_type, TokenType::Else);
        assert_eq!(lexer.next_token().token_type, TokenType::While);
        assert_eq!(lexer.next_token().token_type, TokenType::End);
        assert_eq!(lexer.next_token().token_type, TokenType::Check);
        assert_eq!(lexer.next_token().token_type, TokenType::And);
        assert_eq!(lexer.next_token().token_type, TokenType::Or);
        assert_eq!(lexer.next_token().token_type, TokenType::Not);
        assert_eq!(lexer.next_token().token_type, TokenType::True);
        assert_eq!(lexer.next_token().token_type, TokenType::False);
    }
    
    #[test]
    fn test_identifiers() {
        // Test recognizing identifiers
        let input = "variable x y123 _test test_var";
        let mut lexer = Lexer::new(&input);
        
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
    }
    
    #[test]
    fn test_numbers() {
        // Test recognizing number literals
        let input = "123 45.67 0 0.5";
        let mut lexer = Lexer::new(&input);
        
        assert_eq!(lexer.next_token().token_type, TokenType::Number);
        assert_eq!(lexer.next_token().token_type, TokenType::Number);
        assert_eq!(lexer.next_token().token_type, TokenType::Number);
        assert_eq!(lexer.next_token().token_type, TokenType::Number);
    }
    
    #[test]
    fn test_strings() {
        // Test recognizing string literals
        let input = r#""hello" "world" "123" """#;
        let mut lexer = Lexer::new(&input);
        
        assert_eq!(lexer.next_token().token_type, TokenType::String);
        assert_eq!(lexer.next_token().token_type, TokenType::String);
        assert_eq!(lexer.next_token().token_type, TokenType::String);
        assert_eq!(lexer.next_token().token_type, TokenType::String);
    }
    
    #[test]
    fn test_operators() {
        // Test recognizing operators
        let input = "+ - * / = == != > < >= <= . :";
        let mut lexer = Lexer::new(&input);
        
        assert_eq!(lexer.next_token().token_type, TokenType::Plus);
        assert_eq!(lexer.next_token().token_type, TokenType::Minus);
        assert_eq!(lexer.next_token().token_type, TokenType::Asterisk);
        assert_eq!(lexer.next_token().token_type, TokenType::Slash);
        assert_eq!(lexer.next_token().token_type, TokenType::Assign);
        assert_eq!(lexer.next_token().token_type, TokenType::Equals);
        assert_eq!(lexer.next_token().token_type, TokenType::NotEquals);
        assert_eq!(lexer.next_token().token_type, TokenType::GreaterThan);
        assert_eq!(lexer.next_token().token_type, TokenType::LessThan);
        assert_eq!(lexer.next_token().token_type, TokenType::GreaterEquals);
        assert_eq!(lexer.next_token().token_type, TokenType::LessEquals);
        assert_eq!(lexer.next_token().token_type, TokenType::Dot);
        assert_eq!(lexer.next_token().token_type, TokenType::Colon);
    }
    
    #[test]
    fn test_delimiters() {
        // Test recognizing delimiters
        let input = "( ) [ ] { }";
        let mut lexer = Lexer::new(&input);
        
        assert_eq!(lexer.next_token().token_type, TokenType::LeftParen);
        assert_eq!(lexer.next_token().token_type, TokenType::RightParen);
        assert_eq!(lexer.next_token().token_type, TokenType::LeftBracket);
        assert_eq!(lexer.next_token().token_type, TokenType::RightBracket);
        assert_eq!(lexer.next_token().token_type, TokenType::LeftBrace);
        assert_eq!(lexer.next_token().token_type, TokenType::RightBrace);
    }
    
    #[test]
    fn test_comments() {
        // Test handling comments
        let input = "x # This is a comment\ny";
        let mut lexer = Lexer::new(&input);
        
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        // Comment should be skipped
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
    }
    
    #[test]
    fn test_complex_input() {
        // Test a more complex input combining multiple token types
        let input = r#"
        define variable x = 10
        if x > 5
            define variable result = "greater"
        else
            define variable result = "lesser"
        end if
        "#;
        
        let mut lexer = Lexer::new(&input);
        
        // Test the first few tokens from the complex input
        assert_eq!(lexer.next_token().token_type, TokenType::Define);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier); // variable
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier); // x
        assert_eq!(lexer.next_token().token_type, TokenType::Assign);
        assert_eq!(lexer.next_token().token_type, TokenType::Number); // 10
        // Continue with more assertions as needed
    }
    
    // Add more tests as needed
} 