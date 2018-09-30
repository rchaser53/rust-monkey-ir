use std::collections::HashMap;
use std::fmt;

use parser::expressions::*;
use parser::statements::*;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    String(String),
    Boolean(bool),
    Function {
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
            Object::String(string) => write!(f, "{}", string),
            Object::Boolean(boolean) => write!(f, "{}", boolean.to_string()),
            Object::Function {
                parameters,
                body,
                env: _,
            } => {
                let mut ret_string = String::new();
                for (index, Identifier(ref string)) in parameters.iter().enumerate() {
                    if index == 0 {
                        ret_string.push_str(&format!("{}", string));
                    } else {
                        ret_string.push_str(&format!(", {}", string));
                    }
                }
                for statement in body {
                    ret_string = ret_string + " " + &statement.string();
                }
                write!(f, "fn({}) {{}}", ret_string)
            }
            Object::Null => write!(f, "Null"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
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
