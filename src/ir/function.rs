use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

pub fn add_function(
    target_module: *mut LLVMModule,
    function_type: *mut LLVMType,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMAddFunction(target_module, c_string!(name).as_ptr(), function_type) }
}

pub fn create_function_type(ret_type: *mut LLVMType, args: &mut [*mut LLVMType]) -> *mut LLVMType {
    unsafe { LLVMFunctionType(ret_type, args.as_mut_ptr(), args.len() as u32, 0) }
}
