use std::fmt;

/// Represents a location in the source code
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

/// Represents a node in the AST with location information
#[derive(Debug, Clone)]
pub struct Node<T> {
    pub span: Span,
    pub node: T,
}

/// Represents a type in WFL
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    Text,
    Truth,
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Set(Box<Type>),
    Record(Vec<(String, Type)>),
    Action(Vec<Type>, Box<Type>), // Parameter types and return type
    Any,
    Generic(String),              // For generic type parameters
}

/// Represents a literal value
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    Text(String),
    Truth(bool),
    Nothing,
    Missing,
    Undefined,
    Empty,
}

/// Represents an operator
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    // Comparison
    Equals,
    NotEquals,
    Greater,
    Less,
    GreaterEquals,
    LessEquals,
    Between,
    OneOf,

    // Logical
    And,
    Or,
    Not,

    // Collection
    Join,
    Contains,
    At,
}

/// Represents an expression
#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    BinaryOp {
        left: Box<Node<Expression>>,
        op: Operator,
        right: Box<Node<Expression>>,
    },
    UnaryOp {
        op: Operator,
        expr: Box<Node<Expression>>,
    },
    Call {
        action: Box<Node<Expression>>,
        args: Vec<Node<Expression>>,
        named_args: Vec<(String, Node<Expression>)>,
    },
    Access {
        object: Box<Node<Expression>>,
        field: String,
    },
    List(Vec<Node<Expression>>),
    Map(Vec<(Node<Expression>, Node<Expression>)>),
    Set(Vec<Node<Expression>>),
    Record(Vec<(String, Node<Expression>)>),
}

/// Represents a pattern in pattern matching
#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Variable(String),
    List(Vec<Node<Pattern>>),
    Record(Vec<(String, Node<Pattern>)>),
    TypePattern {
        type_name: Type,
        constraints: Vec<Node<Expression>>,
    },
}

/// Represents a statement
#[derive(Debug, Clone)]
pub enum Statement {
    Store {
        name: String,
        value: Node<Expression>,
        type_annotation: Option<Type>,
    },
    Assign {
        target: Node<Expression>,
        value: Node<Expression>,
    },
    If {
        condition: Node<Expression>,
        then_block: Vec<Node<Statement>>,
        else_block: Option<Vec<Node<Statement>>>,
    },
    Check {
        value: Node<Expression>,
        patterns: Vec<(Node<Pattern>, Vec<Node<Statement>>)>,
        else_block: Option<Vec<Node<Statement>>>,
    },
    ForEach {
        item: String,
        collection: Node<Expression>,
        body: Vec<Node<Statement>>,
    },
    While {
        condition: Node<Expression>,
        body: Vec<Node<Statement>>,
    },
    Until {
        condition: Node<Expression>,
        body: Vec<Node<Statement>>,
    },
    Try {
        body: Vec<Node<Statement>>,
        catch_blocks: Vec<(Pattern, Vec<Node<Statement>>)>,
        finally_block: Option<Vec<Node<Statement>>>,
    },
    Return(Node<Expression>),
    Break(Option<String>),        // Optional label
    Continue(Option<String>),     // Optional label
    Expression(Node<Expression>), // For expression statements
}

/// Represents a parameter in an action declaration
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub default_value: Option<Node<Expression>>,
}

/// Represents visibility level
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

/// Represents a declaration
#[derive(Debug, Clone)]
pub enum Declaration {
    Action {
        name: String,
        visibility: Visibility,
        generic_params: Vec<String>,
        params: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Node<Statement>>,
    },
    Container {
        name: String,
        visibility: Visibility,
        generic_params: Vec<String>,
        extends: Option<Type>,
        implements: Vec<Type>,
        fields: Vec<(String, Type, Visibility)>,
        methods: Vec<Node<Declaration>>,
    },
    Interface {
        name: String,
        visibility: Visibility,
        generic_params: Vec<String>,
        extends: Vec<Type>,
        methods: Vec<(String, Vec<Parameter>, Option<Type>)>,
    },
}

/// Represents a complete WFL program
#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<Node<Declaration>>,
}

// Implement Display for better error messages
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Number => write!(f, "number"),
            Type::Text => write!(f, "text"),
            Type::Truth => write!(f, "truth"),
            Type::List(t) => write!(f, "list of {}", t),
            Type::Map(k, v) => write!(f, "map from {} to {}", k, v),
            Type::Set(t) => write!(f, "set of {}", t),
            Type::Record(fields) => {
                write!(f, "record {{")?;
                for (i, (name, typ)) in fields.iter().enumerate() {
                    if i > 0 { write!(f, ", ")? }
                    write!(f, "{}: {}", name, typ)?;
                }
                write!(f, "}}")
            }
            Type::Action(params, ret) => {
                write!(f, "action(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")? }
                    write!(f, "{}", param)?;
                }
                write!(f, ") giving {}", ret)
            }
            Type::Any => write!(f, "any"),
            Type::Generic(name) => write!(f, "{}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_display() {
        assert_eq!(Type::Number.to_string(), "number");
        assert_eq!(Type::Text.to_string(), "text");
        assert_eq!(Type::Truth.to_string(), "truth");
        assert_eq!(Type::List(Box::new(Type::Number)).to_string(), "list of number");
        assert_eq!(
            Type::Map(Box::new(Type::Text), Box::new(Type::Number)).to_string(),
            "map from text to number"
        );
        assert_eq!(Type::Set(Box::new(Type::Text)).to_string(), "set of text");
        assert_eq!(
            Type::Record(vec![
                ("name".to_string(), Type::Text),
                ("age".to_string(), Type::Number)
            ]).to_string(),
            "record {name: text, age: number}"
        );
        assert_eq!(
            Type::Action(
                vec![Type::Text, Type::Number],
                Box::new(Type::Truth)
            ).to_string(),
            "action(text, number) giving truth"
        );
        assert_eq!(Type::Any.to_string(), "any");
        assert_eq!(Type::Generic("T".to_string()).to_string(), "T");
    }

    #[test]
    fn test_ast_node_creation() {
        let span = Span {
            start: 0,
            end: 5,
            line: 1,
            column: 1,
        };

        // Test literal expression
        let literal_expr = Node {
            span: span.clone(),
            node: Expression::Literal(Literal::Number(42.0)),
        };

        // Test binary operation
        let binary_op = Node {
            span: span.clone(),
            node: Expression::BinaryOp {
                left: Box::new(literal_expr.clone()),
                op: Operator::Plus,
                right: Box::new(Node {
                    span: span.clone(),
                    node: Expression::Literal(Literal::Number(1.0)),
                }),
            },
        };

        // Test statement
        let store_stmt = Node {
            span: span.clone(),
            node: Statement::Store {
                name: "x".to_string(),
                value: binary_op.clone(),
                type_annotation: Some(Type::Number),
            },
        };

        // Verify the AST structure
        if let Statement::Store { name, value, type_annotation } = &store_stmt.node {
            assert_eq!(name, "x");
            assert_eq!(type_annotation, &Some(Type::Number));
            
            if let Expression::BinaryOp { left, op, right } = &value.node {
                assert!(matches!(op, Operator::Plus));
                
                if let Expression::Literal(Literal::Number(n)) = &left.node {
                    assert_eq!(*n, 42.0);
                } else {
                    panic!("Expected number literal");
                }
                
                if let Expression::Literal(Literal::Number(n)) = &right.node {
                    assert_eq!(*n, 1.0);
                } else {
                    panic!("Expected number literal");
                }
            } else {
                panic!("Expected binary operation");
            }
        } else {
            panic!("Expected store statement");
        }
    }

    #[test]
    fn test_container_declaration() {
        let span = Span {
            start: 0,
            end: 10,
            line: 1,
            column: 1,
        };

        let container = Node {
            span: span.clone(),
            node: Declaration::Container {
                name: "Person".to_string(),
                visibility: Visibility::Public,
                generic_params: vec!["T".to_string()],
                extends: None,
                implements: vec![],
                fields: vec![
                    ("name".to_string(), Type::Text, Visibility::Private),
                    ("age".to_string(), Type::Number, Visibility::Private),
                ],
                methods: vec![],
            },
        };

        if let Declaration::Container {
            name,
            visibility,
            generic_params,
            fields,
            ..
        } = &container.node {
            assert_eq!(name, "Person");
            assert_eq!(visibility, &Visibility::Public);
            assert_eq!(generic_params, &vec!["T".to_string()]);
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].0, "name");
            assert_eq!(fields[0].1, Type::Text);
            assert_eq!(fields[0].2, Visibility::Private);
        } else {
            panic!("Expected container declaration");
        }
    }
}