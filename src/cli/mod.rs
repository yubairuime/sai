mod repl;

pub use repl::repl;

use crate::{diagnostics::{self, Diagnostic, render_diagnostics}, interpreter::{Interpreter, Value}, parser::{Lexer, Parser}, types::TypeChecker};

pub fn run_source_in_session(file: &str, source: &str, type_checker: &mut TypeChecker, interpreter: &mut Interpreter) -> Result<Value, Vec<Diagnostic>> {
    let mut lexer = Lexer::new(file.to_string(), source);
    let tokens = lexer.tokenize();

    if !lexer.diagnostics.is_empty() {
        return Err(lexer.diagnostics);
    }

    let mut parser = Parser::new(file, tokens);
    let program = parser.parse()?;

    type_checker.check_program(&program)?;
    interpreter.eval_program(&program)
}

pub fn print_diagnostics(source: Option<&str>, diagnostics: &[Diagnostic], use_color: bool) {
    for rendered in render_diagnostics(source, diagnostics, use_color) {
        println!("{}", rendered);
    }
}
