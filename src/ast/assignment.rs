use crate::parser::Position;

use super::{Expr, Variable};

#[derive(Debug, Clone)]
pub struct Assignment {
    pub position: Position,
    pub variable: Variable,
    pub value: Box<Expr>,
}
