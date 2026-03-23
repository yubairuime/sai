use crate::parser::Position;

use super::Expr;

#[derive(Debug, Clone)]
pub struct OrExpr {
    pub position: Position,
    pub conditions: Vec<Expr>,
}
