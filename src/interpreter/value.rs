use std::{fmt, rc::Rc};

use crate::ast::Block;

use super::runtime_env::EnvRef;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Unit,
    Function(Rc<FunctionValue>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{value}"),
            Value::Float(value) => write!(f, "{value}"),
            Value::Bool(value) => write!(f, "{value}"),
            Value::String(value) => write!(f, "\"{value}\""),
            Value::Unit => write!(f, "unit"),
            Value::Function(_) => write!(f, "<function>"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionValue {
    pub params: Vec<String>,
    pub body: Block,
    pub env: EnvRef,
}
