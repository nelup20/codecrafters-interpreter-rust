use crate::grammar::expression_type::ExpressionType;
use crate::lexer::token::Token;
use crate::parser::equality::equality;
use std::iter::Peekable;
use std::slice::Iter;

pub fn expression<'a>(input: &mut Peekable<Iter<'a, Token>>) -> ExpressionType<'a> {
    equality(input)
}
