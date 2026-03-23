use super::Expr;

#[derive(Debug, Clone)]
pub struct Funcall {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}
