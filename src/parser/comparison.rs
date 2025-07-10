use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType::{
    GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual,
};
use crate::parser::term::term;
use std::iter::Peekable;
use std::slice::Iter;

pub fn comparison<'a>(input: &mut Peekable<Iter<'a, Token>>) -> ExpressionType<'a> {
    let mut result = term(input);

    while input.peek().is_some_and(|token| {
        [GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual].contains(&token.token_type)
    }) {
        let operator = input.next().unwrap();
        let right_operand = term(input);
        result = ExpressionType::Binary(Box::new(result), operator, Box::new(right_operand));
    }

    result
}
