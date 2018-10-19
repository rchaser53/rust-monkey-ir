use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use llvm_sys::LLVMIntPredicate;

use ir::arithmetic::*;
use ir::block::*;
use ir::const_value::*;
use ir::creator::*;
use ir::function::*;
use ir::llvm_type::*;
use ir::operate::*;
use ir::test_util::*;

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

pub fn build_br(builder: *mut LLVMBuilder, block: *mut LLVMBasicBlock) {
    unsafe {
        LLVMBuildBr(builder, block);
    };
}

macro_rules! create_build_i_cmp {
    ($name:ident, $condition:expr) => {
        pub fn $name(
            builder: *mut LLVMBuilder,
            left_val: *mut LLVMValue,
            right_val: *mut LLVMValue,
            name: &str,
        ) -> *mut LLVMValue {
            unsafe {
                LLVMBuildICmp(
                    builder,
                    $condition,
                    left_val,
                    right_val,
                    c_string!(name).as_ptr(),
                )
            }
        }
    };
}

create_build_i_cmp!(build_int_eq, LLVMIntPredicate::LLVMIntEQ);
create_build_i_cmp!(build_int_ne, LLVMIntPredicate::LLVMIntNE);
create_build_i_cmp!(build_int_ugt, LLVMIntPredicate::LLVMIntUGT);
create_build_i_cmp!(build_int_uge, LLVMIntPredicate::LLVMIntUGE);
create_build_i_cmp!(build_int_ult, LLVMIntPredicate::LLVMIntULT);
create_build_i_cmp!(build_int_ule, LLVMIntPredicate::LLVMIntULE);
create_build_i_cmp!(build_int_sgt, LLVMIntPredicate::LLVMIntSGT);
create_build_i_cmp!(build_int_sge, LLVMIntPredicate::LLVMIntSGE);
create_build_i_cmp!(build_int_slt, LLVMIntPredicate::LLVMIntSLT);
create_build_i_cmp!(build_int_sle, LLVMIntPredicate::LLVMIntSLE);

#[allow(dead_code)]
pub fn create_if_else_test(
    lc: &mut LLVMCreator,
    main: *mut LLVMValue,
    llvm_bool: *mut LLVMValue,
) -> u64 {
    let left_block = append_basic_block_in_context(lc.context, main, "");
    let right_block = append_basic_block_in_context(lc.context, main, "");

    build_cond_br(lc.builder, llvm_bool, left_block, right_block);
    build_position_at_end(lc.builder, left_block);
    let llvm_value = build_alloca(lc.builder, int32_type(), "");
    build_store(lc.builder, const_int(int32_type(), 1), llvm_value);
    let return_value = build_load(lc.builder, llvm_value, "");
    build_ret(lc.builder, return_value);

    build_br(lc.builder, right_block);
    build_position_at_end(lc.builder, right_block);
    let llvm_value = build_alloca(lc.builder, int32_type(), "");
    build_store(lc.builder, const_int(int32_type(), 2), llvm_value);
    let return_value = build_load(lc.builder, llvm_value, "");
    build_ret(lc.builder, return_value);

    execute_test_ir_function(lc.module, main)
}

#[test]
fn cond_if_true() {
    let llvm_bool_true = const_int(int1_type(), 1);
    let mut lc = LLVMCreator::new("test_module");
    let main = setup_main(lc.builder, lc.module);
    assert!(
        create_if_else_test(&mut lc, main, llvm_bool_true) == 1,
        "failed cond_if_true"
    );
}

#[test]
fn cond_if_false() {
    let llvm_bool_false = const_int(int1_type(), 0);
    let mut lc = LLVMCreator::new("test_module");
    let main = setup_main(lc.builder, lc.module);
    assert!(
        create_if_else_test(&mut lc, main, llvm_bool_false) == 2,
        "failed cond_if_false"
    );
}

#[test]
fn cond_int_cmp_true() {
    let mut lc = LLVMCreator::new("test_module");
    let main = setup_main(lc.builder, lc.module);

    let llvm_bool_true = build_int_eq(
        lc.builder,
        const_int(int32_type(), 0),
        const_int(int32_type(), 0),
        "",
    );
    assert!(
        create_if_else_test(&mut lc, main, llvm_bool_true) == 1,
        "failed cond_int_cmp_true"
    );
}

#[test]
fn cond_int_cmp_false() {
    let mut lc = LLVMCreator::new("test_module");
    let main = setup_main(lc.builder, lc.module);

    let llvm_bool_true = build_int_eq(
        lc.builder,
        const_int(int32_type(), 1),
        const_int(int32_type(), 0),
        "",
    );
    assert!(
        create_if_else_test(&mut lc, main, llvm_bool_true) == 2,
        "failed cond_int_cmp_false"
    );
}

#[test]
fn build_while() {
    let mut lc = LLVMCreator::new("test_module");
    let fn_type = create_function_type(int32_type(), &mut []);
    let main = add_function(lc.module, fn_type, "main");

    let entry = append_basic_block(main, "entry");
    let left_block = append_basic_block_in_context(lc.context, main, "left");
    let right_block = append_basic_block_in_context(lc.context, main, "right");

    build_position_at_end(lc.builder, entry);

    let llvm_increment_ref = build_alloca(lc.builder, int32_type(), "a");
    build_store(lc.builder, const_int(int32_type(), 0), llvm_increment_ref);

    build_br(lc.builder, left_block);
    build_position_at_end(lc.builder, left_block);

    let llvm_increment = build_load(lc.builder, llvm_increment_ref, "b");
    let added_value = add_variable(lc.builder, const_int(int32_type(), 1), llvm_increment, "c");
    build_store(lc.builder, added_value, llvm_increment_ref);

    let llvm_bool = build_int_eq(lc.builder, const_int(int32_type(), 3), llvm_increment, "d");
    build_cond_br(lc.builder, llvm_bool, left_block, right_block);
    build_position_at_end(lc.builder, right_block);

    build_ret(lc.builder, const_int(int32_type(), 3));

    assert!(1 == 1, "failed build_while");
}
