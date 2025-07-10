use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType::{Minus, Plus};
use crate::parser::factor::factor;
use std::iter::Peekable;
use std::slice::Iter;

pub fn term<'a>(input: &mut Peekable<Iter<'a, Token>>) -> ExpressionType<'a> {
    let mut result = factor(input);

    while input
        .peek()
        .is_some_and(|token| [Minus, Plus].contains(&token.token_type))
    {
        let operator = input.next().unwrap();
        let right_operand = factor(input);
        result = ExpressionType::Binary(Box::new(result), operator, Box::new(right_operand));
    }

    result
}
