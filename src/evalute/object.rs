use std::fmt;
use std::collections::HashMap;

use parser::statements::*;
use parser::expressions::*;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Function{
      parameters: Vec<Identifier>,
      body: BlockStatement,
      env: Environment,
    },
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(int) => write!(f, "{}", int.to_string()),
            Object::Boolean(boolean) => write!(f, "{}", boolean.to_string()),
            Object::Function{ parameters, body, env } => {
              let mut ret_string = String::new();
              for (index, parameter) in parameters.iter().enumerate() {
                  if index != 0 {
                      ret_string = ret_string + ", "
                  }
                  ret_string = ret_string + &parameter.0;
              }
              for statement in body {
                  ret_string = ret_string + " " + &statement.string();
              }
              write!(f, "fn({}) {{}}", ret_string)
            },
            Object::Null => write!(f, "Null"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
  pub store: HashMap<String, Object>
}

impl Environment {
  pub fn new() -> Self {
    Environment{
      store: HashMap::new()
    }
  }

  pub fn get(&self, name: &str) -> Object {
    self.store[name].clone()
  }

  pub fn set(&mut self, name: String, value: Object) -> Object {
    self.store.insert(name, value.clone());
    value
  }
}
