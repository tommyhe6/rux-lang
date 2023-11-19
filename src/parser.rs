use crate::err::{Error, Result};
use crate::token::{Token, TokenType, Keyword};
use std::iter::Peekable;
use std::rc::Rc;
use std::vec::IntoIter;

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(Rc<str>),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Grouping(Box<Expr>),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut e = self.comparison()?;
        while let Some(t) = self.tokens.peek() {
            match t.token_type {
                TokenType::BangEqual | TokenType::EqualEqual => {
                    let op = self.tokens.next().unwrap();
                    let r = self.comparison()?;
                    e = Expr::Binary {
                        left: Box::from(e),
                        operator: op,
                        right: Box::from(r),
                    }
                }
                _ => break,
            }
        }
        Ok(e)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut e = self.term()?;
        while let Some(t) = self.tokens.peek() {
            match t.token_type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    let op = self.tokens.next().unwrap();
                    let r = self.term()?;
                    e = Expr::Binary {
                        left: Box::from(e),
                        operator: op,
                        right: Box::from(r),
                    }
                }
                _ => break,
            }
        }
        Ok(e)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut e = self.factor()?;
        while let Some(t) = self.tokens.peek() {
            match t.token_type {
                TokenType::Minus | TokenType::Plus => {
                    let op = self.tokens.next().unwrap();
                    let r = self.factor()?;
                    e = Expr::Binary {
                        left: Box::from(e),
                        operator: op,
                        right: Box::from(r),
                    }
                }
                _ => break,
            }
        }
        Ok(e)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut e = self.unary()?;
        while let Some(t) = self.tokens.peek() {
            match t.token_type {
                TokenType::Slash | TokenType::Star => {
                    let op = self.tokens.next().unwrap();
                    let r = self.unary()?;
                    e = Expr::Binary {
                        left: Box::from(e),
                        operator: op,
                        right: Box::from(r),
                    }
                }
                _ => break,
            }
        }
        Ok(e)
    }

    fn unary(&mut self) -> Result<Expr> {
        if let Some(t) = self.tokens.peek() {
            match t.token_type {
                TokenType::Bang | TokenType::Minus => {
                    let op = self.tokens.next().unwrap();
                    let r = self.unary()?;
                    return Ok(Expr::Unary {
                        operator: op,
                        right: Box::from(r),
                    });
                }
                _ => {
                    return self.primary();
                }
            }
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        if let Some(t) = self.tokens.peek() {
            match t.token_type {
                TokenType::Keyword(Keyword::False) => {
                    self.tokens.next();
                    return Ok(Expr::Literal(Literal::Boolean(false)));
                }
                TokenType::Keyword(Keyword::True) => {
                    self.tokens.next();
                    return Ok(Expr::Literal(Literal::Boolean(true)));
                }
                TokenType::Keyword(Keyword::Nil) => {
                    self.tokens.next();
                    return Ok(Expr::Literal(Literal::Nil));
                }
                TokenType::Number(n) => {
                    self.tokens.next();
                    return Ok(Expr::Literal(Literal::Number(n)));
                }
                TokenType::String(ref s) => {
                    let temp = s.clone();
                    self.tokens.next();
                    return Ok(Expr::Literal(Literal::String(temp)));
                }
                TokenType::LeftParen => {
                    self.tokens.next();
                    let e = self.expression()?;
                    if let Some(t) = self.tokens.peek() {
                        if t.token_type == TokenType::RightParen {
                            self.tokens.next();
                            return Ok(Expr::Grouping(Box::from(e)));
                        } else {
                            panic!("Expected )");
                        }
                    }
                    panic!("Expected )");
                }
                _ => panic!("primary"),
            }
        }
        panic!("Expected expression")
    }

    fn synchronize(&mut self) {
        while let Some(t) = self.tokens.peek() {
            match t.token_type {
                TokenType::Semicolon => {
                    return;
                }
                TokenType::Keyword(Keyword::Class)
                | TokenType::Keyword(Keyword::Fun)
                | TokenType::Keyword(Keyword::Var)
                | TokenType::Keyword(Keyword::For)
                | TokenType::Keyword(Keyword::If)
                | TokenType::Keyword(Keyword::While)
                | TokenType::Keyword(Keyword::Print)
                | TokenType::Keyword(Keyword::Return) => {
                    return;
                }
                _ => {
                    self.tokens.next();
                }
            }
        }
    }
}
