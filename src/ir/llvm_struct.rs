use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::builder::*;
use ir::const_value::*;
use ir::llvm_type::*;
use ir::operate::*;
use ir::test_util::*;

pub fn create_struct(
    builder: *mut LLVMBuilder,
    context: *mut LLVMContext,
    mut elements: Vec<*mut LLVMType>,
    name: &str,
) -> *mut LLVMValue {
    unsafe {
        let mut range = [
            LLVMConstInt(int32_type(), 0, 0),
            LLVMConstInt(int32_type(), 1, 0),
        ];

        let struct_type = LLVMStructCreateNamed(context, c_string!(name).as_ptr());
        LLVMStructSetBody(struct_type, elements.as_mut_ptr(), elements.len() as u32, 0);

        LLVMBuildAlloca(builder, struct_type, c_string!("").as_ptr())
    }
}

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
        let mut field = LLVMBuildInBoundsGEP(
            builder,
            target_struct,
            range.as_mut_ptr(),
            2,
            c_string!(name).as_ptr(),
        );
        LLVMBuildStore(builder, value, field)
    }
}

#[test]
fn strcut_test() {
    let mut lb = LlvmBuilder::new("test_module");
    let main = lb.setup_main();

    let elements = vec![int32_type(), int32_type()];
    let mut target_struct = create_struct(lb.builder, lb.context, elements, "test");

    set_field_value(
        lb.builder,
        target_struct,
        0,
        const_int(int32_type(), 2, SignedFlag::False),
        "",
    );
    let llvm_value = get_field_value(lb.builder, target_struct, 0, "");
    let return_value = build_load(lb.builder, llvm_value, "");
    build_ret(lb.builder, return_value);

    assert!(
        execute_test_ir_function(lb.module, main) == 2,
        "failed cond_if_false"
    );
}
