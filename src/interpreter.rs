use crate::{
    parser::Expr,
    token::TokenType,
    parser::Literal
};

#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub fn eval(e: Expr) -> Value {
    match e {
        Expr::Literal(l) => {
            match l {
                Literal::Number(n) => {
                    Value::Number(n)
                }
                Literal::String(s) => {
                    Value::String((*s).to_owned())
                }
                Literal::Boolean(b) => {
                    Value::Boolean(b)
                }
                Literal::Nil => {
                    Value::Nil
                }
            }
        }
        Expr::Grouping(e) => {
            eval(*e)
        }
        Expr::Unary { operator, right } => {
            let r = eval(*right);
            match operator.token_type {
                TokenType::Minus => {
                    match r {
                        Value::Number(n) => {
                            Value::Number(-n)
                        }
                        _ => {
                            panic!("unary minus not number");
                        }
                    }
                }
                TokenType::Bang => {
                    match r {
                        Value::Boolean(b) => {
                            Value::Boolean(!b)
                        }
                        _ => {
                            panic!("unary bang not boolean");
                        }
                    }
                }
                _ => {
                    panic!("unary not -");
                }
            }
        }
        Expr::Binary { left, operator, right } => {
            let l = eval(*left);
            let r = eval(*right);
            match operator.token_type {
                TokenType::Minus => {
                    match (l, r) {
                        (Value::Number(n1), Value::Number(n2)) => {
                            Value::Number(n1 - n2)
                        }
                        _ => {
                            panic!("binary minus not number");
                        }
                    }
                }
                TokenType::Plus => {
                    match (l, r) {
                        (Value::Number(n1), Value::Number(n2)) => {
                            Value::Number(n1 + n2)
                        }
                        (Value::String(s1), Value::String(s2)) => {
                            Value::String(s1 + &s2)
                        }
                        _ => {
                            panic!("binary plus not number or string");
                        }
                    }
                }
                TokenType::Star => {
                    match (l, r) {
                        (Value::Number(n1), Value::Number(n2)) => {
                            Value::Number(n1 * n2)
                        }
                        _ => {
                            panic!("binary star not number");
                        }
                    }
                }
                _ => {
                    panic!("binary not valid")
                }
            }
        }
    }
}
