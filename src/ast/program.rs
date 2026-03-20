use super::expr::Expr;

#[derive(Debug, Clone)]
pub struct Program {
    pub name: String,
    pub expressions: Vec<Expr>,
}
