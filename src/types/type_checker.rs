use std::collections::HashMap;

use crate::{ast::{AndExpr, Assignment, Block, Closure, Declaration, Defun, Expr, Funcall, IfExpr, Literal, LiteralType, NotExpr, OrExpr, Program, TypeAnnotation, TypeExpr, Variable}, diagnostics::Diagnostic, parser::Position};

use super::Type;

type TypeEnv = HashMap<String, SymbolInfo>;

#[derive(Debug, Clone)]
pub struct TypeChecker {
    env: TypeEnv,
    diagnostics: Vec<Diagnostic>
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            env: TypeEnv::new(),
            diagnostics: vec![]
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<Diagnostic>> {
        self.diagnostics.clear();

        for expr in &program.expressions {
            let mut env = self.env.clone();
            self.check_expr(expr, &mut env);
            self.env = env;
        }

        if self.diagnostics.is_empty() {
            Ok(())
        } else {
            Err(self.diagnostics.clone())
        }
    }

    fn check_expr(&mut self, expr: &Expr, env: &mut TypeEnv) -> Type {
        match expr {
            Expr::Literal(literal) => self.check_literal(literal),
            Expr::Variable(variable) => self.check_variable(variable, env),
            Expr::Defun(defun) => self.check_defun(defun, env),
            Expr::Declaration(decl) => self.check_declaration(decl, env),
            Expr::Assignment(assignment) => self.check_assignment(assignment, env),
            Expr::Block(block) => self.check_block(block, env),
            Expr::Funcall(funcall) => self.check_funcall(funcall, env),
            Expr::Closure(closure) => self.check_closure(closure, env),
            Expr::IfExpr(if_expr) => self.check_if(if_expr, env),
            Expr::Or(or_expr) => self.check_or(or_expr, env),
            Expr::And(and_expr) => self.check_and(and_expr, env),
            Expr::Not(not_expr) => self.check_not(not_expr, env),
            Expr::Error => unreachable!()
        }
    }

    fn builtin_funcall_name<'a>(&self, funcall: &'a Funcall) -> Option<&'a str> {
        let Expr::Variable(variable) = funcall.callee.as_ref() else {
            return None;
        };

        match variable.value.as_str() {
            "+" | "-" | "*" | "/" | "=" | "<" | "<=" | ">" | ">=" | "len" | "nth" => Some(variable.value.as_str()),
            _ => None,
        }
    }

    fn check_builtin_funcall(&mut self, funcall: &Funcall, name: &str, env: &mut TypeEnv) -> Type {
        match name {
            "+" | "-" | "*" | "/" => self.check_numeric_builtin(funcall, env),
            "=" => self.check_equality_builtin(funcall, env),
            "<" | "<=" | ">" | ">=" => self.check_ordering_builtin(funcall, env),
            _ => unreachable!(),
        }
    }

    fn check_numeric_builtin(&mut self, funcall: &Funcall, env: &mut TypeEnv) -> Type {
        if funcall.args.len() < 2 {
            self.add_diagnostic(funcall.position.clone(), "arithmetic operators require at least 2 arguments");
            return Type::Error;
        }

        let mut saw_float = false;

        for arg in &funcall.args {
            match self.check_expr(arg, env) {
                Type::Int => {}
                Type::Float => saw_float = true,
                Type::Error => return Type::Error,
                _ => {
                    self.add_diagnostic(funcall.position.clone(), "arithmetic operators require all arguments to be ints or all arguments to be floats");
                    return Type::Error;
                }
            }
        }

        let first_ty = self.check_expr(&funcall.args[0], env);
        for arg in &funcall.args[1..] {
            let arg_ty = self.check_expr(arg, env);
            if arg_ty != first_ty {
                self.add_diagnostic(funcall.position.clone(), "arithmetic operators require all arguments to have the same numeric type");
                return Type::Error;
            }
        }

        if saw_float { Type::Float } else { Type::Int }
    }

    fn check_equality_builtin(&mut self, funcall: &Funcall, env: &mut TypeEnv) -> Type {
        if funcall.args.len() != 2 {
            self.add_diagnostic(funcall.position.clone(), "wrong number of arguments");
            return Type::Error;
        }

        let left_ty = self.check_expr(&funcall.args[0], env);
        let right_ty = self.check_expr(&funcall.args[1], env);

        if left_ty == Type::Error || right_ty == Type::Error {
            return Type::Error;
        }

        if left_ty != right_ty {
            self.add_diagnostic(funcall.position.clone(), "'=' requires both arguments to have the same type");
            return Type::Error;
        }

        if matches!(left_ty, Type::Function { .. }) {
            self.add_diagnostic(funcall.position.clone(), "'=' does not support functions");
            Type::Error
        } else {
            Type::Bool
        }
    }

    fn check_ordering_builtin(&mut self, funcall: &Funcall, env: &mut TypeEnv) -> Type {
        if funcall.args.len() != 2 {
            self.add_diagnostic(funcall.position.clone(), "wrong number of arguments");
            return Type::Error;
        }

        let left_ty = self.check_expr(&funcall.args[0], env);
        let right_ty = self.check_expr(&funcall.args[1], env);

        match (&left_ty, &right_ty) {
            (Type::Int, Type::Int) | (Type::Float, Type::Float) => Type::Bool,
            (Type::Error, _) | (_, Type::Error) => Type::Error,
            _ => {
                self.add_diagnostic(funcall.position.clone(), "comparison operators require two ints or two floats");
                Type::Error
            }
        }
    }



    fn check_literal(&self, literal: &Literal) -> Type {
        match literal.ty {
            LiteralType::Int => Type::Int,
            LiteralType::Float => Type::Float,
            LiteralType::Bool => Type::Bool,
            LiteralType::String => Type::String,
            LiteralType::Unit => Type::Unit,
        }
    }

    fn check_variable(&mut self, variable: &Variable, env: &TypeEnv) -> Type {
        if let Some(symbol_info) = env.get(&variable.value) {
            symbol_info.ty.clone()
        } else {
            self.add_diagnostic(variable.position.clone(), &format!("the variable {} doesn't exist", &variable.value));
            Type::Error
        }
    }

    fn check_type_annotation(&mut self, annotation: &TypeAnnotation) -> Type {
        match &annotation.value {
            TypeExpr::Named(name) => {
                match name.as_str() {
                    "int" => Type::Int,
                    "float" => Type::Float,
                    "bool" => Type::Bool,
                    "string" => Type::String,
                    "unit" => Type::Unit,
                    _ => {
                        self.add_diagnostic(annotation.position.clone(), "unknown type");
                        Type::Error
                    }
                }
            }
            TypeExpr::Function { params, ret } => {
                let params: Vec<Type> = params.into_iter()
                    .map(|param| self.check_type_annotation(&param))
                    .collect();
                let return_ty = self.check_type_annotation(&ret);

                Type::Function { params: params, return_ty: Box::new(return_ty) }
            }
        }
    }

    fn check_defun(&mut self, defun: &Defun, env: &mut TypeEnv) -> Type {
        let function_type = self.function_type_from_closure(&defun.closure);

        let symbol_info = SymbolInfo {
            ty: function_type,
            mutable: false,
        };
        env.insert(defun.name.value.clone(), symbol_info);
        self.check_closure(&defun.closure, env);

        Type::Unit
    }

    fn check_declaration(&mut self, declaration: &Declaration, env: &mut TypeEnv) -> Type {
        if self.is_variable_exist(&declaration.signature.name, env) {
            self.add_diagnostic(declaration.signature.name.position.clone(), "variable already exists");
            Type::Error
        } else {
            if self.check_type_annotation(&declaration.signature.ty) != self.check_expr(&declaration.value, env) {
                self.add_diagnostic(declaration.signature.ty.position.clone(), "unmatched type");
                Type::Error
            } else {
                let symbol_info = SymbolInfo {
                    ty: self.check_type_annotation(&declaration.signature.ty),
                    mutable: declaration.mutable
                };

                env.insert(declaration.signature.name.value.clone(), symbol_info);

                Type::Unit
            }
        }
    }

    fn check_assignment(&mut self, assignment: &Assignment, env: &mut TypeEnv) -> Type {
        if let Some(symbol_info) = env.get(&assignment.variable.value).cloned() {
            if !symbol_info.mutable {
                self.add_diagnostic(assignment.variable.position.clone(),
                                    &format!("the variable '{}' cannot be modified", &assignment.variable.value));
                Type::Error
            } else if symbol_info.ty != self.check_expr(&assignment.value, env) {
                self.add_diagnostic(assignment.variable.position.clone(), "unmatched type");
                Type::Error
            } else {
                Type::Unit
            }
        } else {
            self.add_diagnostic(assignment.variable.position.clone(),
                                &format!("variable '{}' doesn't exist", &assignment.variable.value));
            Type::Error
        }
    }

    fn check_block(&mut self, block: &Block, env: &mut TypeEnv) -> Type {
        let mut captured_env = env.clone();
        for expr in &block.contents[0..block.contents.len()] {
            self.check_expr(expr, &mut captured_env);
        }

        let last_expr = block.contents[block.contents.len()-1].clone();
        self.check_expr(&last_expr, &mut captured_env)
    }

    fn check_funcall(&mut self, funcall: &Funcall, env: &mut TypeEnv) -> Type {
        if let Some(name) = self.builtin_funcall_name(funcall) {
            return self.check_builtin_funcall(funcall, name, env);
        }

        match self.check_expr(&funcall.callee, env) {
            Type::Function { params, return_ty } => {
                if params.len() != funcall.args.len() {
                    self.add_diagnostic(funcall.callee.get_position(), "wrong number of arguments");
                    Type::Error
                } else {
                    for index in 0..params.len() {
                        if self.check_expr(&funcall.args[index], env) != params[index] {
                            self.add_diagnostic(funcall.args[index].get_position(), "unmatched type");
                        }
                    }

                    *return_ty
                }
            }
            Type::Error => Type::Error,
            _ => {
                self.add_diagnostic(funcall.callee.get_position(), "callee is not a function");
                Type::Error
            }
        }
    }

    fn check_closure(&mut self, closure: &Closure, env: &mut TypeEnv) -> Type {
        for param in &closure.params {
            let param_info = SymbolInfo {
                ty: self.check_type_annotation(&param.ty),
                mutable: true
            };

            env.insert(param.name.value.clone(), param_info);
        }

        self.check_block(&closure.body, env)
    }

    fn function_type_from_closure(&mut self, closure: &Closure) -> Type {
        let mut params_type: Vec<Type> = vec![];
        let return_ty = Box::new(self.check_type_annotation(&closure.return_type));

        for param in &closure.params {
            params_type.push(self.check_type_annotation(&param.ty));
        }

        Type::Function { params: params_type, return_ty: return_ty }
    }

    fn check_if(&mut self, if_expr: &IfExpr, env: &mut TypeEnv) -> Type {
        let if_type = self.check_block(&if_expr.branches[0].block, env);

        for branch in &if_expr.branches[1..] {
            if self.check_block(&branch.block, env) != if_type {
                self.add_diagnostic(branch.block.position.clone(), "unmatched type");
            }
        }

        if let Some(else_branch) = &if_expr.else_branch {
            if self.check_expr(&else_branch, env) != if_type {
                self.add_diagnostic(else_branch.get_position(), "unmatched type");
            }
        }

        if_type
    }

    fn check_or(&mut self, or_expr: &OrExpr, env: &mut TypeEnv) -> Type {
        for condition in &or_expr.conditions {
            if !self.is_boolean(condition, env) {
                self.add_diagnostic(condition.get_position(), "'or' requires booleans");
            }
        }

        Type::Bool
    }

    fn check_and(&mut self, and_expr: &AndExpr, env: &mut TypeEnv) -> Type {
        for condition in &and_expr.conditions {
            if !self.is_boolean(condition, env) {
                self.add_diagnostic(condition.get_position(), "'and' requires booleans");
            }
        }

        Type::Bool
    }

    fn check_not(&mut self, not_expr: &NotExpr, env: &mut TypeEnv) -> Type {
        if !self.is_boolean(&not_expr.condition, env) {
            self.add_diagnostic(not_expr.condition.get_position(), "'not' requires a boolean");
        }

        Type::Bool
    }

    fn is_boolean(&mut self, expr: &Expr, env: &mut TypeEnv) -> bool {
        match self.check_expr(expr, env) {
            Type::Bool => true,
            _ => false
        }
    }

    fn add_diagnostic(&mut self, position: Position, msg: &str) {
        self.diagnostics.push(Diagnostic::new(position, msg));
    }

    fn is_variable_exist(&self, variable: &Variable, env: &mut TypeEnv) -> bool {
        env.get(&variable.value).is_some()
    }
}

#[derive(Debug, Clone)]
struct SymbolInfo {
    pub ty: Type,
    pub mutable: bool,
}
