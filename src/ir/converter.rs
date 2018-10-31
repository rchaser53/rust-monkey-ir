use llvm_sys::*;

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
