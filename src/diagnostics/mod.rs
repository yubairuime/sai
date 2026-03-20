use std::fmt;

use crate::parser::Position;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub position: Position,
    pub msg: String,
}

impl Diagnostic {
    pub fn new(position: Position, msg: &str) -> Self {
        Self {
            position: position,
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}\n{}",
            self.position.file, self.position.line, self.position.col, self.msg
        )
    }
}
