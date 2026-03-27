use crate::parser::Position;

use super::{
    AndExpr, Defun, IfExpr, NotExpr, OrExpr,
    assignment::{self, Assignment},
    block::Block,
    closure::Closure,
    declaration::Declaration,
    funcall::Funcall,
    literal::Literal,
    variable::{self, Variable},
};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(Variable),

    Defun(Defun),
    Declaration(Declaration),
    Assignment(Assignment),

    Block(Block),
    IfExpr(IfExpr),
    Funcall(Funcall),
    Closure(Closure),
    Or(OrExpr),
    And(AndExpr),
    Not(NotExpr),
    Error,
}

impl Expr {
    pub fn get_position(&self) -> Position {
        match self {
            Expr::Literal(literal) => literal.position.clone(),
            Expr::Variable(variable) => variable.position.clone(),
            Expr::Block(block) => block.position.clone(),
            Expr::Defun(defun) => defun.position.clone(),
            Expr::Declaration(decl) => decl.position.clone(),
            Expr::Assignment(assignment) => assignment.position.clone(),
            Expr::IfExpr(if_expr) => if_expr.position.clone(),
            Expr::Funcall(funcall) => funcall.position.clone(),
            Expr::Closure(closure) => closure.position.clone(),
            Expr::Or(or_expr) => or_expr.position.clone(),
            Expr::And(and_expr) => and_expr.position.clone(),
            Expr::Not(not_expr) => not_expr.position.clone(),
            Expr::Error => unreachable!(),
        }
    }
}
