use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::builder::*;
use ir::llvm_type::*;

pub fn append_basic_block_in_context(
    context: *mut LLVMContext,
    function: *mut LLVMValue,
    function_name: &str
) -> *mut LLVMBasicBlock {
  unsafe {
    LLVMAppendBasicBlockInContext(context, function, c_string!(function_name).as_ptr())
  }
}

pub fn build_cond_br(
  builder: *mut LLVMBuilder,
  condition: *mut LLVMValue,
  left_block: *mut LLVMBasicBlock,
  right_block: *mut LLVMBasicBlock
) {
  unsafe {
    LLVMBuildCondBr(builder, condition, left_block, right_block);
  };
}

pub fn build_position_at_end(
  builder: *mut LLVMBuilder,
  block: *mut LLVMBasicBlock
) {
  unsafe {
    LLVMPositionBuilderAtEnd(builder, block);
  };
}

pub fn build_br(
  builder: *mut LLVMBuilder,
  block: *mut LLVMBasicBlock
) {
  unsafe {
    LLVMPositionBuilderAtEnd(builder, block);
  };
}


#[test]
fn cond() {
    unsafe {
      let mut lb = LlvmBuilder::new("test_module");
      let main = lb.add_function(int32_type(), &mut [], "main");
      let block = lb.append_basic_block("main", "entry");

      let left_block = append_basic_block_in_context(lb.context, main, "");
      let right_block = append_basic_block_in_context(lb.context, main, "");
      let mut llvm_bool = LLVMConstInt(int32_type(), 1, 0);

      build_cond_br(lb.builder, llvm_bool, left_block, right_block);
      build_position_at_end(lb.builder, left_block);

      build_br(lb.builder, right_block);
      build_position_at_end(lb.builder, right_block);
    }
}
