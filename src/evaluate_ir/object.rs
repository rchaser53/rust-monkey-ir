use std::fmt;

use evaluate_ir::environment::*;

use parser::expressions::*;
use parser::statements::*;

use llvm_sys::*;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(*mut LLVMValue),
    String(String),
    Boolean(*mut LLVMValue),
    Function(*mut LLVMValue, Vec<LLVMExpressionType>, LLVMExpressionType),
    Null,
    Error(String),
    BuildIn(BuildIn),
}

#[derive(Debug, Clone)]
pub enum BuildIn {
    Print,
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
            Object::Integer(_) => write!(f, "{}", "TODO"),
            Object::String(string) => write!(f, "{}", string),
            Object::Boolean(_) => write!(f, "{}", "TODO"),
            Object::Function(_, _, _) => write!(f, "{}", "TODO"),
            // Object::Function(ref func) => {
            //     let mut param_string = String::new();
            //     for (index, Identifier(ref string)) in func.parameters.iter().enumerate() {
            //         if index == 0 {
            //             param_string.push_str(&format!("{}", string));
            //         } else {
            //             param_string.push_str(&format!(", {}", string));
            //         }
            //     }
            //     let mut body_string = String::new();
            //     for (index, statement) in func.body.iter().enumerate() {
            //         if index == 0 {
            //             body_string.push_str(&format!("{}", statement.string()));
            //         } else {
            //             body_string.push_str(&format!(" {}", statement.string()));
            //         }
            //     }
            //     write!(f, "fn({}) {{ {} }}", param_string, body_string)
            // },
            Object::Null => write!(f, "Null"),
            Object::Error(string) => write!(f, "{}", string),
            Object::BuildIn(build_in) => match build_in {
                BuildIn::Print => write!(f, "print"),
            },
        }
    }
}
