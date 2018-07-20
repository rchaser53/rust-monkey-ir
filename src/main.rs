extern crate llvm_sys;

use llvm_sys::*;
use llvm_sys::core::*;
use llvm_sys::target;
use llvm_sys::analysis::{LLVMVerifyModule, LLVMVerifierFailureAction};
use llvm_sys::execution_engine::*;
use std::ffi::CString;
use std::os::raw::{c_char};

mod llvm;
use llvm::*;

struct LlvmBuilder {
  builder: *mut LLVMBuilder
}

fn add_main_fn(module: &mut LLVMModule) -> *mut LLVMValue {
    let mut main_args = vec![];
    unsafe {
        let main_type = LLVMFunctionType(int32_type(), main_args.as_mut_ptr(), 0, 0);
        LLVMAddFunction(module, CString::new("main").unwrap().as_ptr(), main_type)
    }
}

impl LlvmBuilder {
  fn new() -> LlvmBuilder {
    LlvmBuilder::initialise();

    LlvmBuilder {
      builder: unsafe { LLVMCreateBuilder() }
    }
  }

  fn initialise() {
    unsafe {
        if target::LLVM_InitializeNativeTarget() != 0 {
            panic!("Could not initialise target");
        }
        if target::LLVM_InitializeNativeAsmPrinter() != 0 {
            panic!("Could not initialise ASM Printer");
        }
    }
  }

  fn create_variable(&mut self, name: &str, value: u64) -> *mut LLVMValue {
    let val_name = CString::new(name).unwrap();
    let llvm_value = unsafe {
      LLVMBuildAlloca(self.builder, int32_type(), val_name.as_ptr())
    };
    unsafe {
      LLVMBuildStore(self.builder, LLVMConstInt(int32_type(), value, 0), llvm_value);
    }

    llvm_value
  }

  fn dump(&self, module: *mut LLVMModule) {
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

fn main() {
    let llvm_error = 1;
    
    let mut llvm_builder = LlvmBuilder::new();
    let builder = llvm_builder.builder;

    let mod_name = CString::new("my_module").unwrap();
    let module = unsafe { LLVMModuleCreateWithName(mod_name.as_ptr()) };

    let function_type = unsafe {
        let mut param_types = [];
        LLVMFunctionType(int32_type(), param_types.as_mut_ptr(), param_types.len() as u32, 0)
    };
    let function_name = CString::new("main").unwrap();
    let function = unsafe { LLVMAddFunction(module, function_name.as_ptr(), function_type) };
    let entry_name = CString::new("entry").unwrap();
    let entry_block = unsafe { LLVMAppendBasicBlock(function, entry_name.as_ptr()) };
    unsafe { LLVMPositionBuilderAtEnd(builder, entry_block); }

    let a = llvm_builder.create_variable("a", 35);
    let b = llvm_builder.create_variable("b", 16);

    let b_val_name = CString::new("b_val").unwrap();
    let b_val = unsafe { LLVMBuildLoad(builder, b, b_val_name.as_ptr()) };
    let a_val_name = CString::new("a_val").unwrap();
    let a_val = unsafe { LLVMBuildLoad(builder, a, a_val_name.as_ptr()) };
    let ab_val_name = CString::new("ab_val").unwrap();
    unsafe {
        let res = LLVMBuildMul(builder, a_val, b_val, ab_val_name.as_ptr());
        LLVMBuildRet(builder, res);
    }

    // verify it's all good
    let mut error: *mut c_char = 0 as *mut c_char;
    let ok = unsafe {
        let buf: *mut *mut c_char = &mut error;
        LLVMVerifyModule(module, LLVMVerifierFailureAction::LLVMReturnStatusAction, buf)
    };
    if ok == llvm_error {
        let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
        panic!("cannot verify module '{:?}'.\nError: {}", mod_name, err_msg);
    }

    llvm_builder.dump(module);

    // create our exe engine
    let mut engine: LLVMExecutionEngineRef = 0 as LLVMExecutionEngineRef;
    let status = unsafe {
        error = 0 as *mut c_char;
        let buf: *mut *mut c_char = &mut error;
        let engine_ref: *mut LLVMExecutionEngineRef = &mut engine;
        LLVMLinkInInterpreter();
        LLVMCreateInterpreterForModule(engine_ref, module, buf)
    };

    if status == llvm_error {
        let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
        println!("Execution error: {}", err_msg);
    }else{
        // run the function!
        let func_name = CString::new("main").unwrap();
        let named_function = unsafe { LLVMGetNamedFunction(module, func_name.as_ptr()) };
        let mut params = [];
        let func_result = unsafe { LLVMRunFunction(engine, named_function, params.len() as u32, params.as_mut_ptr()) };
        let result = unsafe{ LLVMGenericValueToInt(func_result, 0) };
        println!("{}", result);
    }

    // Clean up the module after we're done with it.
    unsafe { LLVMDisposeModule(module) }
}