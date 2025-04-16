use std::collections::HashMap;

// Program is the root node of our AST
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// Represents all possible statement types in WFL
#[derive(Debug, Clone)]
pub enum Statement {
    // Variable/data declarations
    VariableDeclaration {
        name: String,
        value_type: Option<String>,
        initializer: Option<Expression>,
    },
    
    // Flow control
    CheckStatement {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    
    ForEachLoop {
        item_name: String,
        index_name: Option<String>,
        collection: Expression,
        body: Vec<Statement>,
    },
    
    RepeatLoop {
        is_while: bool, // true for while, false for until
        condition: Expression,
        body: Vec<Statement>,
    },
    
    CountLoop {
        counter_name: String,
        start: Expression,
        end: Expression,
        step: Option<Expression>,
        body: Vec<Statement>,
    },
    
    TryCatch {
        try_block: Vec<Statement>,
        catch_variable: Option<String>,
        catch_block: Vec<Statement>,
        finally_block: Option<Vec<Statement>>,
    },
    
    // Module level
    ActionDefinition {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<String>,
        body: Vec<Statement>,
        is_async: bool,
        is_private: bool,
    },
    
    ContainerDefinition {
        name: String,
        fields: Vec<VariableField>,
        methods: Vec<Statement>, // Will contain ActionDefinition statements
        constructor: Option<Vec<Statement>>,
    },
    
    // Expressions as statements
    ExpressionStatement(Expression),
    
    // Control flow
    ReturnStatement(Option<Expression>),
    BreakStatement(Option<String>), // Optional label name
    ContinueStatement(Option<String>), // Optional label name
}

// Parameter definition for function/action parameters
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub default_value: Option<Expression>,
}

// Field definition for container/class fields
#[derive(Debug, Clone)]
pub struct VariableField {
    pub name: String,
    pub field_type: String,
    pub is_private: bool,
    pub initializer: Option<Expression>,
}

// Expressions represent values and operations
#[derive(Debug, Clone)]
pub enum Expression {
    // Literals
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    NothingLiteral,
    
    // Variables and identifiers
    Variable(String),
    
    // Collections
    ListExpression(Vec<Expression>),
    MapExpression(HashMap<String, Expression>),
    SetExpression(Vec<Expression>),
    RecordExpression(HashMap<String, Expression>),
    
    // Operations
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    
    Unary {
        operator: UnaryOperator,
        right: Box<Expression>,
    },
    
    // Function calls
    Call {
        callee: Box<Expression>,
        arguments: Vec<NamedArgument>,
    },
    
    // Member access
    MemberAccess {
        object: Box<Expression>,
        name: String,
    },
    
    // Indexing (for collections)
    Index {
        collection: Box<Expression>,
        index: Box<Expression>,
    },
    
    // Async operations
    Await(Box<Expression>),
    
    // Error handling
    Try(Box<Expression>),
}

// Named argument for function calls
#[derive(Debug, Clone)]
pub struct NamedArgument {
    pub name: Option<String>,
    pub value: Expression,
}

// Binary operators (+, -, *, /, etc.)
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    
    // Comparison
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    
    // Logical
    And,
    Or,
    
    // Collection
    Join, // String concatenation or list joining
    
    // Assignment 
    Assign, // Variable assignment operation
}

// Unary operators (not, -, etc.)
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negate,
} 