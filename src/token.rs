use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier(Rc<str>),
    String(Rc<str>),
    Number(f64),
    // Keywords
    Keyword(Keyword),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Rc<str>,
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: u32) -> Self {
        Self {
            token_type,
            lexeme: Rc::from(lexeme),
            line,
        }
    }
}

