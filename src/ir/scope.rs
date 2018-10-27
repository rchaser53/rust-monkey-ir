use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

#[allow(dead_code)]
pub fn add_global(module: *mut LLVMModule, llvm_type: *mut LLVMType, name: &str) -> *mut LLVMValue {
    unsafe { LLVMAddGlobal(module, llvm_type, c_string!(name).as_ptr()) }
}

#[allow(dead_code)]
pub fn set_linkage(value: *mut LLVMValue, linkage: LLVMLinkage) {
    unsafe {
        LLVMSetLinkage(value, linkage);
    };
}

#[allow(dead_code)]
pub fn set_initializer(target: *mut LLVMValue, value: *mut LLVMValue) {
    unsafe {
        LLVMSetInitializer(target, value);
    }
}

#[allow(dead_code)]
pub fn set_global_constant(value: *mut LLVMValue) {
    unsafe {
        LLVMSetGlobalConstant(value, 1);
    }
}

#[allow(dead_code)]
pub fn set_unnamed_addr(value: *mut LLVMValue) {
    unsafe {
        LLVMSetUnnamedAddr(value, 1);
    }
}
