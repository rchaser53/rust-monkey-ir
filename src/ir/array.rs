use llvm_sys::*;
use llvm_sys::core::*;

use ir::llvm_type::*;

pub fn get_array_length(llvm_type_ref: *mut LLVMType) -> u32 {
  unsafe {
    LLVMGetArrayLength(llvm_type_ref)
  }
}

#[test]
fn get_array_length_test() {
    let expected = 10;
    let actual = get_array_length(array_type(int32_type(), 10));
    assert!(
        expected == actual,
        "test failed \r\nexpected: {}\r\nactual:{}",
        expected,
        actual
    );
}
