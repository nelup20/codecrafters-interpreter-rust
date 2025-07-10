use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType::{Slash, Star};
use crate::parser::unary::unary;
use std::iter::Peekable;
use std::slice::Iter;

pub fn factor<'a>(input: &mut Peekable<Iter<'a, Token>>) -> ExpressionType<'a> {
    let mut result = unary(input);

    while input
        .peek()
        .is_some_and(|token| [Slash, Star].contains(&token.token_type))
    {
        let operator = input.next().unwrap();
        let right_operand = unary(input);
        result = ExpressionType::Binary(Box::new(result), operator, Box::new(right_operand));
    }

    result
}
