use crate::{
    environment::Environment,
    err::{Error, Result},
    parser::{Expr, Literal, Stmt},
    token::TokenType,
};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(None),
        }
    }

    pub fn eval_stmt(&mut self, s: Stmt) -> Result<()> {
        match s {
            Stmt::Var {
                name: t,
                initializer: e,
            } => {
                let v = self.eval(e)?;
                self.environment.define(t.lexeme.as_ref(), v);
                Ok(())
            }
            Stmt::Print(e) => {
                let v = self.eval(e)?;
                println!("{}", v);
                Ok(())
            }
            Stmt::Expr(e) => {
                self.eval(e)?;
                Ok(())
            }
            Stmt::Block(b) => {
                // TODO: remove clone
                self.environment = Environment::new(Some(Box::new(self.environment.clone())));
                for s in b {
                    self.eval_stmt(s)?;
                }
                self.environment = *self.environment.enclosing.clone().unwrap();
                Ok(())
            }
        }
    }

    fn eval(&mut self, e: Expr) -> Result<Value> {
        match e {
            Expr::Literal(l) => match l {
                Literal::Number(n) => Ok(Value::Number(n)),
                Literal::String(s) => Ok(Value::String((*s).to_string())),
                Literal::Boolean(b) => Ok(Value::Boolean(b)),
                Literal::Nil => Ok(Value::Nil),
            },
            Expr::Grouping(e) => self.eval(*e),
            Expr::Unary { operator, right } => {
                let r = self.eval(*right);
                match operator.token_type {
                    TokenType::Minus => {
                        if let Ok(Value::Number(n)) = r {
                            return Ok(Value::Number(-n));
                        }
                        Err(Error::eval(operator.line, "Unary minus not number"))
                    }
                    TokenType::Bang => {
                        if let Ok(Value::Boolean(b)) = r {
                            return Ok(Value::Boolean(!b));
                        }
                        Err(Error::eval(operator.line, "Unary bang not boolean"))
                    }
                    _ => Err(Error::eval(operator.line, "Unary not valid")),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                match operator.token_type {
                    TokenType::Greater => {
                        if let (Ok(Value::Number(n1)), Ok(Value::Number(n2))) = (l, r) {
                            return Ok(Value::Boolean(n1 > n2));
                        }
                        Err(Error::eval(operator.line, "Binary greater not number"))
                    }
                    TokenType::GreaterEqual => {
                        if let (Ok(Value::Number(n1)), Ok(Value::Number(n2))) = (l, r) {
                            return Ok(Value::Boolean(n1 >= n2));
                        }
                        Err(Error::eval(
                            operator.line,
                            "Binary greater equal not number",
                        ))
                    }
                    TokenType::Less => {
                        if let (Ok(Value::Number(n1)), Ok(Value::Number(n2))) = (l, r) {
                            return Ok(Value::Boolean(n1 < n2));
                        }
                        Err(Error::eval(operator.line, "Binary less not number"))
                    }
                    TokenType::LessEqual => {
                        if let (Ok(Value::Number(n1)), Ok(Value::Number(n2))) = (l, r) {
                            return Ok(Value::Boolean(n1 <= n2));
                        }
                        Err(Error::eval(operator.line, "Binary less equal not number"))
                    }
                    TokenType::Minus => {
                        if let (Ok(Value::Number(n1)), Ok(Value::Number(n2))) = (l, r) {
                            return Ok(Value::Number(n1 - n2));
                        }
                        Err(Error::eval(operator.line, "Binary minus not number"))
                    }
                    TokenType::Plus => match (l, r) {
                        (Ok(Value::Number(n1)), Ok(Value::Number(n2))) => {
                            Ok(Value::Number(n1 + n2))
                        }
                        (Ok(Value::String(s1)), Ok(Value::String(s2))) => {
                            Ok(Value::String(s1 + &s2))
                        }
                        _ => Err(Error::eval(
                            operator.line,
                            "Binary plus not number or string",
                        )),
                    },
                    TokenType::Star => {
                        if let (Ok(Value::Number(n1)), Ok(Value::Number(n2))) = (l, r) {
                            return Ok(Value::Number(n1 * n2));
                        }
                        Err(Error::eval(operator.line, "Binary star not number"))
                    }
                    TokenType::Slash => {
                        if let (Ok(Value::Number(n1)), Ok(Value::Number(n2))) = (l, r) {
                            return Ok(Value::Number(n1 / n2));
                        }
                        Err(Error::eval(operator.line, "Binary slash not number"))
                    }
                    _ => Err(Error::eval(operator.line, "Binary expression not valid")),
                }
            }
            Expr::Variable(t) => {
                let name = t.lexeme;
                let value = self.environment.retrieve(&name);
                if let Some(v) = value {
                    Ok(v.clone())
                } else {
                    Err(Error::eval(t.line, "Undefined variable"))
                }
            }
            Expr::Assign { name, value } => {
                let v = self.eval(*value)?;
                self.environment
                    .assign(name.lexeme.as_ref(), v.clone())
                    .map_err(|_| Error::eval(name.line, "Assignment to undefined variable"))?;
                Ok(v)
            }
        }
    }
}
