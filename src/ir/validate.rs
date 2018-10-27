use std::ffi::CString;
use std::os::raw::c_char;

use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
use llvm_sys::*;

const LLVM_ERROR: i32 = 1;

#[allow(dead_code)]
pub fn validate_module(module: *mut LLVMModule) {
    unsafe {
        let mut error: *mut c_char = 0 as *mut c_char;
        let buf: *mut *mut c_char = &mut error;
        let ok = LLVMVerifyModule(
            module,
            LLVMVerifierFailureAction::LLVMReturnStatusAction,
            buf,
        );
        if ok == LLVM_ERROR {
            let err_msg = CString::from_raw(error).into_string().unwrap();
            panic!("cannot verify module.\nError: {}", err_msg);
        }
    }
}
