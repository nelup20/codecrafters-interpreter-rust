use crate::grammar::expression_type::ExpressionType;
use crate::lexer::reserved_keyword::ReservedKeyword;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use crate::lexer::token_type::TokenType::*;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

const RULES: [&str; 6] = [
    "equality",
    "comparison",
    "term",
    "factor",
    "unary",
    "primary",
];

pub fn get_rule_operators() -> HashMap<&'static str, Vec<TokenType>> {
    let mut rule_operators = HashMap::new();
    rule_operators.insert(RULES[0], vec![BangEqual, DoubleEqual]);
    rule_operators.insert(
        RULES[1],
        vec![GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual],
    );
    rule_operators.insert(RULES[2], vec![Minus, Plus]);
    rule_operators.insert(RULES[3], vec![Slash, Star]);
    rule_operators.insert(RULES[4], vec![Bang, Minus]);

    rule_operators
}

pub fn parse_rule<'a>(
    rules_depth: usize,
    input: &mut Peekable<Iter<'a, Token>>,
    rule_operators: &HashMap<&str, Vec<TokenType>>,
) -> ExpressionType<'a> {
    match rules_depth {
        ..4 => {
            let mut result = parse_rule(rules_depth + 1, input, rule_operators);

            while input.peek().is_some_and(|token| {
                rule_operators
                    .get(&RULES[rules_depth])
                    .unwrap()
                    .contains(&token.token_type)
            }) {
                let operator = input.next().unwrap();
                let right_operand = parse_rule(rules_depth + 1, input, rule_operators);
                result =
                    ExpressionType::Binary(Box::new(result), operator, Box::new(right_operand));
            }

            result
        }

        4 => {
            if input.peek().is_some_and(|token| {
                rule_operators
                    .get(&RULES[rules_depth])
                    .unwrap()
                    .contains(&token.token_type)
            }) {
                let operator = input.next().unwrap();
                // Same rule index so we can parse inner unary
                let right_operand = parse_rule(rules_depth, input, rule_operators);
                ExpressionType::Unary(operator, Box::new(right_operand))
            } else {
                parse_rule(rules_depth + 1, input, rule_operators)
            }
        }

        5 => {
            let next_token = input.next().unwrap();
            match next_token.token_type {
                Reserved(ReservedKeyword::False)
                | Reserved(ReservedKeyword::True)
                | Reserved(ReservedKeyword::Nil)
                | Number(_)
                | StringLiteral(_) => ExpressionType::Literal(next_token),

                LeftParen => {
                    let expression = parse_rule(0, input, rule_operators);
                    if input
                        .next()
                        .is_none_or(|token| token.token_type != RightParen)
                    {
                        eprintln!("Expected ')' after expression.");
                        std::process::exit(65);
                    }

                    ExpressionType::Grouping(Box::new(expression))
                }
                _ => {
                    eprintln!("Current parser doesn't support this token type.");
                    std::process::exit(65);
                }
            }
        }
        _ => panic!("Exceeded max depth"),
    }
}
