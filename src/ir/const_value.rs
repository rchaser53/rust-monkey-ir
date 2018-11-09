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

#[allow(dead_code)]
pub fn const_array(llvm_type: *mut LLVMType, mut value: Vec<*mut LLVMValue>) -> *mut LLVMValue {
    unsafe { LLVMConstArray(llvm_type, value.as_mut_ptr(), value.len() as u32) }
}

#[allow(dead_code)]
pub fn const_neg(value: *mut LLVMValue) -> *mut LLVMValue {
    unsafe { LLVMConstNeg(value) }
}
