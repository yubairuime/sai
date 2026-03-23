use crate::parser::Position;

use super::Expr;

#[derive(Debug, Clone)]
pub struct AndExpr {
    pub position: Position,
    pub conditions: Vec<Expr>,
}
