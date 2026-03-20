mod keyword;
mod lexer;
mod position;

use keyword::Keyword;
use lexer::TokenType;
pub use lexer::{Lexer, Token};
pub use position::Position;

use crate::{ast::Program, diagnostics::{self, Diagnostic}};

#[derive(Debug, Clone)]
pub struct Parser {
    file: String,
    tokens: Vec<Token>,
    index: usize,
    current_token: Option<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Parser {
    pub fn new(file: &str, tokens: Vec<Token>) -> Self {
        let current_token = tokens.get(0).cloned();
        Parser {
            file: file.to_string(),
            tokens: tokens,
            index: 0,
            current_token: current_token,
            diagnostics: vec![],
        }
    }

    pub fn parse(&mut self) -> Option<Program> {
        match self.read_all_tokentrees() {
            Ok(tokentrees) => {
                println!("{:#?}", tokentrees);

                None
            }
            Err(diagnostic) => {
                self.diagnostics.push(diagnostic);

                None
            }
        }
    }

    fn parse_tokentree(&self, tokentree: &TokenTree) {
        
    }

    fn parse_special_forms(&self, keyword: Keyword, tokentree: &Vec<TokenTree>) {
    }

    fn parse_block(&self) {}

    fn parse_defun(&self) {}

    fn parse_declaration(&self) {}

    fn parse_assignment(&self) {}

    fn parse_if(&self) {}

    fn parse_lambda(&self) {}

    fn advance(&mut self) {
        self.index += 1;
        self.current_token = self.tokens.get(self.index).cloned();
    }

    fn read_all_tokentrees(&mut self) -> Result<Vec<TokenTree>, Diagnostic> {
        let mut tokentrees: Vec<TokenTree> = vec![];

        while self.current_token.is_some() {
            tokentrees.push(self.read_tokentree()?);
        }

        Ok(tokentrees)
    }

    fn read_tokentree(&mut self) -> Result<TokenTree, Diagnostic> {
        if let Some(current_token) = self.current_token.clone() {
            if matches!(current_token.ty, TokenType::LParen) {
                let left_paren_position = current_token.position;
                self.advance();

                let mut list: Vec<TokenTree> = vec![];

                while let Some(current_token) = self.current_token.clone() {
                    if matches!(current_token.ty, TokenType::RParen) {
                        self.advance();
                        return Ok(TokenTree::List(list));
                    }

                    list.push(self.read_tokentree()?);
                }

                Err(Diagnostic::new(left_paren_position, "'(' was never closed"))
            } else if matches!(current_token.ty, TokenType::RParen) {
                Err(Diagnostic::new(current_token.position, "unmatch ')'"))
            } else {
                self.advance();
                Ok(TokenTree::Token(current_token))
            }
        } else {
            Ok(TokenTree::List(vec![]))
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenTree {
    Token(Token),
    List(Vec<TokenTree>),
}
