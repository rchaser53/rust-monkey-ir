extern crate llvm_sys;

use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use std::ffi::CString;

mod llvm;
use llvm::*;

mod validater;
use validater::*;

const MODULE_NAME: &'static str = "my_module";

fn main() {
  let mut validater = Validater::new();
  let mut llvm_builder = LlvmBuilder::new();
  let mut engine: LLVMExecutionEngineRef = 0 as LLVMExecutionEngineRef;

  let module = add_module(MODULE_NAME);
  llvm_builder.append_basic_block(add_function(module, "main", &mut [], int32_type()), "entry");

  let a = llvm_builder.create_variable("a", 35, int32_type());
  let b = llvm_builder.create_variable("b", 16, int32_type());
  let res = llvm_builder.multiple_variable(a, b, CString::new("ab_val").unwrap());
  llvm_builder.return_variable(res);

  validater.validate(module);
  if validater.has_error {
    panic!("cannot verify module '{:?}'.\nError: {}", MODULE_NAME, validater.error_message);
  }
  
  llvm_builder.dump(module);

  let _ = excute_module_by_interpreter(&mut engine, module).map_err(|err_msg| {
    panic!("Execution error: {}", err_msg);
  });

  let func_name = CString::new("main").unwrap();
  let named_function = unsafe { LLVMGetNamedFunction(module, func_name.as_ptr()) };
  let mut params = [];
  let func_result = unsafe { LLVMRunFunction(engine, named_function, params.len() as u32, params.as_mut_ptr()) };
  let result = unsafe{ LLVMGenericValueToInt(func_result, 0) };
  println!("{}", result);

  // Clean up the module after we're done with it.
  unsafe { LLVMDisposeModule(module) }
}