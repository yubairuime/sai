use super::expr::Expr;

#[derive(Debug, Clone)]
pub struct Block {
    pub content: Vec<Expr>,
}
