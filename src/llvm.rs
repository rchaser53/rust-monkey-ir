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

pub fn append_basic_block(builder: *mut LLVMBuilder, function: *mut LLVMValue, name: &str) {
  let entry_name = CString::new(name).unwrap();
  let entry_block = unsafe { LLVMAppendBasicBlock(function, entry_name.as_ptr()) };
  unsafe { LLVMPositionBuilderAtEnd(builder, entry_block); }
}

pub fn add_module(module_name: &str) -> *mut LLVMModule {
  let mod_name = CString::new(module_name).unwrap();
  unsafe { LLVMModuleCreateWithName(mod_name.as_ptr()) }
}

pub fn emit_error(error: *mut i8) -> String {
  unsafe { CString::from_raw(error).into_string().unwrap() }
}

// pub fn load

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

  pub fn create_variable(&mut self, name: &str, value: u64, llvm_type: *mut LLVMType) -> *mut LLVMValue {
    let val_name = CString::new(name).unwrap();
    let llvm_value = unsafe {
      LLVMBuildAlloca(self.builder, llvm_type, val_name.as_ptr())
    };
    unsafe {
      LLVMBuildStore(self.builder, LLVMConstInt(llvm_type, value, 0), llvm_value);
    }
    unsafe { LLVMBuildLoad(self.builder, llvm_value, val_name.as_ptr()) }
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