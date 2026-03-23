use crate::parser::Position;

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub position: Position,
    pub value: TypeExpr,
}

#[derive(Debug, Clone)]
pub enum TypeExpr {
    Named(String),
    Function {
        params: Vec<TypeAnnotation>,
        ret: Box<TypeAnnotation>,
    },
}
