use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

pub fn int32_type() -> *mut LLVMType {
    unsafe { LLVMInt32Type() }
}

pub fn int8_type() -> *mut LLVMType {
    unsafe { LLVMInt8Type() }
}

pub fn int1_type() -> *mut LLVMType {
    unsafe { LLVMInt1Type() }
}

pub fn array_type(llvm_type: *mut LLVMType, length: u32) -> *mut LLVMType {
    unsafe { LLVMArrayType(llvm_type, length) }
}

pub fn pointer_type() -> *mut LLVMType {
    unsafe { LLVMPointerType(int8_type(), 0) }
}

pub fn function_type(ret_type: *mut LLVMType, args: &mut [*mut LLVMType]) -> *mut LLVMType {
    unsafe { LLVMFunctionType(ret_type, args.as_mut_ptr(), args.len() as u32, 0) }
}

pub fn function_type_var_arg(ret_type: *mut LLVMType, args: &mut [*mut LLVMType]) -> *mut LLVMType {
    unsafe { LLVMFunctionType(ret_type, args.as_mut_ptr(), 0, 1) }
}

pub fn type_of(value: *mut LLVMValue) -> *mut LLVMType {
    unsafe { LLVMTypeOf(value) }
}

pub fn cast_type(
    builder: *mut LLVMBuilder,
    value: *mut LLVMValue,
    dest_type: *mut LLVMType,
    name: &str
) -> *mut LLVMValue {
  unsafe {
    LLVMBuildBitCast(builder, value, pointer_type(), c_string!(name).as_ptr())
  }
}