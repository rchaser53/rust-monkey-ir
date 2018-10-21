use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::function::*;
use ir::llvm_type::*;

pub fn create_printf(module: *mut LLVMModule) -> *mut LLVMValue {
    let mut printf_args_type_list = vec![pointer_type()];
    let printf_type = function_type_var_arg(pointer_type(), &mut printf_args_type_list);

    add_function(module, printf_type, "printf")
}

pub fn create_strcmp(module: *mut LLVMModule) -> *mut LLVMValue {
    let mut strcmp_args_type_list = vec![pointer_type(), pointer_type()];
    let strcmp_type = function_type_var_arg(int32_type(), &mut strcmp_args_type_list);
    return add_function(module, strcmp_type, "strcmp");
}
