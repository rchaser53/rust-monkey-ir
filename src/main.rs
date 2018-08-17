// extern crate libc;
// extern crate llvm_sys;

// use llvm_sys::LLVMIntPredicate;
// use llvm_sys::core::*;
// use llvm_sys::prelude::*;
// use llvm_sys::execution_engine::*;
// use std::ffi::CString;

// mod llvm;
// use llvm::*;

fn fun_test(value: i32, f: fn(i32) -> i32) -> i32 {
    println!("{}", f (value));
    value + 111
}

fn koyan(a: i32) -> i32 {
  a * 2
}

fn main() {
  let aaa = fun_test(1, koyan);
  println!("{}", aaa);
}
