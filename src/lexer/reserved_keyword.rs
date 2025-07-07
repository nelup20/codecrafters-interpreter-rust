use crate::lexer::reserved_keyword::ReservedKeyword::*;

pub enum ReservedKeyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While
}

const RESERVED_KEYWORDS: [&str; 16] = [
    "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return", "super",
    "this", "true", "var", "while",
];

impl ReservedKeyword {
    pub fn as_string(&self) -> String {
        match self {
            And => "AND and null".to_string(),
            Class => "CLASS class null".to_string(),
            Else => "ELSE else null".to_string(),
            False => "FALSE false null".to_string(),
            For => "FOR for null".to_string(),
            Fun => "FUN fun null".to_string(),
            If => "IF if null".to_string(),
            Nil => "NIL nil null".to_string(),
            Or => "OR or null".to_string(),
            Print => "PRINT print null".to_string(),
            Return => "RETURN return null".to_string(),
            Super => "SUPER super null".to_string(),
            This => "THIS this null".to_string(),
            True => "TRUE true null".to_string(),
            Var => "VAR var null".to_string(),
            While => "WHILE while null".to_string(),
        }
    }

    pub fn is_reserved_keyword(input: &str) -> bool {
        RESERVED_KEYWORDS.contains(&input)
    }

    pub fn from_str(input: &str) -> Self {
        match input {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "for" => For,
            "fun" => Fun,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
            _ => unreachable!()
        }
    }
}