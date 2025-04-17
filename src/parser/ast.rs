use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        value: Expression,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    IfStatement {
        condition: Expression,
        then_block: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    },
    SingleLineIf {
        condition: Expression,
        then_stmt: Box<Statement>,
        else_stmt: Option<Box<Statement>>,
    },
    ForEachLoop {
        item_name: String,
        collection: Expression,
        reversed: bool,
        body: Vec<Statement>,
    },
    CountLoop {
        start: Expression,
        end: Expression,
        step: Option<Expression>,
        downward: bool,
        body: Vec<Statement>,
    },
    WhileLoop {
        condition: Expression,
        body: Vec<Statement>,
    },
    RepeatUntilLoop {
        condition: Expression,
        body: Vec<Statement>,
    },
    ForeverLoop {
        body: Vec<Statement>,
    },
    DisplayStatement {
        value: Expression,
    },
    ActionDefinition {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
        return_type: Option<Type>,
    },
    ReturnStatement {
        value: Option<Expression>,
    },
    ExpressionStatement {
        expression: Expression,
    },
    BreakStatement,
    ContinueStatement,
    OpenFileStatement {
        path: Expression,
        variable_name: String,
    },
    ReadFileStatement {
        file: Expression,
        variable_name: String,
    },
    WriteFileStatement {
        file: Expression,
        content: Expression,
    },
    CloseFileStatement {
        file: Expression,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    BinaryOperation {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    UnaryOperation {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Argument>,
    },
    MemberAccess {
        object: Box<Expression>,
        property: String,
    },
    IndexAccess {
        collection: Box<Expression>,
        index: Box<Expression>,
    },
    Concatenation {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Nothing,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    And,
    Or,
    Contains,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<Type>,
    pub default_value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: Option<String>,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Text,
    Number,
    Boolean,
    Nothing,
    Custom(String),
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl ParseError {
    pub fn new(message: String, line: usize, column: usize) -> Self {
        ParseError {
            message,
            line,
            column,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for ParseError {}
