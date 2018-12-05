use std::ffi::CString;

use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
use llvm_sys::*;

const LLVM_ERROR: i32 = 1;

#[allow(dead_code)]
pub fn validate_module(module: *mut LLVMModule) {
    let mut error = 0 as *mut i8;
    let buf: *mut *mut i8 = &mut error;
    let ok = unsafe {
        LLVMVerifyModule(
            module,
            LLVMVerifierFailureAction::LLVMReturnStatusAction,
            buf,
        )
    };
    if ok == LLVM_ERROR {
        panic!("cannot verify module.\nError: {}", string_from_raw!(error));
    }
}
