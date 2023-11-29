use crate::{
    err::{Error, Result},
    token::{Keyword, Token, TokenType},
};
use std::{iter::Peekable, rc::Rc, vec::IntoIter};

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(Rc<str>),
    Boolean(bool),
    Nil,
}

// TODO: consider restricting Token types
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
    Variable(Token),
    Assign {
        name: Token,
        value: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Expr(Expr),
    Block(Vec<Stmt>),
    Var { name: Token, initializer: Expr },
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while self.tokens.peek().is_some() {
            statements.push(self.declaration().map_err(|e| {
                self.synchronize();
                e
            })?);
        }
        Ok(statements)
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while let Some(t) = self.tokens.peek() {
            if t.token_type == TokenType::RightBrace {
                self.tokens.next();
                return Ok(statements);
            }
            statements.push(self.declaration().map_err(|e| {
                self.synchronize();
                e
            })?);
        }
        Err(Error::parse(
            self.tokens.peek().unwrap().line,
            "Expected } at end of block",
        ))
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if let Some(TokenType::Keyword(Keyword::Var)) = self.tokens.peek().map(|t| &t.token_type) {
            self.tokens.next();
            return self.var_declaration();
        }
        let s = self.statement()?;
        Ok(s)
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        if let Some(t) = self.tokens.next() {
            if let TokenType::Identifier(_) = t.token_type {
                if self.tokens.next().map(|t| t.token_type) == Some(TokenType::Equal) {
                    let e = self.expression()?;
                    if self.tokens.next().map(|t| t.token_type) == Some(TokenType::Semicolon) {
                        return Ok(Stmt::Var {
                            name: t,
                            initializer: e,
                        });
                    }
                    return Err(Error::parse(t.line, "Expected ; for var declaration"));
                }
                return Err(Error::parse(t.line, "Expected = for var declaration"));
            }
            return Err(Error::parse(
                t.line,
                "Expected identifier for var declaration",
            ));
        }
        panic!("Expected =");
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.tokens.peek().map(|t| &t.token_type) == Some(&TokenType::Keyword(Keyword::Print)) {
            let t = self.tokens.next().unwrap();
            let e = self.expression()?;
            if self.tokens.next().map(|t| t.token_type) == Some(TokenType::Semicolon) {
                return Ok(Stmt::Print(e));
            }
            return Err(Error::parse(t.line, "Expected ; for print statement"));
        }
        if self.tokens.peek().map(|t| &t.token_type) == Some(&TokenType::LeftBrace) {
            self.tokens.next();
            return Ok(Stmt::Block(self.block()?));
        }
        let e = self.expression()?;
        if self.tokens.next().map(|t| t.token_type) == Some(TokenType::Semicolon) {
            return Ok(Stmt::Expr(e));
        }
        // TODO: keep track of proper error line
        panic!("Expected ; for expression statement");
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let e = self.equality()?;
        if let Some(t) = self.tokens.peek() {
            if t.token_type == TokenType::Equal {
                let l = t.line;
                self.tokens.next();
                let value = self.assignment()?;
                if let Expr::Variable(name) = e {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    });
                }
                return Err(Error::parse(l, "Invalid assignment target"));
            }
        }
        Ok(e)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut e = self.comparison()?;
        while let Some(t) = self.tokens.peek() {
            match t.token_type {
                TokenType::BangEqual | TokenType::EqualEqual => {
                    let op = self.tokens.next().unwrap();
                    let r = self.comparison()?;
                    e = Expr::Binary {
                        left: Box::new(e),
                        operator: op,
                        right: Box::new(r),
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
                        left: Box::new(e),
                        operator: op,
                        right: Box::new(r),
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
                        left: Box::new(e),
                        operator: op,
                        right: Box::new(r),
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
                        left: Box::new(e),
                        operator: op,
                        right: Box::new(r),
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
                        right: Box::new(r),
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
        if let Some(t) = self.tokens.peek().cloned() {
            // TODO: consider &t.token_type
            match t.token_type {
                TokenType::Identifier(_) => {
                    self.tokens.next();
                    return Ok(Expr::Variable(t));
                }
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
                    if self.tokens.peek().map(|t| &t.token_type) == Some(&TokenType::RightParen) {
                        self.tokens.next();
                        return Ok(Expr::Grouping(Box::new(e)));
                    }
                    return Err(Error::parse(t.line, "Expected )"));
                }
                _ => {
                    return Err(Error::parse(
                        t.line,
                        "Unexpected token for a primary expression",
                    ));
                }
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
