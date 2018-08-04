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

  unsafe {
    /* === */

    let a = llvm_builder.create_variable("a", 35, int32_type());
    let b = llvm_builder.create_variable("b", 16, int32_type());
    let res = llvm_builder.multiple_variable(a, b, CString::new("ab_val").unwrap());

    llvm_builder.return_variable(res);

    /* === */


    /* === */

    let mut param_types = LLVMPointerType(LLVMInt8Type(), 0);
    let llvm_printf_type = LLVMFunctionType(LLVMInt32Type(), &mut param_types, 0, 1);
    let print_name = CString::new("printf").unwrap();
    let llvm_printf = LLVMAddFunction(module, print_name.as_ptr(), llvm_printf_type);
    llvm_builder.append_basic_block(llvm_printf, "entry");

    let nyan = b"nyan\0".as_ptr() as *const i8;
    let val_name = CString::new("nyan").unwrap();
    let llvm_type =  LLVMArrayType(LLVMInt8Type(), 6);
    let llvm_value = LLVMBuildAlloca(llvm_builder.builder, llvm_type, val_name.as_ptr());
    LLVMBuildStore(llvm_builder.builder, LLVMConstString(nyan, 6, 1), llvm_value);
    let temp_str = LLVMBuildLoad(llvm_builder.builder, llvm_value, val_name.as_ptr());

    let hohoho = llvm_builder.create_variable("hohoho", 16, int32_type());
    llvm_builder.return_variable(hohoho);

    /* === */

    validater.validate(module);
    if validater.has_error {
      panic!("cannot verify module '{:?}'.\nError: {}", MODULE_NAME, validater.error_message);
    }
    
    LLVMLinkInMCJIT();

    let mut error = 0 as *mut ::libc::c_char;
    LLVMCreateExecutionEngineForModule(&mut engine, module, &mut error);
    llvm_builder.dump(module);

    LLVMDisposeModule(module);
  }
}