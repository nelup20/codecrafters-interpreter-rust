use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType::{Bang, Minus};
use crate::parser::primary::primary;
use std::iter::Peekable;
use std::slice::Iter;

pub fn unary<'a>(input: &mut Peekable<Iter<'a, Token>>) -> ExpressionType<'a> {
    if input
        .peek()
        .is_some_and(|token| [Bang, Minus].contains(&token.token_type))
    {
        let operator = input.next().unwrap();
        let right_operand = unary(input);
        ExpressionType::Unary(operator, Box::new(right_operand))
    } else {
        primary(input)
    }
}
