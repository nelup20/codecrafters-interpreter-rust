use crate::lexer::token::Token;

pub enum ExpressionType<'a> {
    Literal(&'a Token),
    Unary(&'a Token, Box<ExpressionType<'a>>),
    Binary(Box<ExpressionType<'a>>, &'a Token, Box<ExpressionType<'a>>),
    Grouping(Box<ExpressionType<'a>>),
}

impl<'a> ExpressionType<'a> {
    pub fn as_string(&self) -> String {
        match self {
            ExpressionType::Literal(token) => token.as_string_for_parser(),

            ExpressionType::Unary(token, expression) => {
                format!(
                    "({} {})",
                    token.as_string_for_parser(),
                    expression.as_string()
                )
            }

            ExpressionType::Binary(left, operator, right) => {
                format!(
                    "({} {} {})",
                    operator.as_string_for_parser(),
                    left.as_string(),
                    right.as_string()
                )
            }

            ExpressionType::Grouping(expression) => format!("(group {})", expression.as_string()),
        }
    }
}
