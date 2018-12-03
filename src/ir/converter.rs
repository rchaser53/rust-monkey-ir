use llvm_sys::*;

use evaluate_ir::object::*;
use ir::llvm_type::*;
use parser::expressions::*;

pub fn get_llvm_type_from_object(object: &mut Object) -> *mut LLVMType {
    match *object {
        Object::Integer(_) => int32_type(),
        Object::String(_, length) => array_type(int8_type(), length),
        Object::Boolean(_) => int1_type(),
        Object::Function(_) => int1_type(), // need to fix
        Object::Array(ref child_type, _, length) => {
            let mut child_type = convert_llvm_type(child_type.clone());
            array_type(child_type, length)
        }
        _ => panic!("failed to get llvm_type: {:?}", object),
    }
}

pub fn convert_llvm_type(expression_type: LLVMExpressionType) -> *mut LLVMType {
    match expression_type {
        LLVMExpressionType::Integer => int32_type(),
        LLVMExpressionType::Boolean => int1_type(),
        LLVMExpressionType::String(length) => array_type(int8_type(), length),
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
        Object::String(llvm_value, _) => llvm_value,
        Object::Boolean(llvm_value) => llvm_value,
        Object::Function(ref func) => func.llvm_value,
        Object::Array(_, llvm_value, _) => llvm_value,
        _ => panic!("failed to unwrap object: {:?}", object),
    }
}

pub fn wrap_llvm_value(expression_type: LLVMExpressionType, llvm_value: *mut LLVMValue) -> Object {
    match expression_type {
        LLVMExpressionType::Integer => Object::Integer(llvm_value),
        LLVMExpressionType::String(length) => Object::String(llvm_value, length),
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
        Object::String(_, length) => Object::String(llvm_value_ref, length),
        Object::Boolean(_) => Object::Boolean(llvm_value_ref),
        Object::Array(llvm_child_type, _, array_length) => {
            Object::Array(llvm_child_type, llvm_value_ref, array_length)
        }
        _ => object,
    }
}
