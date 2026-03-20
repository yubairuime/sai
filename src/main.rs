use cli::repl;

mod ast;
mod cli;
mod diagnostics;
mod interpreter;
mod parser;

fn main() {
    repl();
}
