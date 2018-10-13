use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::builder::*;
use ir::llvm_type::*;

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

pub fn multiple_variable(
    builder: *mut LLVMBuilder,
    var_a: *mut LLVMValue,
    var_b: *mut LLVMValue,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMBuildMul(builder, var_a, var_b, c_string!(name).as_ptr()) }
}

#[allow(dead_code)]
fn setup_llvm() -> LlvmBuilder {
    unsafe {
        let mut lb = LlvmBuilder::new("test_module");
        lb.setup_main();
        lb
    }
}

#[allow(dead_code)]
fn int_arithmetic_assert(actual: *mut LLVMValue, expect: *mut LLVMValue) {
    unsafe {
        assert!(
            actual == expect,
            "\r\nexpected: {:?} \r\nactual: {:?}",
            LLVMConstIntGetZExtValue(actual),
            LLVMConstIntGetZExtValue(expect)
        );
    }
}

#[test]
fn add() {
    unsafe {
        let lb = setup_llvm();
        int_arithmetic_assert(
            add_variable(
                lb.builder,
                LLVMConstInt(int32_type(), 1, 0),
                LLVMConstInt(int32_type(), 2, 0),
                "",
            ),
            LLVMConstInt(int32_type(), 3, 0),
        );
    }
}
