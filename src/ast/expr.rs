use crate::parser::Position;

use super::{
    AndExpr, Defun, IfExpr, NotExpr, OrExpr, assignment::Assignment, block::Block,
    closure::Closure, declaration::Declaration, funcall::Funcall, literal::Literal,
    variable::Variable,
};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(Variable),

    Block(Block),
    Defun(Defun),
    Declaration(Declaration),
    Assignment(Assignment),
    IfExpr(IfExpr),
    Funcall(Funcall),
    Closure(Closure),
    Or(OrExpr),
    And(AndExpr),
    Not(NotExpr),
    Error,
}
