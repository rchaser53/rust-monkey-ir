extern crate llvm_sys;

use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use std::ffi::CString;
use std::os::raw::{c_char};

mod llvm;
use llvm::*;

mod validater;
use validater::*;

const MODULE_NAME: &'static str = "my_module";
const LLVM_ERROR: i32 = 1;

fn main() {
  let mut error = 0 as *mut c_char;
  let mut validater = Validater::new();

  let mut llvm_builder = LlvmBuilder::new();
  let builder = llvm_builder.builder;

  let module = add_module(MODULE_NAME);
  let function = add_function(module, "main", &mut [], int32_type());
  append_basic_block(builder, function, "entry");

  let a = llvm_builder.create_variable("a", 35, int32_type());
  let b = llvm_builder.create_variable("b", 16, int32_type());

  let ab_val_name = CString::new("ab_val").unwrap();
  unsafe {
    let res = LLVMBuildMul(builder, a, b, ab_val_name.as_ptr());
    LLVMBuildRet(builder, res);
  }

  validater.validate(module);
  if validater.has_error {
    panic!("cannot verify module '{:?}'.\nError: {}", MODULE_NAME, validater.error_message);
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

  if status == LLVM_ERROR {
    let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
    println!("Execution error: {}", err_msg);
  } else {
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