use crate::parser::Position;

use super::{Block, TypeAnnotation, Variable};

#[derive(Debug, Clone)]
pub struct Closure {
    pub position: Position,
    pub params: Vec<Parameter>,
    pub return_type: TypeAnnotation,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub position: Position,
    pub name: Variable,
    pub ty: TypeAnnotation,
}
