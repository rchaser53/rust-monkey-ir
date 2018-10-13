use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use llvm_sys::*;

pub fn build_alloca(
    builder: *mut LLVMBuilder,
    llvm_type: *mut LLVMType,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMBuildAlloca(builder, llvm_type, c_string!(name).as_ptr()) }
}

pub fn build_store(
    builder: *mut LLVMBuilder,
    value: *mut LLVMValue,
    target: *mut LLVMValue,
) -> *mut LLVMValue {
    unsafe { LLVMBuildStore(builder, value, target) }
}

pub fn build_load(
    builder: *mut LLVMBuilder,
    llvm_value: *mut LLVMValue,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMBuildLoad(builder, llvm_value, c_string!(name).as_ptr()) }
}

pub fn build_ret(builder: *mut LLVMBuilder, llvm_value: *mut LLVMValue) -> *mut LLVMValue {
    unsafe { LLVMBuildRet(builder, llvm_value) }
}

pub fn run_function(
    engine: LLVMExecutionEngineRef,
    function: *mut LLVMValue,
    args_length: u32,
    args: *mut LLVMGenericValueRef,
) -> LLVMGenericValueRef {
    unsafe { LLVMRunFunction(engine, function, args_length, args) }
}
