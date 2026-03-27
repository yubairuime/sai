use crate::{
    ast::{AndExpr, Assignment, Block, Closure, Declaration, Defun, Expr, Funcall, IfExpr, NotExpr, OrExpr, Program},
    diagnostics::Diagnostic,
};

use super::Position;

#[derive(Debug, Clone)]
pub struct ASTValidator {
    diagnostics: Vec<Diagnostic>,
}

impl ASTValidator {
    pub fn new() -> Self {
        Self { diagnostics: vec![] }
    }

    pub fn validate(&mut self, program: &Program) -> Result<(), Vec<Diagnostic>> {
        for expr in &program.expressions {
            self.validate_expr(expr);
        }

        if self.diagnostics.is_empty() { Ok(()) } else { Err(self.diagnostics.clone()) }
    }

    fn validate_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(_) => (),
            Expr::Variable(_) => (),
            Expr::Block(block) => self.validate_block(block),
            Expr::Defun(defun) => self.validate_defun(defun),
            Expr::Declaration(decl) => self.validate_declaration(decl),
            Expr::Assignment(assignment) => self.validate_assignment(assignment),
            Expr::IfExpr(if_expr) => self.validate_if_expr(if_expr),
            Expr::Funcall(funcall) => self.validate_funcall(funcall),
            Expr::Closure(closure) => self.validate_closure(closure),
            Expr::Or(or_expr) => self.validate_or_expr(or_expr),
            Expr::And(and_expr) => self.validate_and_expr(and_expr),
            Expr::Not(not_expr) => self.validate_not_expr(not_expr),
            Expr::Error => unreachable!(),
        }
    }

    fn validate_block(&mut self, block: &Block) {
        for expr in &block.contents {
            self.validate_expr(expr);
        }
    }

    fn validate_defun(&mut self, defun: &Defun) {
        self.validate_closure(&defun.closure);
    }

    fn validate_declaration(&mut self, declaration: &Declaration) {
        if let Err(e) = self.validate_value(&(*declaration.value)) {
            self.diagnostics.push(e);
        }
    }

    fn validate_assignment(&mut self, assignment: &Assignment) {
        self.validate_expr(&assignment.value);
    }

    fn validate_if_expr(&mut self, if_expr: &IfExpr) {
        if if_expr.branches.is_empty() {
            self.add_diagnostic(if_expr.position.clone(), "'if' has no branch");
        } else {
            for branch in &if_expr.branches {
                self.validate_condition(&branch.condition);
                self.validate_block(&branch.block);
            }

            if let Some(else_branch) = &if_expr.else_branch {
                if let Expr::Block(block) = *(else_branch.clone()) {
                    self.validate_block(&block);
                } else {
                    unreachable!()
                }
            }
        }
    }
    fn validate_funcall(&mut self, funcall: &Funcall) {
        self.validate_callee(&funcall.callee);

        for arg in &funcall.args {
            if let Err(e) = self.validate_value(arg) {
                self.diagnostics.push(e);
            }
        }
    }

    fn validate_callee(&mut self, callee: &Expr) {
        if let Err(e) = self.validate_value(callee) {
            self.diagnostics.push(e);
        } else {
            match callee {
                Expr::Or(or_expr) => self.add_diagnostic(or_expr.position.clone(), "unexpected 'or'"),
                Expr::And(and_expr) => self.add_diagnostic(and_expr.position.clone(), "unexpected 'and'"),
                Expr::Not(not_expr) => self.add_diagnostic(not_expr.position.clone(), "unexpected 'and'"),
                Expr::Error => unreachable!(),
                _ => (),
            }
        }
    }

    fn validate_closure(&mut self, closure: &Closure) {
        self.validate_block(&closure.body);
    }
    fn validate_or_expr(&mut self, or_expr: &OrExpr) {
        for condition in &or_expr.conditions {
            self.validate_condition(condition);
        }
    }
    fn validate_and_expr(&mut self, and_expr: &AndExpr) {
        for condition in &and_expr.conditions {
            self.validate_condition(condition);
        }
    }
    fn validate_not_expr(&mut self, not_expr: &NotExpr) {
        self.validate_condition(&not_expr.condition);
    }

    fn validate_condition(&mut self, condition: &Expr) {
        if let Err(e) = self.validate_value(condition) {
            self.diagnostics.push(e);
        } else {
            if let Expr::Closure(closure) = condition {
                self.add_diagnostic(closure.position.clone(), "unexpected closure")
            }
        }
    }

    fn validate_value(&self, expr: &Expr) -> Result<(), Diagnostic> {
        match expr {
            Expr::Declaration(decl) => Err(Diagnostic::new(decl.position.clone(), "unexpected variable declaration")),
            Expr::Assignment(assignment) => Err(Diagnostic::new(assignment.position.clone(), "unexpected assignment")),
            Expr::Defun(defun) => Err(Diagnostic::new(defun.position.clone(), "unexpected function declaration")),
            Expr::Error => unreachable!(),
            _ => Ok(()),
        }
    }

    fn add_diagnostic(&mut self, position: Position, msg: &str) {
        self.diagnostics.push(Diagnostic::new(position, msg));
    }
}
