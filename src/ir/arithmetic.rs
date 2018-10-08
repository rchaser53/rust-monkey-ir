use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::builder::*;
use ir::llvm_type::*;

pub fn multiple_variable(
    builder: *mut LLVMBuilder,
    var_a: *mut LLVMValue,
    var_b: *mut LLVMValue,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMBuildMul(builder, var_a, var_b, c_string!(name).as_ptr()) }
}

pub fn add_variable(
    builder: *mut LLVMBuilder,
    var_a: *mut LLVMValue,
    var_b: *mut LLVMValue,
    name: &str,
) -> *mut LLVMValue {
    unsafe {
      return LLVMBuildAdd(builder, var_a, var_b, c_string!(name).as_ptr());
    };
}

pub fn sub_variable(
    builder: *mut LLVMBuilder,
    var_a: *mut LLVMValue,
    var_b: *mut LLVMValue,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMBuildSub(builder, var_a, var_b, c_string!(name).as_ptr()) }
} 

#[test]
fn try_to_know_how_to_test_ir() {
    unsafe {
      assert!(
          LLVMConstInt(int32_type(), 3, 0) == LLVMConstInt(int32_type(), 3, 0),
          "nya-n"
      );
    }

}