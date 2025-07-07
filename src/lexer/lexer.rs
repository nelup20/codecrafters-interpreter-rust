use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use std::io::Write;

pub struct Lexer {
    tokens: Vec<Token>,
    has_error: bool,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            has_error: false,
        }
    }

    pub fn scan_input(&mut self, input: &str) {
        let mut column = 0;
        let mut line = 0;

        for char in input.chars() {
            match char {
                '(' => self.tokens.push(Token {
                    token_type: TokenType::LeftParen,
                    column,
                    line,
                }),

                ')' => self.tokens.push(Token {
                    token_type: TokenType::RightParen,
                    column,
                    line,
                }),

                '\n' => line += 1,
                _ => {}
            }

            column += 1;
        }
    }

    pub fn write_tokens_to_stream(&self, output_stream: &mut dyn Write) {
        for token in &self.tokens {
            writeln!(output_stream, "{}", token.as_str());
        }

        writeln!(output_stream, "EOF  null");
    }
}
