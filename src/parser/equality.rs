use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType::{BangEqual, DoubleEqual};
use crate::parser::comparison::comparison;

pub fn equality(input: &Vec<Token>, index: usize) -> ExpressionType {
    if input
        .get(index + 1)
        .is_some_and(|token| [BangEqual, DoubleEqual].contains(&token.token_type))
    {
        let left_operand = comparison(input, index);
        let operator = input.get(index + 1).unwrap();
        let right_operand = comparison(input, index + 2);
        ExpressionType::Binary(Box::new(left_operand), operator, Box::new(right_operand))
    } else {
        comparison(input, index)
    }
}
