use crate::parser::Position;

use super::{block::Block, expr::Expr};

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub position: Position,
    pub branches: Vec<Branch>,
    pub else_branch: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub condition: Box<Expr>,
    pub block: Block,
}
