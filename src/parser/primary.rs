use crate::grammar::expression_type::ExpressionType;
use crate::lexer::reserved_keyword::ReservedKeyword;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use crate::parser::expression::expression;
use std::iter::Peekable;
use std::slice::Iter;

pub fn primary<'a>(input: &mut Peekable<Iter<'a, Token>>) -> ExpressionType<'a> {
    let next_token = input.next().unwrap();
    match next_token.token_type {
        TokenType::Reserved(ReservedKeyword::False)
        | TokenType::Reserved(ReservedKeyword::True)
        | TokenType::Reserved(ReservedKeyword::Nil)
        | TokenType::Number(_)
        | TokenType::StringLiteral(_) => ExpressionType::Literal(next_token),

        TokenType::LeftParen => {
            let expression = expression(input);
            if input
                .next()
                .is_none_or(|token| token.token_type != TokenType::RightParen)
            {
                panic!("Expected ')' after expression.");
            }

            ExpressionType::Grouping(Box::new(expression))
        }
        _ => {
            unreachable!("Current parser doesn't support this token type.")
        }
    }
}
