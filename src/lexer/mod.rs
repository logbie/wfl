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

        let source_chars: Vec<char> = source.chars().collect();

        Self {
            source: source_chars,
            position: 0,
            line: 1,
            column: 1,
            indent_stack: vec![0],
            keywords,
        }
    }

    // Helper methods
    fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }

    fn is_alpha(c: char) -> bool {
        c.is_alphabetic()
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
        Token {
            token_type,
            value,
            line: self.line,
            column: self.column - value.len(),
        }
    }

    fn scan_identifier(&mut self) -> Token {
        let start_position = self.position;
        while let Some(c) = self.peek() {
            if Self::is_identifier_part(c) {
                self.advance();
            } else {
                break;
            }
        }

        let value: String = self.source[start_position..self.position].iter().collect();
        let lower_value = value.to_lowercase();

        let token_type = self.keywords.get(&lower_value).cloned().unwrap_or(TokenType::Identifier);
        self.create_token(token_type, value)
    }

    fn scan_number(&mut self) -> Token {
        let start_position = self.position;
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

        let value: String = self.source[start_position..self.position].iter().collect();
        self.create_token(TokenType::NumberLiteral, value)
    }

    fn scan_string(&mut self) -> Token {
        self.advance(); // Skip opening quote
        let start_position = self.position;
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            } else {
                self.advance();
            }
        }
        let value: String = self.source[start_position..self.position].iter().collect();
        if self.peek() == Some('"') {
            self.advance(); // Skip closing quote
        }
        self.create_token(TokenType::StringLiteral, value)
    }

    fn scan_comment(&mut self) -> Token {
        self.advance(); // Skip first '/'
        self.advance(); // Skip second '/'
        let start_position = self.position;
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            } else {
                self.advance();
            }
        }
        let value: String = self.source[start_position..self.position].iter().collect();
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
                // Handle single-character tokens
                let token_type = match c {
                    ':' => TokenType::Colon,
                    ',' => TokenType::Comma,
                    '.' => TokenType::Dot,
                    '+' => TokenType::Plus,
                    '-' => TokenType::Minus,
                    '*' => TokenType::Multiply,
                    '/' => TokenType::Divide,
                    '%' => TokenType::Modulo,
                    _ => {
                        self.advance(); // Skip unknown character
                        return self.next_token();
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
            let token = self.next_token();
            if token.token_type == TokenType::Newline {
                tokens.push(token);
                tokens.extend(self.handle_indentation());
            } else {
                tokens.push(token);
            }
            if token.token_type == TokenType::EOF {
                break;
            }
        }
        tokens
    }
}
