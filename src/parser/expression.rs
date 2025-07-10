use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::parser::equality::equality;

pub fn expression(input: &Vec<Token>, index: usize) -> ExpressionType {
    equality(input, index)
}
