use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::builder::*;
use ir::llvm_type::*;

pub fn create_struct(
  context: *mut LLVMContext,
  mut elements: Vec<*mut LLVMType>,
  name: &str
) -> *mut LLVMType {
  unsafe {
    let named = LLVMStructCreateNamed(context, c_string!(name).as_ptr());
    LLVMStructSetBody(named, elements.as_mut_ptr(), elements.len() as u32, 0);
    named
  }
}

pub fn get_field_value(
  builder: *mut LLVMBuilder,
  target_struct: *mut LLVMValue,
  target_index: u64,
  name: &str
) -> *mut LLVMValue {
  unsafe {
    let mut range = [
      LLVMConstInt(int32_type(), 0, 0),
      LLVMConstInt(int32_type(), target_index, 0)
    ];
    LLVMBuildInBoundsGEP(builder, target_struct, range.as_mut_ptr(), 2, c_string!(name).as_ptr())
  }
}

pub fn set_field_value(
  builder: *mut LLVMBuilder,
  target_struct: *mut LLVMValue,
  target_index: u64,
  value: *mut LLVMValue,
  name: &str,
) -> *mut LLVMValue {
  unsafe {
    let mut range = [
      LLVMConstInt(int32_type(), 0, 0),
      LLVMConstInt(int32_type(), target_index, 0)
    ];
    let mut field = LLVMBuildInBoundsGEP(builder, target_struct, range.as_mut_ptr(), 2, c_string!(name).as_ptr());
    LLVMBuildStore(builder, value, field)
  }
}

#[allow(dead_code)]
fn setup_llvm() -> LlvmBuilder {
    unsafe {
        let mut lb = LlvmBuilder::new("test_module");
        lb.setup_main();
        lb
    }
}

#[test]
fn strcut_test() {
  let lb = setup_llvm();
  let elements = vec![
    int32_type(),
    int32_type()
  ];
  let mut struct_type = create_struct(lb.context, elements, "test");
}
