use crate::parser::Position;

use super::{Closure, Variable};

#[derive(Debug, Clone)]
pub struct Defun {
    pub position: Position,
    pub name: Variable,
    pub closure: Closure,
}
