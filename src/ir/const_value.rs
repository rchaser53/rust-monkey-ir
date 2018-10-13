use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::builder::*;
use ir::llvm_type::*;

pub enum SignedFlag {
    True = 1,
    False = 0,
}

pub fn SignedFlagConverter(flag: SignedFlag) -> i32 {
    match flag {
        SignedFlag::True => 1,
        SignedFlag::False => 0,
    }
}

pub fn const_int(llvm_type: *mut LLVMType, value: u64, signed_flag: SignedFlag) -> *mut LLVMValue {
    unsafe { LLVMConstInt(llvm_type, value, SignedFlagConverter(signed_flag)) }
}
