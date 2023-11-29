use crate::{
    err::{Error, Result},
    token::{Keyword, Token, TokenType},
};
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "and" => Keyword::And,
    "class" => Keyword::Class,
    "else" => Keyword::Else,
    "false" => Keyword::False,
    "for" => Keyword::For,
    "fun" => Keyword::Fun,
    "if" => Keyword::If,
    "nil" => Keyword::Nil,
    "or" => Keyword::Or,
    "print" => Keyword::Print,
    "return" => Keyword::Return,
    "super" => Keyword::Super,
    "this" => Keyword::This,
    "true" => Keyword::True,
    "var" => Keyword::Var,
    "while" => Keyword::While,
};

pub fn scan_tokens(source: &str) -> Result<Vec<Token>> {
    let mut line: u32 = 1;
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::new(TokenType::LeftParen, "(", line)),
            ')' => tokens.push(Token::new(TokenType::RightParen, ")", line)),
            '{' => tokens.push(Token::new(TokenType::LeftBrace, "{", line)),
            '}' => tokens.push(Token::new(TokenType::RightBrace, "}", line)),
            ',' => tokens.push(Token::new(TokenType::Comma, ",", line)),
            '.' => tokens.push(Token::new(TokenType::Dot, ".", line)),
            '-' => tokens.push(Token::new(TokenType::Minus, "-", line)),
            '+' => tokens.push(Token::new(TokenType::Plus, "+", line)),
            ';' => tokens.push(Token::new(TokenType::Semicolon, ";", line)),
            '*' => tokens.push(Token::new(TokenType::Star, "*", line)),
            '!' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    tokens.push(Token::new(TokenType::BangEqual, "!=", line));
                }
                _ => tokens.push(Token::new(TokenType::Bang, "!", line)),
            },
            '=' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    tokens.push(Token::new(TokenType::EqualEqual, "==", line));
                }
                _ => tokens.push(Token::new(TokenType::Equal, "=", line)),
            },
            '<' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    tokens.push(Token::new(TokenType::LessEqual, "<=", line));
                }
                _ => tokens.push(Token::new(TokenType::Less, "<", line)),
            },
            '>' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    tokens.push(Token::new(TokenType::GreaterEqual, ">=", line));
                }
                _ => tokens.push(Token::new(TokenType::Greater, ">", line)),
            },
            '/' => match chars.peek() {
                Some('/') => loop {
                    match chars.peek() {
                        Some('\n') => break,
                        _ => chars.next(),
                    };
                },
                _ => tokens.push(Token::new(TokenType::Slash, "/", line)),
            },
            '"' => {
                let mut s = String::new();
                loop {
                    match chars.next() {
                        Some('"') => break,
                        Some(c) => {
                            if c == '\n' {
                                line += 1;
                            }
                            s.push(c);
                        }
                        None => return Err(Error::scan(line, "Unterminated string.")),
                    };
                }
                tokens.push(Token::new(TokenType::String(s.clone().into()), &s, line));
            }
            '0'..='9' => {
                let mut s = String::new();
                s.push(c);
                while let Some('0'..='9') = chars.peek() {
                    s.push(chars.next().unwrap());
                }
                if let Some('.') = chars.peek() {
                    s.push(chars.next().unwrap());
                    while let Some('0'..='9') = chars.peek() {
                        s.push(chars.next().unwrap());
                    }
                }
                tokens.push(Token::new(
                    TokenType::Number(
                        s.parse::<f64>()
                            .map_err(|_| Error::scan(line, "Invalid number."))?,
                    ),
                    &s,
                    line,
                ));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut s = String::from(c);
                while let Some(a) = chars.peek() {
                    if a.is_alphanumeric() || *a == '_' {
                        s.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match KEYWORDS.get(s.as_str()) {
                    Some(t) => tokens.push(Token::new(TokenType::Keyword(*t), &s, line)),
                    None => tokens.push(Token::new(
                        TokenType::Identifier(s.clone().into()),
                        &s,
                        line,
                    )),
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => line += 1,
            _ => return Err(Error::scan(line, "Unexpected character.")),
        }
    }
    Ok(tokens)
}
