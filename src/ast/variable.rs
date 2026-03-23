use crate::parser::Position;

#[derive(Debug, Clone)]
pub struct Variable {
    pub position: Position,
    pub value: String,
}
