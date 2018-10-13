use llvm_sys::core::*;
use llvm_sys::*;

pub fn int32_type() -> *mut LLVMType {
    unsafe { LLVMInt32Type() }
}

pub fn int1_type() -> *mut LLVMType {
    unsafe { LLVMInt1Type() }
}
