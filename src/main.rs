use cli::repl;

mod ast;
mod cli;
mod diagnostics;
mod interpreter;
mod parser;
mod types;

fn main() {
    repl();
}
