use crate::parser::Position;

#[derive(Debug, Clone)]
pub struct Literal {
    pub position: Position,
    pub ty: LiteralType,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum LiteralType {
    Int,
    Float,
    Bool,
    String,
    Unit,
}

impl Literal {
    pub fn unit(position: Position) -> Self {
        Literal {
            position: position,
            ty: LiteralType::Unit,
            value: "".to_string(),
        }
    }
}
