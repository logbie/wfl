use std::str::CharIndices;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords (unchanged)
    Define,
    Action,
    End,
    Store,
    As,
    When,
    Check,
    Otherwise,
    Give,
    Back,
    Create,
    Try,
    Catch,
    Finally,
    Module,
    Public,
    Private,
    With,
    Needs,
    Do,
    
    // Control Flow
    If,
    Then,
    For,
    Each,
    In,
    While,
    Until,
    Perform,
    
    // Data Types
    Number,
    Text,
    Truth,
    Nothing,
    Map,
    List,
    
    // Literals
    StringLiteral(String),
    NumberLiteral(f64),
    TruthLiteral(bool),
    
    // Identifiers and Properties
    Identifier(String),
    Property(String),        // New: For handling 's properties
    
    // Extended Operators
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Modulo,                 // New
    Power,                  // New
    Increment,              // New
    Decrement,             // New
    And,                   // New
    Or,                    // New
    Not,                   // New
    
    // WFL-specific operators
    Concatenate,           // New: for text joining
    Contains,             // New: for collection membership
    Matches,              // New: for pattern matching
    
    // Delimiters
    Colon,
    Comma,
    Newline,
    Period,              // New
    
    // Special
    EOF,
    Invalid(String),
}

#[derive(Debug, Clone)] // Add Clone derive
pub struct TokenLocation {
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub absolute_position: usize,
    pub line_content: String,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub location: TokenLocation,
}

pub struct Lexer<'a> {
    input: Peekable<CharIndices<'a>>,
    line: usize,
    column: usize,
    absolute_position: usize,
    current_char: Option<(usize, char)>,
    lines: Vec<String>,
    current_line_content: String,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
        let mut lexer = Lexer {
            input: input.char_indices().peekable(),
            line: 1,
            column: 0,
            absolute_position: 0,
            current_char: None,
            lines,
            current_line_content: String::new(),
        };
        lexer.advance();
        lexer
    }
    
    fn advance(&mut self) {
        self.current_char = self.input.next();
        if let Some((pos, ch)) = self.current_char {
            self.absolute_position = pos;
            if ch == '\n' {
                self.line += 1;
                self.column = 0;
                self.current_line_content = self.lines.get(self.line - 1)
                    .unwrap_or(&String::new())
                    .clone();
            } else {
                self.column += 1;
            }
        }
    }
    
    fn peek_next(&mut self) -> Option<char> {
        self.input.peek().map(|(_, ch)| *ch)
    }
    
    fn skip_whitespace(&mut self) {
        while let Some((_, ch)) = self.current_char {
            match ch {
                ' ' | '\t' | '\r' => self.advance(),
                '/' => {
                    if let Some('/') = self.peek_next() {
                        self.skip_comment();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
    
    fn skip_comment(&mut self) {
        self.advance(); // Skip second '/'
        while let Some((_, ch)) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }
    
    fn read_identifier_or_keyword(&mut self) -> (TokenType, usize) {
        let start_pos = self.absolute_position;
        let mut identifier = String::new();
        
        while let Some((_, ch)) = self.current_char {
            if !ch.is_alphanumeric() && ch != '_' {
                // Handle property access ('s)
                if ch == '\'' && self.peek_next() == Some('s') {
                    self.advance(); // Skip '
                    self.advance(); // Skip s
                    return (TokenType::Property(identifier), self.absolute_position - start_pos);
                }
                break;
            }
            identifier.push(ch);
            self.advance();
        }
        
        let token_type = match identifier.as_str() {
            // Keywords
            "define" => TokenType::Define,
            "action" => TokenType::Action,
            "end" => TokenType::End,
            "store" => TokenType::Store,
            "as" => TokenType::As,
            "when" => TokenType::When,
            "check" => TokenType::Check,
            "otherwise" => TokenType::Otherwise,
            "give" => TokenType::Give,
            "back" => TokenType::Back,
            "create" => TokenType::Create,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "finally" => TokenType::Finally,
            "if" => TokenType::If,
            "then" => TokenType::Then,
            "for" => TokenType::For,
            "each" => TokenType::Each,
            "in" => TokenType::In,
            "while" => TokenType::While,
            "until" => TokenType::Until,
            "with" => TokenType::With,
            "module" => TokenType::Module,
            "public" => TokenType::Public,
            "private" => TokenType::Private,
            "needs" => TokenType::Needs,
            "do" => TokenType::Do,
            "perform" => TokenType::Perform,
            "number" => TokenType::Number,
            "text" => TokenType::Text,
            "truth" => TokenType::Truth,
            "nothing" => TokenType::Nothing,
            "map" => TokenType::Map,
            "list" => TokenType::List,
            "yes" => TokenType::TruthLiteral(true),
            "no" => TokenType::TruthLiteral(false),
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
            "contains" => TokenType::Contains,
            "matches" => TokenType::Matches,
            _ => TokenType::Identifier(identifier),
        };
        
        (token_type, self.absolute_position - start_pos)
    }
    
    fn read_number(&mut self) -> Result<(TokenType, usize), String> {
        let start_pos = self.absolute_position;
        let mut number = String::new();
        let mut has_decimal = false;
        
        while let Some((_, ch)) = self.current_char {
            match ch {
                '0'..='9' => {
                    number.push(ch);
                    self.advance();
                }
                '.' if !has_decimal => {
                    has_decimal = true;
                    number.push(ch);
                    self.advance();
                }
                _ => break,
            }
        }
        
        match number.parse::<f64>() {
            Ok(n) => Ok((TokenType::NumberLiteral(n), self.absolute_position - start_pos)),
            Err(e) => Err(e.to_string()),
        }
    }
    
    fn read_string(&mut self) -> Result<(TokenType, usize), String> {
        let start_pos = self.absolute_position;
        let mut string = String::new();
        self.advance(); // Skip opening quote
        
        while let Some((_, ch)) = self.current_char {
            match ch {
                '"' => {
                    self.advance(); // Skip closing quote
                    return Ok((TokenType::StringLiteral(string), self.absolute_position - start_pos));
                }
                '\\' => {
                    self.advance();
                    if let Some((_, escape_char)) = self.current_char {
                        match escape_char {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            '\\' => string.push('\\'),
                            '"' => string.push('"'),
                            _ => return Err(format!("Invalid escape sequence: \\{}", escape_char)),
                        }
                        self.advance();
                    }
                }
                _ => {
                    string.push(ch);
                    self.advance();
                }
            }
        }
        
        Err("Unterminated string literal".to_string())
    }
    
    fn create_token_location(&self, length: usize) -> TokenLocation {
        TokenLocation {
            line: self.line,
            column: self.column,
            length,
            absolute_position: self.absolute_position,
            line_content: self.current_line_content.clone(),
        }
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let token = match self.current_char {
            None => Token {
                token_type: TokenType::EOF,
                location: self.create_token_location(0),
            },
            
            Some((_, ch)) => {
                let (token_type, length) = match ch {
                    // Single character tokens
                    ':' => {
                        self.advance();
                        (TokenType::Colon, 1)
                    }
                    ',' => {
                        self.advance();
                        (TokenType::Comma, 1)
                    }
                    '.' => {
                        self.advance();
                        (TokenType::Period, 1)
                    }
                    '+' => {
                        self.advance();
                        if let Some('+') = self.peek_next() {
                            self.advance();
                            (TokenType::Increment, 2)
                        } else {
                            (TokenType::Plus, 1)
                        }
                    }
                    '-' => {
                        self.advance();
                        if let Some('-') = self.peek_next() {
                            self.advance();
                            (TokenType::Decrement, 2)
                        } else {
                            (TokenType::Minus, 1)
                        }
                    }
                    '*' => {
                        self.advance();
                        if let Some('*') = self.peek_next() {
                            self.advance();
                            (TokenType::Power, 2)
                        } else {
                            (TokenType::Times, 1)
                        }
                    }
                    '/' => {
                        if let Some('/') = self.peek_next() {
                            self.skip_comment();
                            return self.next_token();
                        } else {
                            self.advance();
                            (TokenType::Divide, 1)
                        }
                    }
                    '%' => {
                        self.advance();
                        (TokenType::Modulo, 1)
                    }
                    '=' => {
                        self.advance();
                        if let Some('=') = self.peek_next() {
                            self.advance();
                            (TokenType::Equal, 2)
                        } else {
                            (TokenType::Equal, 1)
                        }
                    }
                    '!' => {
                        self.advance();
                        if let Some('=') = self.peek_next() {
                            self.advance();
                            (TokenType::NotEqual, 2)
                        } else {
                            (TokenType::Not, 1)
                        }
                    }
                    '>' => {
                        self.advance();
                        if let Some('=') = self.peek_next() {
                            self.advance();
                            (TokenType::GreaterEqual, 2)
                        } else {
                            (TokenType::Greater, 1)
                        }
                    }
                    '<' => {
                        self.advance();
                        if let Some('=') = self.peek_next() {
                            self.advance();
                            (TokenType::LessEqual, 2)
                        } else {
                            (TokenType::Less, 1)
                        }
                    }
                    '\n' => {
                        self.advance();
                        (TokenType::Newline, 1)
                    }
                    
                    // String literals
                    '"' => match self.read_string() {
                        Ok((token_type, length)) => (token_type, length),
                        Err(e) => (TokenType::Invalid(e), 1),
                    },
                    
                    // Numbers
                    '0'..='9' => match self.read_number() {
                        Ok((token_type, length)) => (token_type, length),
                        Err(e) => (TokenType::Invalid(e), 1),
                    },
                    
                    // Identifiers and keywords
                    ch if ch.is_alphabetic() || ch == '_' => {
                        self.read_identifier_or_keyword()
                    }
                    
                    // Skip carriage return
                    '\r' => {
                        self.advance();
                        return self.next_token();
                    }
                    
                    // Invalid characters
                    _ => {
                        let invalid_char = ch;
                        self.advance();
                        (TokenType::Invalid(format!("Unexpected character: {}", invalid_char)), 1)
                    }
                };
                
                Token {
                    token_type,
                    location: self.create_token_location(length),
                }
            }
        };
        
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compound_tokens() {
        let input = "greeting's text";
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier("greeting".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::Property("text".to_string()));
    }
    
    #[test]
    fn test_operators() {
        let input = "1 + 2 >= 3 != 4";
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token().token_type, TokenType::NumberLiteral(1.0));
        assert_eq!(lexer.next_token().token_type, TokenType::Plus);
        assert_eq!(lexer.next_token().token_type, TokenType::NumberLiteral(2.0));
        assert_eq!(lexer.next_token().token_type, TokenType::GreaterEqual);
        assert_eq!(lexer.next_token().token_type, TokenType::NumberLiteral(3.0));
        assert_eq!(lexer.next_token().token_type, TokenType::NotEqual);
        assert_eq!(lexer.next_token().token_type, TokenType::NumberLiteral(4.0));
    }
    
    #[test]
    fn test_location_tracking() {
        let input = "define action\n  hello";
        let mut lexer = Lexer::new(input);
        
        let token = lexer.next_token();
        assert_eq!(token.token_type, TokenType::Define);
        assert_eq!(token.location.line, 1);
        assert_eq!(token.location.column, 1);
        assert_eq!(token.location.length, 6);
        assert_eq!(token.location.line_content, "define action");
        
        let token = lexer.next_token();
        assert_eq!(token.token_type, TokenType::Action);
        assert_eq!(token.location.line, 1);
        
        let token = lexer.next_token();
        assert_eq!(token.token_type, TokenType::Newline);
        
        let token = lexer.next_token();
        assert_eq!(token.token_type, TokenType::Identifier("hello".to_string()));
        assert_eq!(token.location.line, 2);
        assert_eq!(token.location.column, 3);
    }
    
    #[test]
    fn test_string_escapes() {
        let input = r#""Hello\nWorld\t\"Test\"""#;
        let mut lexer = Lexer::new(input);
        
        match lexer.next_token().token_type {
            TokenType::StringLiteral(s) => {
                assert_eq!(s, "Hello\nWorld\t\"Test\"");
            }
            _ => panic!("Expected string literal"),
        }
    }
    
    #[test]
    fn test_comments() {
        let input = "define // this is a comment\naction";
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token().token_type, TokenType::Define);
        assert_eq!(lexer.next_token().token_type, TokenType::Newline);
        assert_eq!(lexer.next_token().token_type, TokenType::Action);
    }
}
