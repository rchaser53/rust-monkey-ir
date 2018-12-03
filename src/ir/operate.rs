use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use llvm_sys::*;

use ir::const_value::*;
use ir::llvm_type::*;

#[allow(dead_code)]
pub fn build_alloca(
    builder: *mut LLVMBuilder,
    llvm_type: *mut LLVMType,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMBuildAlloca(builder, llvm_type, c_string!(name).as_ptr()) }
}

#[allow(dead_code)]
pub fn build_store(
    builder: *mut LLVMBuilder,
    value: *mut LLVMValue,
    target: *mut LLVMValue,
) -> *mut LLVMValue {
    unsafe { LLVMBuildStore(builder, value, target) }
}

#[allow(dead_code)]
pub fn build_load(
    builder: *mut LLVMBuilder,
    llvm_value: *mut LLVMValue,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMBuildLoad(builder, llvm_value, c_string!(name).as_ptr()) }
}

#[allow(dead_code)]
pub fn build_ret(builder: *mut LLVMBuilder, llvm_value: *mut LLVMValue) -> *mut LLVMValue {
    unsafe { LLVMBuildRet(builder, llvm_value) }
}

#[allow(dead_code)]
pub fn build_ret_void(builder: *mut LLVMBuilder) -> *mut LLVMValue {
    unsafe { LLVMBuildRetVoid(builder) }
}

#[allow(dead_code)]
pub fn run_function(
    engine: LLVMExecutionEngineRef,
    function: *mut LLVMValue,
    args_length: u32,
    args: *mut LLVMGenericValueRef,
) -> LLVMGenericValueRef {
    unsafe { LLVMRunFunction(engine, function, args_length, args) }
}

#[allow(dead_code)]
pub fn append_basic_block(function: *mut LLVMValue, function_name: &str) -> *mut LLVMBasicBlock {
    unsafe { LLVMAppendBasicBlock(function, c_string!(function_name).as_ptr()) }
}

#[allow(dead_code)]
pub fn build_position_at_end(builder: *mut LLVMBuilder, block: *mut LLVMBasicBlock) {
    unsafe {
        LLVMPositionBuilderAtEnd(builder, block);
    };
}

#[allow(dead_code)]
pub fn build_gep(
    builder: *mut LLVMBuilder,
    llvm_value: *mut LLVMValue,
    mut args: Vec<*mut LLVMValue>,
    name: &str,
) -> *mut LLVMValue {
    unsafe {
        LLVMBuildGEP(
            builder,
            llvm_value,
            args.as_mut_ptr(),
            args.len() as u32,
            c_string!(name).as_ptr(),
        )
    }
}

#[allow(dead_code)]
pub fn build_const_gep(llvm_const_value: *mut LLVMValue) -> *mut LLVMValue {
    let mut args = vec![const_int(int32_type(), 0), const_int(int32_type(), 0)];
    unsafe { LLVMConstInBoundsGEP(llvm_const_value, args.as_mut_ptr(), args.len() as u32) }
}

#[allow(dead_code)]
pub fn get_u64_from_llvm_value(llvm_const_value: *mut LLVMValue) -> u64 {
    unsafe { LLVMConstIntGetZExtValue(llvm_const_value) }
}
