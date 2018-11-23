use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

#[allow(dead_code)]
pub fn int32_type() -> *mut LLVMType {
    unsafe { LLVMInt32Type() }
}

#[allow(dead_code)]
pub fn int8_type() -> *mut LLVMType {
    unsafe { LLVMInt8Type() }
}

#[allow(dead_code)]
pub fn int1_type() -> *mut LLVMType {
    unsafe { LLVMInt1Type() }
}

#[allow(dead_code)]
pub fn array_type(llvm_type: *mut LLVMType, length: u32) -> *mut LLVMType {
    unsafe { LLVMArrayType(llvm_type, length) }
}

#[allow(dead_code)]
pub fn pointer_type() -> *mut LLVMType {
    unsafe { LLVMPointerType(int8_type(), 0) }
}

#[allow(dead_code)]
pub fn function_type(ret_type: *mut LLVMType, args: &mut [*mut LLVMType]) -> *mut LLVMType {
    unsafe { LLVMFunctionType(ret_type, args.as_mut_ptr(), args.len() as u32, 0) }
}

#[allow(dead_code)]
pub fn function_type_var_arg(ret_type: *mut LLVMType, args: &mut [*mut LLVMType]) -> *mut LLVMType {
    unsafe { LLVMFunctionType(ret_type, args.as_mut_ptr(), 0, 1) }
}

#[allow(dead_code)]
pub fn type_of(value: *mut LLVMValue) -> *mut LLVMType {
    unsafe { LLVMTypeOf(value) }
}

#[allow(dead_code)]
pub fn cast_type(
    builder: *mut LLVMBuilder,
    value: *mut LLVMValue,
    dest_type: *mut LLVMType,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMBuildBitCast(builder, value, dest_type, c_string!(name).as_ptr()) }
}

#[allow(dead_code)]
pub fn void_type() -> *mut LLVMType {
    unsafe { LLVMVoidType() }
}
