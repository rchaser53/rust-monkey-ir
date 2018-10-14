use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::creator::*;
use ir::llvm_type::*;

pub enum SignedFlag {
    True = 1,
    False = 0,
}

fn signed_flag_converter(flag: SignedFlag) -> i32 {
    match flag {
        SignedFlag::True => 1,
        SignedFlag::False => 0,
    }
}

pub fn const_int(llvm_type: *mut LLVMType, value: u64) -> *mut LLVMValue {
    unsafe { LLVMConstInt(llvm_type, value, 0) }
}

pub fn const_int_signed(llvm_type: *mut LLVMType, value: u64) -> *mut LLVMValue {
    unsafe { LLVMConstInt(llvm_type, value, 0) }
}
