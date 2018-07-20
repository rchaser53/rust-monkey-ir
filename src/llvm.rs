use std::ffi::CString;

use llvm_sys::*;
use llvm_sys::core::*;

pub fn int8_type() -> *mut LLVMType {
    unsafe { LLVMInt8Type() }
}

pub fn int32_type() -> *mut LLVMType {
    unsafe { LLVMInt32Type() }
}

pub fn add_function(module: *mut LLVMModule,
                fn_name: &str,
                args: &mut [*mut LLVMType],
                ret_type: *mut LLVMType) -> *mut LLVMValue {
    unsafe {
        let fn_type = LLVMFunctionType(ret_type, args.as_mut_ptr(), args.len() as u32, 0);
        let cstring = CString::new(fn_name).unwrap();
        let ptr = cstring.as_ptr() as *mut _;
        LLVMAddFunction(module, ptr, fn_type)
    }
}

pub struct LlvmBuilder {
  pub builder: *mut LLVMBuilder
}

impl LlvmBuilder {
  pub fn new() -> LlvmBuilder {
    LlvmBuilder::initialise();

    LlvmBuilder {
      builder: unsafe { LLVMCreateBuilder() }
    }
  }

  pub fn initialise() {
    unsafe {
      if target::LLVM_InitializeNativeTarget() != 0 {
        panic!("Could not initialise target");
      }
      if target::LLVM_InitializeNativeAsmPrinter() != 0 {
        panic!("Could not initialise ASM Printer");
      }
    }
  }

  pub fn create_variable(&mut self, name: &str, value: u64) -> *mut LLVMValue {
    let val_name = CString::new(name).unwrap();
    let llvm_value = unsafe {
      LLVMBuildAlloca(self.builder, int32_type(), val_name.as_ptr())
    };
    unsafe {
      LLVMBuildStore(self.builder, LLVMConstInt(int32_type(), value, 0), llvm_value);
    }

    llvm_value
  }

  pub fn dump(&self, module: *mut LLVMModule) {
    unsafe { LLVMDumpModule(module) }
  }
}

impl Drop for LlvmBuilder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
        }
    }
}