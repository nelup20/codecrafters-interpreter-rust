use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType::{Slash, Star};
use crate::parser::unary::unary;

pub fn factor(input: &Vec<Token>, index: usize) -> ExpressionType {
    if input
        .get(index + 1)
        .is_some_and(|token| [Slash, Star].contains(&token.token_type))
    {
        let left_operand = unary(input, index);
        let operator = input.get(index + 1).unwrap();
        let right_operand = unary(input, index + 2);
        ExpressionType::Binary(Box::new(left_operand), operator, Box::new(right_operand))
    } else {
        unary(input, index)
    }
}
