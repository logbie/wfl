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
        keywords.insert("gives".to_string(), TokenType::Give);
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
        
        // Add new keywords
        keywords.insert("set".to_string(), TokenType::Set);
        keywords.insert("to".to_string(), TokenType::To);
        keywords.insert("is".to_string(), TokenType::Is);
        keywords.insert("increase".to_string(), TokenType::Increase);
        keywords.insert("by".to_string(), TokenType::By);
        keywords.insert("at".to_string(), TokenType::At);
        keywords.insert("must".to_string(), TokenType::Must);
        keywords.insert("display".to_string(), TokenType::Display);
        keywords.insert("new".to_string(), TokenType::New);
        keywords.insert("container".to_string(), TokenType::Container);
        keywords.insert("private".to_string(), TokenType::Private);
        keywords.insert("public".to_string(), TokenType::Public);
        keywords.insert("join".to_string(), TokenType::Join);
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("default".to_string(), TokenType::Default);
        keywords.insert("created".to_string(), TokenType::Created);
        keywords.insert("perform".to_string(), TokenType::Perform);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("each".to_string(), TokenType::Each);
        keywords.insert("in".to_string(), TokenType::In);
        keywords.insert("repeat".to_string(), TokenType::Repeat);
        keywords.insert("while".to_string(), TokenType::While);
        keywords.insert("until".to_string(), TokenType::Until);
        keywords.insert("try".to_string(), TokenType::Try);
        keywords.insert("catch".to_string(), TokenType::Catch);
        keywords.insert("finally".to_string(), TokenType::Finally);

        // Add data types
        keywords.insert("number".to_string(), TokenType::Number);
        keywords.insert("text".to_string(), TokenType::Text);
        keywords.insert("truth".to_string(), TokenType::Truth);
        keywords.insert("list".to_string(), TokenType::List);
        keywords.insert("map".to_string(), TokenType::Map);
        keywords.insert("record".to_string(), TokenType::Record);

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
        let column = if self.column >= value.len() {
            self.column - value.len()
        } else {
            self.column
        };

        Token {
            token_type,
            value: value.clone(),
            line: self.line,
            column,
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
                    '(' => TokenType::LeftParen,
                    ')' => TokenType::RightParen,
                    '[' => TokenType::LeftBracket,
                    ']' => TokenType::RightBracket,
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
