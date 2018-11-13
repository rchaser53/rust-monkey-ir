use std::fmt;

use parser::expressions::*;

use llvm_sys::*;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(*mut LLVMValue),
    Boolean(*mut LLVMValue),
    String(LLVMExpressionType, *mut LLVMValue),
    Array(LLVMExpressionType, *mut LLVMValue),
    Function(Function),
    Null,
    Error(String),
    BuildIn(BuildIn),
    Argument(LLVMExpressionType, *mut LLVMValue, u32),
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
            Object::Integer(_) => write!(f, "Integer"),     // TODO
            Object::Boolean(_) => write!(f, "Boolean"), // TODO
            Object::String(llvm_type, _) => write!(f, "{}", llvm_type), // TODO
            Object::Array(child_type, _) => write!(f, "{}", child_type), // TODO
            Object::Function(_) => write!(f, "{}", "TODO"),
            Object::Null => write!(f, "Null"),
            Object::Error(string) => write!(f, "{}", string),
            Object::BuildIn(build_in) => match build_in {
                BuildIn::Printf => write!(f, "printf"),
            },
            Object::Argument(_, _, _) => write!(f, "TODO"),
        }
    }
}
