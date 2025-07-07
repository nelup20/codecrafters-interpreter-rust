use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use std::io::Write;

pub struct Lexer {
    tokens: Vec<Token>,
    pub has_errors: bool,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            has_errors: false,
        }
    }

    pub fn scan_input(&mut self, input: &str) {
        let mut column = 0;
        let mut line = 1;

        let mut input_chars = input.chars().peekable();
        while let Some(char) = input_chars.next() {
            let mut token_type: Option<TokenType> = None;

            match char {
                '(' => token_type = Some(TokenType::LeftParen),
                ')' => token_type = Some(TokenType::RightParen),
                '{' => token_type = Some(TokenType::LeftBrace),
                '}' => token_type = Some(TokenType::RightBrace),
                '*' => token_type = Some(TokenType::Star),
                '+' => token_type = Some(TokenType::Plus),
                '-' => token_type = Some(TokenType::Minus),
                ',' => token_type = Some(TokenType::Comma),
                '.' => token_type = Some(TokenType::Dot),
                ';' => token_type = Some(TokenType::Semicolon),

                '!' => match input_chars.peek() {
                    Some('=') => {
                        token_type = Some(TokenType::BangEqual);
                        input_chars.next();
                    },
                    _ => {
                        token_type = Some(TokenType::Bang)
                    }
                },

                '=' => match input_chars.peek() {
                    Some('=') => {
                        token_type = Some(TokenType::DoubleEqual);
                        input_chars.next();
                    },
                    _ => {
                        token_type = Some(TokenType::Equal)
                    }
                },
                '\n' => line += 1,
                invalid_char => {
                    token_type = Some(TokenType::Invalid(invalid_char));
                    self.has_errors = true;
                }
            }

            match token_type {
                None => {}
                Some(token_type) => self.tokens.push(Token {
                    token_type,
                    line,
                    column,
                }),
            }

            column += 1;
        }

    }

    pub fn write_tokens_to_stream(
        &self,
        stdout_stream: &mut dyn Write,
        stderr_stream: &mut dyn Write,
    ) {
        for token in &self.tokens {
            match token.token_type {
                TokenType::Invalid(_) => {
                    writeln!(stderr_stream, "{}", token.as_string());
                }

                _ => {
                    writeln!(stdout_stream, "{}", token.as_string());
                }
            }
        }

        writeln!(stdout_stream, "EOF  null");
    }
}
