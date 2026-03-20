use std::io::{self, Write};

use crate::{
    diagnostics,
    interpreter::{self, Interpreter},
    parser::{Lexer, Parser},
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
            parser.parse();

            if !parser.diagnostics.is_empty() {
                for diagnostic in parser.diagnostics {
                    println!("{}", diagnostic);
                }
            } else {
                
            }

            // println!("{:#?}", tokens);
        }

        // let interpreter = Interpreter::new("<stdin>".to_string(), input);
        // interpreter.eval();
    }
}
