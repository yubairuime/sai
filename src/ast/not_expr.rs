use crate::parser::Position;

use super::Expr;

#[derive(Debug, Clone)]
pub struct NotExpr {
    pub position: Position,
    pub condition: Box<Expr>,
}
