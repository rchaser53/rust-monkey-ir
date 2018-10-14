use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::creator::*;
use ir::const_value::*;
use ir::llvm_type::*;
use ir::operate::*;
use ir::test_util::*;

pub fn const_string_in_context(context: *mut LLVMContext, input_str: &str) -> *mut LLVMValue {
    let length = input_str.len() as u32;
    unsafe { LLVMConstStringInContext(context, c_string!(input_str).as_ptr(), length - 1, 0) }
}

pub fn codegen_string(
    builder: *mut LLVMBuilder,
    context: *mut LLVMContext,
    input_str: &str,
    name: &str,
) -> *mut LLVMValue {
    let length = input_str.len() as u32;
    let str_val = const_string_in_context(context, input_str);
    let llvm_value = build_alloca(builder, array_type(int8_type(), length), "");
    build_store(builder, str_val, llvm_value);

    let mut args = vec![const_int(int32_type(), 0), const_int(int32_type(), 0)];

    return build_gep(builder, llvm_value, args, name);
}
