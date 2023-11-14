use crate::err;
use crate::token::{Token, TokenType};
use anyhow::Result;
use std::collections::HashMap;

// TODO: replace uwnrap
pub fn scan_tokens(source: &str) -> Result<Vec<Token>> {
    let KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ]);

    let mut line = 0;
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
                    match chars.peek() {
                        Some('"') => {
                            chars.next();
                            break;
                        }
                        Some('\n') => line += 1,
                        None => {
                            err::err(line, "Unterminated string.");
                            break;
                        }
                        _ => (),
                    };
                    let c = chars.next().unwrap();
                    s.push(c);
                }
                tokens.push(Token::new(TokenType::String(s.clone()), &s, line));
            }
            '0'..='9' => {
                let mut s = String::new();
                s.push(c);
                loop {
                    match chars.peek() {
                        Some('0'..='9') => {
                            s.push(chars.next().unwrap());
                        }
                        _ => break,
                    };
                }
                if let Some('.') = chars.peek() {
                    s.push(chars.next().unwrap());
                    loop {
                        match chars.peek() {
                            Some('0'..='9') => {
                                s.push(chars.next().unwrap());
                            }
                            _ => break,
                        };
                    }
                }
                tokens.push(Token::new(TokenType::Number(s.parse::<f64>()?), &s, line));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut s = String::from(c);
                loop {
                    match chars.peek() {
                        Some(a) if a.is_alphanumeric() => {
                            s.push(chars.next().unwrap());
                        }
                        _ => {
                            break;
                        }
                    }
                }
                match KEYWORDS.get(s.as_str()) {
                    Some(t) => tokens.push(Token::new(t.clone(), &s, line)),
                    None => tokens.push(Token::new(TokenType::Identifier(s.clone()), &s, line)),
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => line += 1,
            _ => {
                err::err(line, "Unexpected character.");
                break;
            }
        }
    }
    Ok(tokens)
}
