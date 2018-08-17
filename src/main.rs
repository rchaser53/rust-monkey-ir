extern crate libc;
extern crate llvm_sys;

use llvm_sys::LLVMIntPredicate;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::CString;

mod llvm;

fn fun_test(cmp: LLVMIntPredicate)-> Box<Fn(LLVMBuilderRef, u64, u64) -> LLVMValueRef> {
  Box::new(move |builder: LLVMBuilderRef, lhs_val: u64, rhs_val: u64| {
    unsafe {
      let lhs = LLVMConstInt(LLVMInt32Type(), lhs_val, 0);
      let rhs = LLVMConstInt(LLVMInt32Type(), rhs_val, 0);
      LLVMBuildICmp(builder, cmp, lhs, rhs, CString::new("").unwrap().as_ptr())
    }
  })
}

fn main() {
  let aaa = fun_test(LLVMIntPredicate::LLVMIntEQ);
  let builder = unsafe {
    LLVMCreateBuilder()
  };
  println!("{:?}", aaa(builder, 1, 1));
}
