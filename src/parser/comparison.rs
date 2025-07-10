use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType::{
    GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual,
};
use crate::parser::term::term;

pub fn comparison(input: &Vec<Token>, index: usize) -> ExpressionType {
    if input.get(index + 1).is_some_and(|token| {
        [GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual].contains(&token.token_type)
    }) {
        let left_operand = term(input, index);
        let operator = input.get(index + 1).unwrap();
        let right_operand = term(input, index + 2);
        ExpressionType::Binary(Box::new(left_operand), operator, Box::new(right_operand))
    } else {
        term(input, index)
    }
}
