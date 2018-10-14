use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::creator::*;
use ir::llvm_type::*;
use ir::test_util::*;

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
fn setup_llvm() -> LLVMCreator {
    let lc = LLVMCreator::new("test_module");
    setup_main(lc.builder, lc.module);
    lc
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
        let lc = setup_llvm();
        int_arithmetic_assert(
            add_variable(
                lc.builder,
                LLVMConstInt(int32_type(), 1, 0),
                LLVMConstInt(int32_type(), 2, 0),
                "",
            ),
            LLVMConstInt(int32_type(), 3, 0),
        );
    }
}
