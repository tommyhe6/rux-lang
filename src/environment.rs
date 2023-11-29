use crate::interpreter::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct AssignError;
impl std::fmt::Display for AssignError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Assignment to undefined variable")
    }
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    pub fn retrieve(&self, name: &str) -> Option<&Value> {
        if self.values.contains_key(name) {
            return self.values.get(name);
        }
        if let Some(en) = &self.enclosing {
            return en.retrieve(name);
        }
        None
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), AssignError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return Ok(());
        }
        if let Some(en) = &mut self.enclosing {
            return en.assign(name, value);
        }
        Err(AssignError)
    }
}
