use crate::lexer::reserved_keyword::ReservedKeyword::*;

#[derive(PartialEq, Debug)]
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
    pub fn as_string_for_parser(&self) -> String {
        match self {
            And => "and".to_string(),
            Class => "class".to_string(),
            Else => "else".to_string(),
            False => "false".to_string(),
            For => "for".to_string(),
            Fun => "fun".to_string(),
            If => "if".to_string(),
            Nil => "nil".to_string(),
            Or => "or".to_string(),
            Print => "print".to_string(),
            Return => "return".to_string(),
            Super => "super".to_string(),
            This => "this".to_string(),
            True => "true".to_string(),
            Var => "var".to_string(),
            While => "while".to_string(),
        }
    }
    
    pub fn as_string_for_lexer(&self) -> String {
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