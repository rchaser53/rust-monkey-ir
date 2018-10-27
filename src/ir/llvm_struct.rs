use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::creator::*;
use ir::llvm_type::*;
use ir::test_util::*;

#[allow(unused_imports)]
use ir::const_value::*;

#[allow(unused_imports)]
use ir::operate::*;

#[allow(dead_code)]
pub fn create_struct(
    builder: *mut LLVMBuilder,
    context: *mut LLVMContext,
    mut elements: Vec<*mut LLVMType>,
    name: &str,
) -> *mut LLVMValue {
    unsafe {
        let struct_type = LLVMStructCreateNamed(context, c_string!(name).as_ptr());
        LLVMStructSetBody(struct_type, elements.as_mut_ptr(), elements.len() as u32, 0);

        LLVMBuildAlloca(builder, struct_type, c_string!("").as_ptr())
    }
}

#[allow(dead_code)]
pub fn get_field_value(
    builder: *mut LLVMBuilder,
    target_struct: *mut LLVMValue,
    target_index: u64,
    name: &str,
) -> *mut LLVMValue {
    unsafe {
        let mut range = [
            LLVMConstInt(int32_type(), 0, 0),
            LLVMConstInt(int32_type(), target_index, 0),
        ];
        LLVMBuildInBoundsGEP(
            builder,
            target_struct,
            range.as_mut_ptr(),
            2,
            c_string!(name).as_ptr(),
        )
    }
}

#[allow(dead_code)]
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
            LLVMConstInt(int32_type(), target_index, 0),
        ];
        let field = LLVMBuildInBoundsGEP(
            builder,
            target_struct,
            range.as_mut_ptr(),
            2,
            c_string!(name).as_ptr(),
        );
        LLVMBuildStore(builder, value, field)
    }
}

#[allow(dead_code)]
fn assert_llvm_struct<F>(test_func: F)
where
    F: Fn(LLVMCreator, *mut LLVMValue) -> u64,
{
    let mut lc = LLVMCreator::new("test_module");
    let main = setup_main(&mut lc);

    assert!(test_func(lc, main) == 2, "failed cond_if_false");
}

#[test]
fn strcut_test() {
    assert_llvm_struct(move |lc, main| {
        let elements = vec![int32_type(), int32_type()];
        let target_struct = create_struct(lc.builder, lc.context, elements, "test");

        set_field_value(lc.builder, target_struct, 0, const_int(int32_type(), 2), "");
        let llvm_value = get_field_value(lc.builder, target_struct, 0, "");
        let return_value = build_load(lc.builder, llvm_value, "");
        build_ret(lc.builder, return_value);
        execute_test_ir_function(lc.module, main)
    });
}
