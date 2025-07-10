use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType::{BangEqual, DoubleEqual};
use crate::parser::comparison::comparison;
use std::iter::Peekable;
use std::slice::Iter;

pub fn equality<'a>(input: &mut Peekable<Iter<'a, Token>>) -> ExpressionType<'a> {
    let mut result = comparison(input);

    while input
        .peek()
        .is_some_and(|token| [BangEqual, DoubleEqual].contains(&token.token_type))
    {
        let operator = input.next().unwrap();
        let right_operand = comparison(input);
        result = ExpressionType::Binary(Box::new(result), operator, Box::new(right_operand));
    }

    result
}
