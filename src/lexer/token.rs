#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Keywords
    Define,
    Action,
    Called,
    Needs,
    Does,
    End,
    Give,
    Back,
    Store,
    As,
    Create,
    With,
    Check,
    If,
    Otherwise,
    When,
    For,
    Each,
    In,
    Repeat,
    While,
    Until,
    Try,
    Catch,
    Finally,
    Public,
    Private,
    Protected,
    Container,
    From,
    Interface,
    Implements,

    // Data Types
    Number,
    Text,
    Truth,
    List,
    Map,
    Set,
    Record,
    Any,
    Of,
    Type,

    // Literals
    StringLiteral,     // "hello"
    NumberLiteral,     // 42, 3.14
    TruthLiteral,      // yes, no, true, false
    Identifier,        // variable names, action names

    // Special Values
    Nothing,
    Missing,
    Undefined,
    Empty,

    // Operators
    Plus,              // plus, +
    Minus,             // minus, -
    Multiply,          // times, *
    Divide,            // divided by, /
    Modulo,            // modulo, %
    Join,              // join
    To,               // to
    Into,             // into
    At,               // at
    Where,            // where
    Contains,         // contains
    Matches,          // matches

    // Comparison Operators
    Equals,            // is, =
    NotEquals,         // is not, ≠
    Greater,           // is greater than, >
    Less,              // is less than, <
    GreaterEquals,     // is at least, ≥
    LessEquals,        // is at most, ≤
    Between,           // is between
    OneOf,            // is one of

    // Logical Operators
    And,
    Or,
    Not,

    // Delimiters
    Colon,             // :
    Comma,             // ,
    Dot,               // .
    Quotes,            // "
    LeftBrace,         // {
    RightBrace,        // }
    LeftBracket,       // [
    RightBracket,      // ]
    LeftParen,         // (
    RightParen,        // )

    // Special
    Comment,           // // Single line comment
    Newline,
    Indent,
    Dedent,
    EOF,
    Error,            // For lexical errors
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
    pub span: std::ops::Range<usize>,
    pub error: Option<String>,  // For storing error messages
}

impl Token {
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize, span: std::ops::Range<usize>) -> Self {
        Self {
            token_type,
            value,
            line,
            column,
            span,
            error: None,
        }
    }

    pub fn error(message: String, line: usize, column: usize, span: std::ops::Range<usize>) -> Self {
        Self {
            token_type: TokenType::Error,
            value: String::new(),
            line,
            column,
            span,
            error: Some(message),
        }
    }

    pub fn is_error(&self) -> bool {
        self.token_type == TokenType::Error
    }

    pub fn location(&self) -> String {
        format!("line {}, column {}", self.line, self.column)
    }

    pub fn span(&self) -> std::ops::Range<usize> {
        self.span.clone()
    }
}
