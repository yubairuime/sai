use crate::parser::Position;

use super::Expr;

#[derive(Debug, Clone)]
pub struct Funcall {
    pub position: Position,
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}
