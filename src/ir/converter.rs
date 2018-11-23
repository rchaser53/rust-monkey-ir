use llvm_sys::*;

use evaluate_ir::object::*;
use ir::llvm_type::*;
use parser::expressions::*;

pub fn convert_llvm_type(expression_type: LLVMExpressionType) -> *mut LLVMType {
    match expression_type {
        LLVMExpressionType::Integer => int32_type(),
        LLVMExpressionType::Boolean => int1_type(),
        LLVMExpressionType::String => int32_type(), // need to fix
        LLVMExpressionType::Null => void_type(),
        LLVMExpressionType::Function => int32_type(), // need to fix
        LLVMExpressionType::Array(child_type, length) => {
            let mut child_type = convert_llvm_type(*child_type);
            array_type(child_type, length)
        }
        LLVMExpressionType::Call => void_type(),
    }
}

pub fn unwrap_object(object: &mut Object) -> *mut LLVMValue {
    match *object {
        Object::Integer(llvm_value) => llvm_value,
        Object::String(_, llvm_value) => llvm_value,
        Object::Boolean(llvm_value) => llvm_value,
        Object::Function(ref func) => func.llvm_value,
        Object::Array(_, llvm_value, _) => llvm_value,
        _ => panic!("failed to unwrap object: {:?}", object),
    }
}

pub fn wrap_llvm_value(expression_type: LLVMExpressionType, llvm_value: *mut LLVMValue) -> Object {
    match expression_type {
        LLVMExpressionType::Integer => Object::Integer(llvm_value),
        LLVMExpressionType::String => Object::Integer(llvm_value),
        LLVMExpressionType::Boolean => Object::Boolean(llvm_value),
        LLVMExpressionType::Array(child_type, array_length) => {
            Object::Array(*child_type, llvm_value, array_length)
        }
        _ => Object::Null,
    }
}

pub fn rewrap_llvm_value_ref(object: Object, llvm_value_ref: *mut LLVMValue) -> Object {
    match object {
        Object::Integer(_) => Object::Integer(llvm_value_ref),
        Object::String(llvm_expression_type, _) => {
            Object::String(llvm_expression_type, llvm_value_ref)
        }
        Object::Boolean(_) => Object::Boolean(llvm_value_ref),
        Object::Array(llvm_child_type, _, array_length) => {
            Object::Array(llvm_child_type, llvm_value_ref, array_length)
        }
        _ => object,
    }
}
