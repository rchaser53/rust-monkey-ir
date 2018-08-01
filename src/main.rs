extern crate libc;
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
  let function = add_function(module, int32_type(), &mut [], "main");
  llvm_builder.append_basic_block(function, "entry");

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

  let named_function = llvm_builder.get_named_function(module, "main");

  let mut params = [];
  let result = llvm_builder.run_function(engine, named_function, &mut params);
  println!("{}", result);

  unsafe { LLVMDisposeModule(module) }
}

// unsafe {
//   // LLVMTypeRef param_types[] = { LLVMPointerType(LLVMInt8Type(), 0) };
//   let mut param_types = { LLVMPointerType(LLVMInt8Type(), 0) };

//   // LLVMTypeRef llvm_printf_type = LLVMFunctionType(LLVMInt32Type(), param_types, 0, true);
//   let llvm_printf_type = LLVMFunctionType(LLVMInt32Type(), &mut param_types, 0, 0);

//   // LLVMValueRef llvm_printf = LLVMAddFunction(mod, "printf", llvm_printf_type);
//   let cstr = CString::new("printf").unwrap().as_ptr();
//   let llvm_printf = LLVMAddFunction(module, cstr, llvm_printf_type);
// }




// extern{
//   fn static_func();
// }

// fn main() {
//   unsafe {static_func();};
// }


/*
  failed to use LLVMConstString, cause cannnot use AOT
  https://stackoverflow.com/questions/39234493/llvm-error-constant-unimplemented-for-type
*/
// let nyan = b"nyan\0".as_ptr() as *const i8;
// let val_name = CString::new("nyan").unwrap();
// let llvm_type =  LLVMArrayType(LLVMInt8Type(), 6);
// let llvm_value = LLVMBuildAlloca(llvm_builder.builder, llvm_type, val_name.as_ptr());
// LLVMBuildStore(llvm_builder.builder, LLVMConstString(nyan, 6, 1), llvm_value);
// let temp_str = LLVMBuildLoad(llvm_builder.builder, llvm_value, val_name.as_ptr());  // unsafe {
  //   let a = llvm_builder.create_variable("a", 35, int32_type());
  //   let b = llvm_builder.create_variable("b", 16, int32_type());
  //   let res = llvm_builder.multiple_variable(a, b, CString::new("ab_val").unwrap());
  //   llvm_builder.return_variable(res);

  //   validater.validate(module);
  //   if validater.has_error {
  //     panic!("cannot verify module '{:?}'.\nError: {}", MODULE_NAME, validater.error_message);
  //   }
    
  //   let nyan = b"nyan\0".as_ptr() as *const i8;
  //   let val_name = CString::new("nyan").unwrap();
  //   let llvm_type =  LLVMArrayType(LLVMInt8Type(), 6);
  //   let llvm_value = LLVMBuildAlloca(llvm_builder.builder, llvm_type, val_name.as_ptr());
  //   LLVMBuildStore(llvm_builder.builder, LLVMConstString(nyan, 6, 1), llvm_value);
  //   let temp_str = LLVMBuildLoad(llvm_builder.builder, llvm_value, val_name.as_ptr());

  //   LLVMLinkInMCJIT();

  //   let mut error = 0 as *mut ::libc::c_char;
  //   LLVMCreateExecutionEngineForModule(&mut engine, module, &mut error);
  //   llvm_builder.dump(module);

