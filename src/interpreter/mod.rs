use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub file: String,
    pub source_code: String,
}

impl Interpreter {
    pub fn new(file: String, source_code: String) -> Self {
        Self {
            file: file,
            source_code: source_code,
        }
    }
    pub fn eval(&self) {}
}
