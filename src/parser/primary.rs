use crate::grammar::expression_type::ExpressionType;
use crate::lexer::reserved_keyword::ReservedKeyword;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use crate::parser::expression::expression;

pub fn primary(input: &Vec<Token>, index: usize) -> ExpressionType {
    let token = input.get(index).unwrap();
    match &token.token_type {
        TokenType::Reserved(ReservedKeyword::False)
        | TokenType::Reserved(ReservedKeyword::True)
        | TokenType::Reserved(ReservedKeyword::Nil)
        | TokenType::Number(_)
        | TokenType::StringLiteral(_) => ExpressionType::Literal(token),

        TokenType::LeftParen => {
            let expression = expression(input, index + 1);
            if input
                .get(index + 2)
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
