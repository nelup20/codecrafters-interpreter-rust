use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType::{Minus, Plus};
use crate::parser::factor::factor;

pub fn term(input: &Vec<Token>, index: usize) -> ExpressionType {
    if input
        .get(index + 1)
        .is_some_and(|token| [Minus, Plus].contains(&token.token_type))
    {
        let left_operand = factor(input, index);
        let operator = input.get(index + 1).unwrap();
        let right_operand = factor(input, index + 2);
        ExpressionType::Binary(Box::new(left_operand), operator, Box::new(right_operand))
    } else {
        factor(input, index)
    }
}