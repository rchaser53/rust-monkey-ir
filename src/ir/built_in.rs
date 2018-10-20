use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::function::*;
use ir::llvm_type::*;

pub fn create_printf(module: *mut LLVMModule) -> *mut LLVMValue {
    unsafe {
        let mut printf_args_type_list = vec![LLVMPointerType(LLVMInt8Type(), 0)];
        let printf_type =
            LLVMFunctionType(LLVMInt32Type(), printf_args_type_list.as_mut_ptr(), 0, 1);

        return LLVMAddFunction(module, c_string!("printf").as_ptr(), printf_type);
    }
}

pub fn create_strcmp(module: *mut LLVMModule) -> *mut LLVMValue {
    let mut strcmp_args_type_list = vec![pointer_type(), pointer_type()];
    let strcmp_type = function_type(int32_type(), &mut strcmp_args_type_list);
    return add_function(module, strcmp_type, "strcmp");
}
