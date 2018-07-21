use std::os::raw::{c_char};
use std::ffi::CString;

use llvm_sys::*;
use llvm_sys::analysis::{LLVMVerifyModule, LLVMVerifierFailureAction};

const LLVM_ERROR: i32 = 1;

pub struct Validater {
  pub done: bool,
  pub has_error: bool,
  pub error_message: String,
}

impl Validater {
  pub fn new() -> Validater {
    Validater {
      done: false,
      has_error: false,
      error_message: String::new(),
    }
  }

  pub fn validate(&mut self, module: *mut LLVMModule) {
    self.has_error = unsafe {  
      let mut error = 0 as *mut c_char;
      let buf: *mut *mut c_char = &mut error;
      let ret_flag = LLVMVerifyModule(module, LLVMVerifierFailureAction::LLVMReturnStatusAction, buf) == LLVM_ERROR;
      self.error_message = emit_error(error);
      ret_flag
    };
    self.done = true;
  }
}

pub fn emit_error(error: *mut i8) -> String {
  unsafe { CString::from_raw(error).into_string().unwrap() }
}