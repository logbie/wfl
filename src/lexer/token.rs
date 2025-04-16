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
    
    // New keywords
    Set,
    To,
    Is,
    Increase,
    By,
    At,
    Must,
    Display,
    New,
    Container,
    Private,
    Public,
    Join,
    Default,
    Created,
    And,
    Perform,

    // Data Types
    Number,
    Text,
    Truth,
    List,
    Map,
    Record,

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

    // Comparison Operators
    Equals,            // is, =
    NotEquals,         // is not, ≠
    Greater,           // is greater than, >
    Less,              // is less than, <
    GreaterEquals,     // is at least, ≥
    LessEquals,        // is at most, ≤

    // Logical Operators
    Or,
    Not,

    // Delimiters
    Colon,             // :
    Comma,             // ,
    Dot,               // .
    Quotes,            // "
    LeftParen,         // (
    RightParen,        // )
    LeftBracket,       // [
    RightBracket,      // ]
    
    // Special
    Comment,           // // Single line comment
    Newline,
    Indent,
    Dedent,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}
