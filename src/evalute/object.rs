use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Integer(int) => write!(f, "{}", int.to_string()),
            Object::Boolean(boolean) => write!(f, "{}", boolean.to_string()),
            Object::Null => write!(f, "Null"),
        }
    }
}

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
