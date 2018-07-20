use llvm_sys::*;
use llvm_sys::core::*;

pub fn int8_type() -> *mut LLVMType {
    unsafe { LLVMInt8Type() }
}

pub fn int32_type() -> *mut LLVMType {
    unsafe { LLVMInt32Type() }
}
