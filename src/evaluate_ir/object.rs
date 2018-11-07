use std::fmt;

use evaluate_ir::environment::*;

use parser::expressions::*;
use parser::statements::*;

use llvm_sys::*;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(*mut LLVMValue),
    String(*mut LLVMValue),
    Boolean(*mut LLVMValue),
    Array(Vec<*mut LLVMValue>),
    Function(Function),
    Null,
    Error(String),
    BuildIn(BuildIn),
    Argument(*mut LLVMValue, LLVMExpressionType, u32),
}

#[derive(Debug, Clone)]
pub enum BuildIn {
    Printf,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub llvm_value: *mut LLVMValue,
    pub llvm_block: *mut LLVMBasicBlock,
    pub return_type: LLVMExpressionType,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(_) => write!(f, "{}", "TODO"),
            Object::String(_) => write!(f, "{}", "TODO"),
            Object::Boolean(_) => write!(f, "{}", "TODO"),
            Object::Function(_) => write!(f, "{}", "TODO"),
            Object::Array(_) => write!(f, "{}", "TODO"),
            Object::Null => write!(f, "Null"),
            Object::Error(string) => write!(f, "{}", string),
            Object::BuildIn(build_in) => match build_in {
                BuildIn::Printf => write!(f, "printf"),
            },
            Object::Argument(_, _, _) => write!(f, "TODO"),
        }
    }
}

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
