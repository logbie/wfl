mod token;
pub use token::{Token, TokenType};

use std::collections::HashMap;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    indent_stack: Vec<usize>,
    keywords: HashMap<String, TokenType>,
    start_position: usize,  // Track start of current token
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        // Initialize keywords
        keywords.insert("define".to_string(), TokenType::Define);
        keywords.insert("action".to_string(), TokenType::Action);
        keywords.insert("called".to_string(), TokenType::Called);
        keywords.insert("needs".to_string(), TokenType::Needs);
        keywords.insert("does".to_string(), TokenType::Does);
        keywords.insert("end".to_string(), TokenType::End);
        keywords.insert("give".to_string(), TokenType::Give);
        keywords.insert("back".to_string(), TokenType::Back);
        keywords.insert("store".to_string(), TokenType::Store);
        keywords.insert("as".to_string(), TokenType::As);
        keywords.insert("create".to_string(), TokenType::Create);
        keywords.insert("with".to_string(), TokenType::With);
        keywords.insert("check".to_string(), TokenType::Check);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("otherwise".to_string(), TokenType::Otherwise);
        keywords.insert("when".to_string(), TokenType::When);
        keywords.insert("yes".to_string(), TokenType::TruthLiteral);
        keywords.insert("no".to_string(), TokenType::TruthLiteral);
        keywords.insert("true".to_string(), TokenType::TruthLiteral);
        keywords.insert("false".to_string(), TokenType::TruthLiteral);
        keywords.insert("nothing".to_string(), TokenType::Nothing);
        keywords.insert("missing".to_string(), TokenType::Missing);
        keywords.insert("undefined".to_string(), TokenType::Undefined);
        keywords.insert("empty".to_string(), TokenType::Empty);
        
        // Access modifiers
        keywords.insert("public".to_string(), TokenType::Public);
        keywords.insert("private".to_string(), TokenType::Private);
        keywords.insert("protected".to_string(), TokenType::Protected);
        
        // Container-related
        keywords.insert("container".to_string(), TokenType::Container);
        keywords.insert("from".to_string(), TokenType::From);
        keywords.insert("interface".to_string(), TokenType::Interface);
        keywords.insert("implements".to_string(), TokenType::Implements);
        
        // Type-related
        keywords.insert("any".to_string(), TokenType::Any);
        keywords.insert("of".to_string(), TokenType::Of);
        keywords.insert("type".to_string(), TokenType::Type);
        
        // Operators
        keywords.insert("join".to_string(), TokenType::Join);
        keywords.insert("to".to_string(), TokenType::To);
        keywords.insert("into".to_string(), TokenType::Into);
        keywords.insert("at".to_string(), TokenType::At);
        keywords.insert("where".to_string(), TokenType::Where);
        keywords.insert("contains".to_string(), TokenType::Contains);
        keywords.insert("matches".to_string(), TokenType::Matches);
        keywords.insert("between".to_string(), TokenType::Between);
        keywords.insert("one".to_string(), TokenType::OneOf); // "one of" is handled specially
        
        // Logical operators
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("not".to_string(), TokenType::Not);

        let source_chars: Vec<char> = source.chars().collect();

        Self {
            source: source_chars,
            position: 0,
            line: 1,
            column: 1,
            indent_stack: vec![0],
            keywords,
            start_position: 0,
        }
    }

    // Helper methods
    fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    fn is_identifier_start(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_identifier_part(c: char) -> bool {
        Self::is_identifier_start(c) || c.is_digit(10)
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.position + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.position += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if Self::is_whitespace(c) && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn handle_indentation(&mut self) -> Vec<Token> {
        let mut indent = 0;
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' {
                indent += 1;
                self.advance();
            } else {
                break;
            }
        }

        let mut tokens = Vec::new();
        let current_indent = *self.indent_stack.last().unwrap();

        if indent > current_indent {
            self.indent_stack.push(indent);
            tokens.push(self.create_token(TokenType::Indent, "indent".to_string()));
        } else if indent < current_indent {
            while indent < *self.indent_stack.last().unwrap() {
                self.indent_stack.pop();
                tokens.push(self.create_token(TokenType::Dedent, "dedent".to_string()));
            }
        }

        tokens
    }

    fn create_token(&self, token_type: TokenType, value: String) -> Token {
        let column = if self.column >= value.len() {
            self.column - value.len()
        } else {
            self.column
        };

        Token::new(
            token_type,
            value.clone(),
            self.line,
            column,
            self.start_position..self.position,
        )
    }

    fn create_error(&self, message: String) -> Token {
        Token::error(
            message,
            self.line,
            self.column,
            self.start_position..self.position,
        )
    }

    fn scan_identifier(&mut self) -> Token {
        self.start_position = self.position;
        while let Some(c) = self.peek() {
            if Self::is_identifier_part(c) {
                self.advance();
            } else {
                break;
            }
        }

        let value: String = self.source[self.start_position..self.position].iter().collect();
        let lower_value = value.to_lowercase();

        // Check for compound tokens
        if lower_value == "one" {
            self.skip_whitespace();
            if let Some(next_word) = self.peek_word() {
                if next_word.to_lowercase() == "of" {
                    // Consume "of"
                    for _ in 0..next_word.len() {
                        self.advance();
                    }
                    return self.create_token(TokenType::OneOf, "one of".to_string());
                }
            }
        } else if lower_value == "is" {
            self.skip_whitespace();
            if let Some(next_word) = self.peek_word() {
                let next_lower = next_word.to_lowercase();
                if next_lower == "not" {
                    // Consume "not"
                    for _ in 0..next_word.len() {
                        self.advance();
                    }
                    return self.create_token(TokenType::NotEquals, "is not".to_string());
                } else if next_lower == "between" {
                    // Consume "between"
                    for _ in 0..next_word.len() {
                        self.advance();
                    }
                    return self.create_token(TokenType::Between, "is between".to_string());
                }
            }
            return self.create_token(TokenType::Equals, "is".to_string());
        }

        let token_type = self.keywords.get(&lower_value).cloned().unwrap_or(TokenType::Identifier);
        self.create_token(token_type, value)
    }

    fn peek_word(&mut self) -> Option<String> {
        let mut pos = self.position;
        let mut word = String::new();
        
        // Skip any whitespace
        while pos < self.source.len() && Self::is_whitespace(self.source[pos]) {
            pos += 1;
        }
        
        // Collect word characters
        while pos < self.source.len() && Self::is_identifier_part(self.source[pos]) {
            word.push(self.source[pos]);
            pos += 1;
        }
        
        if word.is_empty() {
            None
        } else {
            Some(word)
        }
    }

    fn scan_number(&mut self) -> Token {
        self.start_position = self.position;
        let mut has_decimal_point = false;

        while let Some(c) = self.peek() {
            if c == '.' {
                if has_decimal_point {
                    break; // Second decimal point
                } else {
                    has_decimal_point = true;
                }
            } else if !Self::is_digit(c) {
                break;
            }
            self.advance();
        }

        let value: String = self.source[self.start_position..self.position].iter().collect();
        self.create_token(TokenType::NumberLiteral, value)
    }

    fn scan_string(&mut self) -> Token {
        self.start_position = self.position;
        self.advance(); // Skip opening quote
        let mut value = String::new();
        let mut escaped = false;

        loop {
            match self.peek() {
                None => {
                    return self.create_error("Unterminated string literal".to_string());
                }
                Some('\\') if !escaped => {
                    escaped = true;
                    self.advance();
                }
                Some('"') if !escaped => {
                    self.advance(); // Skip closing quote
                    break;
                }
                Some('n') if escaped => {
                    value.push('\n');
                    escaped = false;
                    self.advance();
                }
                Some('t') if escaped => {
                    value.push('\t');
                    escaped = false;
                    self.advance();
                }
                Some('r') if escaped => {
                    value.push('\r');
                    escaped = false;
                    self.advance();
                }
                Some('\\') if escaped => {
                    value.push('\\');
                    escaped = false;
                    self.advance();
                }
                Some('"') if escaped => {
                    value.push('"');
                    escaped = false;
                    self.advance();
                }
                Some('_') if escaped => {
                    // Line continuation
                    escaped = false;
                    self.advance();
                    // Skip whitespace until newline
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            self.advance();
                            break;
                        } else if !Self::is_whitespace(c) {
                            return self.create_error("Only whitespace allowed after line continuation".to_string());
                        }
                        self.advance();
                    }
                    // Skip whitespace after newline
                    self.skip_whitespace();
                }
                Some(c) if escaped => {
                    return self.create_error(format!("Invalid escape sequence: \\{}", c));
                }
                Some(c) => {
                    value.push(c);
                    self.advance();
                }
            }
        }

        self.create_token(TokenType::StringLiteral, value)
    }

    fn scan_comment(&mut self) -> Token {
        self.start_position = self.position;
        self.advance(); // Skip first '/'
        self.advance(); // Skip second '/'
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            } else {
                self.advance();
            }
        }
        let value: String = self.source[self.start_position..self.position].iter().collect();
        self.create_token(TokenType::Comment, value.trim().to_string())
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.source.len() {
            return self.create_token(TokenType::EOF, "".to_string());
        }

        match self.peek() {
            Some('\n') => {
                self.advance();
                self.create_token(TokenType::Newline, "\n".to_string())
            }
            Some('"') => self.scan_string(),
            Some(c) if Self::is_digit(c) => self.scan_number(),
            Some(c) if Self::is_identifier_start(c) => self.scan_identifier(),
            Some('/') if self.peek_next() == Some('/') => self.scan_comment(),
            Some(c) => {
                // Handle single-character tokens and brackets
                let token_type = match c {
                    ':' => TokenType::Colon,
                    ',' => TokenType::Comma,
                    '.' => TokenType::Dot,
                    '+' => TokenType::Plus,
                    '-' => TokenType::Minus,
                    '*' => TokenType::Multiply,
                    '/' => TokenType::Divide,
                    '%' => TokenType::Modulo,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    '[' => TokenType::LeftBracket,
                    ']' => TokenType::RightBracket,
                    '(' => TokenType::LeftParen,
                    ')' => TokenType::RightParen,
                    _ => {
                        // Handle unknown character
                        let message = format!("Unexpected character: {}", c);
                        self.advance();
                        return self.create_error(message);
                    }
                };
                self.advance();
                self.create_token(token_type, c.to_string())
            }
            None => self.create_token(TokenType::EOF, "".to_string()),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        loop {
            // Handle indentation after newlines
            if tokens.last().map_or(true, |t: &Token| t.token_type == TokenType::Newline) {
                tokens.extend(self.handle_indentation());
            }
    
            let token = self.next_token();
            
            if token.token_type == TokenType::EOF {
                // Handle any remaining dedents
                while self.indent_stack.len() > 1 {
                    self.indent_stack.pop();
                    tokens.push(self.create_token(TokenType::Dedent, "dedent".to_string()));
                }
                tokens.push(token);
                break;
            }
            
            tokens.push(token);
        }
        
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn get_token_types(tokens: Vec<Token>) -> Vec<TokenType> {
        tokens.iter()
            .filter(|t| t.token_type != TokenType::EOF)
            .map(|t| t.token_type.clone())
            .collect()
    }

    #[test]
    fn test_basic_tokens() {
        let source = "define action called test";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens[0].token_type, TokenType::Define);
        assert_eq!(tokens[1].token_type, TokenType::Action);
        assert_eq!(tokens[2].token_type, TokenType::Called);
        assert_eq!(tokens[3].token_type, TokenType::Identifier);
    }

    #[test]
    fn test_string_literals() {
        let source = r#"store message as "Hello\nWorld\t\"quoted\"\\_continued""#;
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens[0].token_type, TokenType::Store);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::As);
        assert_eq!(tokens[3].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[3].value, "Hello\nWorld\t\"quoted\"\\_continued");
    }

    #[test]
    fn test_compound_tokens() {
        let source = "check if value is not between 1 and 10 or is one of options";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        let types = get_token_types(tokens);
        assert_eq!(types, vec![
            TokenType::Check,
            TokenType::If,
            TokenType::Identifier,
            TokenType::NotEquals,
            TokenType::Between,
            TokenType::NumberLiteral,
            TokenType::And,
            TokenType::NumberLiteral,
            TokenType::Or,
            TokenType::Equals,
            TokenType::OneOf,
            TokenType::Identifier,
        ]);
    }

    #[test]
    fn test_indentation() {
        let source = "if true:\n    action\n        nested\n    back\nend if";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        let types = get_token_types(tokens);
        assert!(types.contains(&TokenType::Indent));
        assert!(types.contains(&TokenType::Dedent));
        
        // Count indents and dedents
        let indent_count = types.iter().filter(|&t| *t == TokenType::Indent).count();
        let dedent_count = types.iter().filter(|&t| *t == TokenType::Dedent).count();
        assert_eq!(indent_count, dedent_count);
    }

    #[test]
    fn test_error_handling() {
        let source = "\"unterminated string";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens[0].token_type, TokenType::Error);
        assert!(tokens[0].error.is_some());
        assert_eq!(tokens[0].error.as_ref().unwrap(), "Unterminated string literal");
    }

    #[test]
    fn test_numbers() {
        let source = "42 3.14 1000000";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        let types = get_token_types(tokens);
        
        assert_eq!(types[0], TokenType::NumberLiteral);
        assert_eq!(types[1], TokenType::NumberLiteral);
        assert_eq!(types[2], TokenType::NumberLiteral);
    }

    #[test]
    fn test_comments() {
        let source = "action // This is a comment\ncode // Another comment";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        let types = get_token_types(tokens);
        assert_eq!(types[0], TokenType::Action);
        assert_eq!(types[1], TokenType::Comment);
        assert_eq!(types[2], TokenType::Newline);
        assert_eq!(types[3], TokenType::Identifier);
        assert_eq!(types[4], TokenType::Comment);
    }
}
