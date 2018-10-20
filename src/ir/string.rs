use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::LLVMLinkage::*;
use llvm_sys::*;

use ir::creator::*;
use ir::llvm_type::*;
use ir::operate::*;
use ir::scope::*;

pub fn const_string_in_context(context: *mut LLVMContext, input_str: &str) -> *mut LLVMValue {
    let length = input_str.len() as u32;
    unsafe { LLVMConstStringInContext(context, c_string!(input_str).as_ptr(), length, 1) }
}

pub fn codegen_string(lc: &mut LLVMCreator, input_str: &str, name: &str) -> *mut LLVMValue {
    let str_val = const_string_in_context(lc.context, input_str);
    let global_str_val = add_global(lc.module, type_of(str_val), name);
    set_linkage(global_str_val, LLVMPrivateLinkage);
    set_initializer(global_str_val, str_val);
    set_global_constant(global_str_val);
    set_unnamed_addr(global_str_val);

    return build_const_gep(global_str_val);
}
