mod lexer;
pub use lexer::{Lexer, Token, TokenType};

#[derive(Clone)]
pub struct TokenLocation {
    pub line_content: String,
    pub length: usize,
    // ... other fields ...
}
