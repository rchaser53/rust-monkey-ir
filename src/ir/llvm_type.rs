use llvm_sys::core::*;
use llvm_sys::*;

pub fn int32_type() -> *mut LLVMType {
    unsafe { LLVMInt32Type() }
}

pub fn int8_type() -> *mut LLVMType {
    unsafe { LLVMInt8Type() }
}

pub fn int1_type() -> *mut LLVMType {
    unsafe { LLVMInt1Type() }
}

pub fn array_type(llvm_type: *mut LLVMType, length: u32) -> *mut LLVMType {
    unsafe { LLVMArrayType(llvm_type, length) }
}
