use llvm_sys::core::*;
use llvm_sys::*;

#[allow(dead_code)]
pub fn const_int(llvm_type: *mut LLVMType, value: u64) -> *mut LLVMValue {
    unsafe { LLVMConstInt(llvm_type, value, 0) }
}

#[allow(dead_code)]
pub fn const_int_signed(llvm_type: *mut LLVMType, value: u64) -> *mut LLVMValue {
    unsafe { LLVMConstInt(llvm_type, value, 0) }
}
