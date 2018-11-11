use llvm_sys::*;

use evaluate_ir::object::*;
use ir::llvm_type::*;
use parser::expressions::*;

pub fn convert_llvm_type(expression_type: LLVMExpressionType) -> *mut LLVMType {
    match expression_type {
        LLVMExpressionType::Int => int32_type(),
        LLVMExpressionType::Boolean => int1_type(),
        LLVMExpressionType::String => int32_type(), // need to fix
        LLVMExpressionType::Null => int32_type(),   // need to fix
    }
}

pub fn unwrap_object(object: &mut Object) -> *mut LLVMValue {
    match *object {
        Object::Integer(llvm_value) => llvm_value,
        Object::String(llvm_value) => llvm_value,
        Object::Boolean(llvm_value) => llvm_value,
        Object::Function(ref func) => func.llvm_value,
        Object::Array(llvm_value) => llvm_value,
        _ => panic!("failed to unwrap object: {:?}", object),
    }
}

pub fn wrap_llvm_value(expression_type: LLVMExpressionType, llvm_value: *mut LLVMValue) -> Object {
    match expression_type {
        LLVMExpressionType::Int => Object::Integer(llvm_value),
        LLVMExpressionType::String => Object::Integer(llvm_value),
        LLVMExpressionType::Boolean => Object::Boolean(llvm_value),
        LLVMExpressionType::Null => Object::Null,
    }
}
