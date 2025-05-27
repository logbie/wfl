#[derive(Debug, Clone, PartialEq, Default)]
pub enum ControlFlow {
    #[default]
    None,
    Break,
    Continue,
    Exit,
    Return(crate::interpreter::Value),
}
