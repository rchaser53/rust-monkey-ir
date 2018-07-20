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

fn main() {
    let llvm_error = 1;
    
    let mut llvm_builder = LlvmBuilder::new();
    let builder = llvm_builder.builder;

    let mod_name = CString::new("my_module").unwrap();
    let module = unsafe { LLVMModuleCreateWithName(mod_name.as_ptr()) };

    let function = add_function(module, "main", &mut [], int32_type());
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