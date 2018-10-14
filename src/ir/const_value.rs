use llvm_sys::core::*;
use llvm_sys::*;

use ir::creator::*;

pub fn const_int(llvm_type: *mut LLVMType, value: u64) -> *mut LLVMValue {
    unsafe { LLVMConstInt(llvm_type, value, 0) }
}

pub fn const_int_signed(llvm_type: *mut LLVMType, value: u64) -> *mut LLVMValue {
    unsafe { LLVMConstInt(llvm_type, value, 0) }
}
