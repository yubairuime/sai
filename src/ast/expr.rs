use super::literal::Literal;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable,

    Block,
    Defun,
    Declaration,
    Assignment,
    Funcall,
    Lambda,
}
