use std::io::{self, Write};

use crate::{
    diagnostics,
    interpreter::{self, Interpreter},
    parser::{Lexer, Parser}, types::TypeChecker,
};

pub fn repl() {
    println!("Welcome to Sai");

    loop {
        print!("sai > ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new("<stdin>".to_string(), &input);
        let tokens = lexer.tokenize();

        if !lexer.diagnostics.is_empty() {
            for diagnostic in lexer.diagnostics {
                println!("{}", diagnostic);
            }
        } else {
            let mut parser = Parser::new("<stdin>", tokens);
            let ast = parser.parse();

            match ast {
                Ok(program) => {
                    let mut type_checker = TypeChecker::new();

                    if let Err(diagnostics) = type_checker.check_program(&program) {
                        for diagnostic in diagnostics {
                            println!("{}", diagnostic)
                        }
                    } else {
                        
                    }
                }
                Err(diagnostics) => {
                    for diagnostic in diagnostics {
                        println!("{}", diagnostic);
                    }
                }
            }
        }
    }
}
