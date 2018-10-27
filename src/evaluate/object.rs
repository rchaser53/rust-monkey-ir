use std::fmt;

use evaluate::environment::*;

use parser::expressions::*;
use parser::statements::*;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    String(String),
    Boolean(bool),
    Function(Function),
    Null,
    Error(String),
    BuildIn(BuildIn),
}

#[derive(Debug, Clone)]
pub enum BuildIn {
    Print,
}

#[derive(Debug, Clone)]
pub struct BuildInPrint {}
impl BuildInPrint {
    pub fn new() -> Self {
        BuildInPrint {}
    }
    pub fn print(&self, print_str: &str) {
        println!("{}", print_str);
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Environment,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(int) => write!(f, "{}", int.to_string()),
            Object::String(string) => write!(f, "{}", string),
            Object::Boolean(boolean) => write!(f, "{}", boolean.to_string()),
            Object::Function(ref func) => {
                let mut param_string = String::new();
                for (index, Identifier(ref string)) in func.parameters.iter().enumerate() {
                    if index == 0 {
                        param_string.push_str(&format!("{}", string));
                    } else {
                        param_string.push_str(&format!(", {}", string));
                    }
                }
                let mut body_string = String::new();
                for (index, statement) in func.body.iter().enumerate() {
                    if index == 0 {
                        body_string.push_str(&format!("{}", statement.string()));
                    } else {
                        body_string.push_str(&format!(" {}", statement.string()));
                    }
                }
                write!(f, "fn({}) {{ {} }}", param_string, body_string)
            }
            Object::Null => write!(f, "Null"),
            Object::Error(string) => write!(f, "{}", string),
            Object::BuildIn(build_in) => match build_in {
                BuildIn::Print => write!(f, "print"),
            },
        }
    }
}
