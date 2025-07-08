use crate::lexer::reserved_keyword::ReservedKeyword;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use std::io::Write;

pub struct Parser {
    // TODO: result should be an AST with Expr types
    pub result: Vec<String>,
    pub has_errors: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            result: Vec::new(),
            has_errors: false,
        }
    }

    pub fn parse(&mut self, input: Vec<Token>) {
        for token in input {
            match token.token_type {
                TokenType::Reserved(reserved_keyword) => match reserved_keyword {
                    ReservedKeyword::False => {
                        self.result.push("false".to_string());
                    }
                    ReservedKeyword::True => {
                        self.result.push("true".to_string());
                    }
                    ReservedKeyword::Nil => {
                        self.result.push("nil".to_string());
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    pub fn print_result(&self, stdout_stream: &mut dyn Write, stderr_stream: &mut dyn Write) {
        for expression in &self.result {
            writeln!(stdout_stream, "{expression}").unwrap();
        }
    }
}
