use std::io::{self, Write};

use rustyline::{DefaultEditor, error::ReadlineError};

use crate::{
    cli::{print_diagnostics, run_source_in_session}, diagnostics, interpreter::{self, Interpreter, Value}, parser::{Lexer, Parser}, types::TypeChecker
};

pub fn repl() {
    println!("Welcome to Sai");

    let mut type_checker = TypeChecker::new();
    let mut interpreter = Interpreter::new("<stdin>".to_string(), String::new());
    let mut editor = DefaultEditor::new().unwrap();

    loop {
        let input = match read_submission(&mut editor) {
            ReadOutcome::Submission(input) => input,
            ReadOutcome::Cancelled => continue,
            ReadOutcome::Exit => {
                println!();
                return;
            }
        };

        match run_source_in_session("<stdin>", &input, &mut type_checker, &mut interpreter) {
            Ok(Value::Unit) => {}
            Ok(value) => println!("{value}"),
            Err(diagnostics) => print_diagnostics(Some(&input), &diagnostics, true),
        }
    }
}

enum ReadOutcome {
    Submission(String),
    Cancelled,
    Exit,
}

fn read_submission(editor: &mut DefaultEditor) -> ReadOutcome {
    let mut buffer = String::new();
    let mut prompt = "sai > ";

    loop {
        match editor.readline(prompt) {
            Ok(line) => {
                if !buffer.is_empty() || !line.trim().is_empty() {
                    buffer.push_str(&line);
                    buffer.push_str("\n");
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!();
                return ReadOutcome::Cancelled;
            }
            Err(ReadlineError::Eof) => {
                if buffer.trim().is_empty() {
                    return ReadOutcome::Exit;
                } else {
                    return ReadOutcome::Submission(buffer);
                }
            }
            Err(e) => {
                eprintln!("failed to read input: {e}");
                return ReadOutcome::Exit;
            }
        }

        return ReadOutcome::Submission(buffer);
    }
}
