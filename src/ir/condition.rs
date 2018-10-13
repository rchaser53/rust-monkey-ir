use std::ffi::CString;
use std::os::raw::c_char;

use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use llvm_sys::*;

use ir::builder::*;
use ir::const_value::*;
use ir::llvm_type::*;
use ir::operate::*;
use ir::test_util::*;

pub fn append_basic_block_in_context(
    context: *mut LLVMContext,
    function: *mut LLVMValue,
    function_name: &str,
) -> *mut LLVMBasicBlock {
    unsafe { LLVMAppendBasicBlockInContext(context, function, c_string!(function_name).as_ptr()) }
}

pub fn append_basic_block(function: *mut LLVMValue, function_name: &str) -> *mut LLVMBasicBlock {
    unsafe { LLVMAppendBasicBlock(function, c_string!(function_name).as_ptr()) }
}

pub fn build_cond_br(
    builder: *mut LLVMBuilder,
    condition: *mut LLVMValue,
    left_block: *mut LLVMBasicBlock,
    right_block: *mut LLVMBasicBlock,
) {
    unsafe {
        LLVMBuildCondBr(builder, condition, left_block, right_block);
    };
}

pub fn build_position_at_end(builder: *mut LLVMBuilder, block: *mut LLVMBasicBlock) {
    unsafe {
        LLVMPositionBuilderAtEnd(builder, block);
    };
}

pub fn build_br(builder: *mut LLVMBuilder, block: *mut LLVMBasicBlock) {
    unsafe {
        LLVMPositionBuilderAtEnd(builder, block);
    };
}

#[allow(dead_code)]
pub fn create_if_else_test(llvm_bool: *mut LLVMValue) -> u64 {
    let mut lb = LlvmBuilder::new("test_module");
    let main = lb.setup_main();

    let left_block = append_basic_block_in_context(lb.context, main, "");
    let right_block = append_basic_block_in_context(lb.context, main, "");

    build_cond_br(lb.builder, llvm_bool, left_block, right_block);
    build_position_at_end(lb.builder, left_block);
    let llvm_value = build_alloca(lb.builder, int32_type(), "");
    build_store(
        lb.builder,
        const_int(int32_type(), 1, SignedFlag::False),
        llvm_value,
    );
    let return_value = build_load(lb.builder, llvm_value, "");
    build_ret(lb.builder, return_value);

    build_br(lb.builder, right_block);
    build_position_at_end(lb.builder, right_block);
    let llvm_value = build_alloca(lb.builder, int32_type(), "");
    build_store(
        lb.builder,
        const_int(int32_type(), 2, SignedFlag::False),
        llvm_value,
    );
    let return_value = build_load(lb.builder, llvm_value, "");
    build_ret(lb.builder, return_value);

    execute_test_ir_function(lb.module, main)
}

#[test]
fn cond_if_true() {
    let mut llvm_bool_true = const_int(int1_type(), 1, SignedFlag::False);
    assert!(
        create_if_else_test(llvm_bool_true) == 1,
        "failed cond_if_true"
    );
}

#[test]
fn cond_if_false() {
    let mut llvm_bool_false = const_int(int1_type(), 0, SignedFlag::False);
    assert!(
        create_if_else_test(llvm_bool_false) == 2,
        "failed cond_if_false"
    );
}
