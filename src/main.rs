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

const ModuleName: &'static str = "my_module";

fn main() {
    let llvm_error = 1;
    
    let mut llvm_builder = LlvmBuilder::new();
    let mut builder = llvm_builder.builder;

    let module = add_module(ModuleName);
    let function = add_function(module, "main", &mut [], int32_type());
    append_basic_block(builder, function, "entry");

    let a = llvm_builder.create_variable("a", 35, int32_type());
    let b = llvm_builder.create_variable("b", 16, int32_type());

    let ab_val_name = CString::new("ab_val").unwrap();
    unsafe {
      let res = LLVMBuildMul(builder, a, b, ab_val_name.as_ptr());
      LLVMBuildRet(builder, res);
    }

    // verify it's all good
    let mut error: *mut c_char = 0 as *mut c_char;
    let ok = unsafe {
      let buf: *mut *mut c_char = &mut error;
      LLVMVerifyModule(module, LLVMVerifierFailureAction::LLVMReturnStatusAction, buf)
    };
    if ok == llvm_error {
      panic!("cannot verify module '{:?}'.\nError: {}", ModuleName, emit_error(error));
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
    } else{
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