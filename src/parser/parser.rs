use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::parser::rules::{get_rule_operators, parse_rule};
use std::io::Write;

pub struct Parser {
    pub has_errors: bool,
    input: Vec<Token>,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            has_errors: false,
            input,
        }
    }

    pub fn parse(&self) -> ExpressionType {
        parse_rule(0, &mut self.input.iter().peekable(), &get_rule_operators())
    }

    pub fn print_result(
        &self,
        input: ExpressionType,
        stdout_stream: &mut dyn Write,
        stderr_stream: &mut dyn Write,
    ) {
        writeln!(stdout_stream, "{}", input.as_string()).unwrap()
    }
}
