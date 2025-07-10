use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType::{Bang, Minus};
use crate::parser::primary::primary;

pub fn unary(input: &Vec<Token>, index: usize) -> ExpressionType {
    if input
        .get(index)
        .is_some_and(|token| [Bang, Minus].contains(&token.token_type))
    {
        let operator = input.get(index).unwrap();
        let right_operand = unary(input, index + 1);
        ExpressionType::Unary(operator, Box::new(right_operand))
    } else {
        primary(input, index)
    }
}
