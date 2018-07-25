use std::ffi::CString;
use std::os::raw::{c_char};

use llvm_sys::*;
use llvm_sys::core::*;
use llvm_sys::execution_engine::*;

const LLVM_ERROR: i32 = 1;

pub fn int32_type() -> *mut LLVMType {
  unsafe { LLVMInt32Type() }
}

pub fn add_function(module: *mut LLVMModule,
                    ret_type: *mut LLVMType,
                    args: &mut [*mut LLVMType],
                    fn_name: &str) -> *mut LLVMValue {
  unsafe {
    let fn_type = LLVMFunctionType(ret_type, args.as_mut_ptr(), args.len() as u32, 0);
    let cstring = CString::new(fn_name).unwrap();
    let ptr = cstring.as_ptr() as *mut _;
    LLVMAddFunction(module, ptr, fn_type)
  }
}

// create our exe engine
pub fn excute_module_by_interpreter(engine_ref: *mut LLVMExecutionEngineRef, module: *mut LLVMModule) -> Result<i32, String> {
  let mut error = 0 as *mut c_char;
  let status = unsafe {
    let buf: *mut *mut c_char = &mut error;
    LLVMLinkInInterpreter();
    LLVMCreateInterpreterForModule(engine_ref, module, buf)
  };

  if status == LLVM_ERROR {
    let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
    return Err(err_msg);
  }

  Ok(status)
}

pub fn add_module(module_name: &str) -> *mut LLVMModule {
  let mod_name = CString::new(module_name).unwrap();
  unsafe { LLVMModuleCreateWithName(mod_name.as_ptr()) }
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

  pub fn append_basic_block(&mut self, function: *mut LLVMValue, name: &str) {
    let entry_name = CString::new(name).unwrap();
    let entry_block = unsafe { LLVMAppendBasicBlock(function, entry_name.as_ptr()) };
    unsafe { LLVMPositionBuilderAtEnd(self.builder, entry_block); }
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

  pub fn multiple_variable(&mut self, var_a: *mut LLVMValue, var_b: *mut LLVMValue, c_str: CString) -> *mut LLVMValue {
    unsafe {
      LLVMBuildMul(self.builder, var_a, var_b, c_str.as_ptr())
    }
  }

  pub fn return_variable(&mut self, res: *mut LLVMValue) {
    unsafe {
      LLVMBuildRet(self.builder, res);
    }
  }

  pub fn run_function(&mut self, engine: *mut LLVMOpaqueExecutionEngine, named_function: *mut LLVMValue, params: &mut [*mut LLVMOpaqueGenericValue]) -> u64 {
    let func_result = unsafe { LLVMRunFunction(engine, named_function, params.len() as u32, params.as_mut_ptr()) };
    unsafe{ LLVMGenericValueToInt(func_result, 0) }
  }

  pub fn get_named_function(&mut self, module: *mut LLVMModule, name: &str) -> *mut LLVMValue {
    let func_name = CString::new(name).unwrap();
    unsafe { LLVMGetNamedFunction(module, func_name.as_ptr()) }
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