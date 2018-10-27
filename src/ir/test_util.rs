use std::ffi::CString;
use std::os::raw::c_char;

use llvm_sys::execution_engine::*;
use llvm_sys::*;

use ir::block::*;
use ir::creator::*;
use ir::function::*;
use ir::llvm_type::*;
use ir::operate::*;
use ir::validate::*;

const LLVM_ERROR: i32 = 1;

#[allow(dead_code)]
fn create_llvm_engine(module: *mut LLVMModule) -> LLVMExecutionEngineRef {
    let error: *mut c_char = 0 as *mut c_char;
    let mut engine: LLVMExecutionEngineRef = 0 as LLVMExecutionEngineRef;
    let ok = unsafe {
        let mut error = 0 as *mut c_char;
        let buf: *mut *mut c_char = &mut error;
        let engine_ref: *mut LLVMExecutionEngineRef = &mut engine;
        LLVMLinkInInterpreter();
        LLVMCreateInterpreterForModule(engine_ref, module, buf)
    };
    if ok == LLVM_ERROR {
        let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
        panic!("Execution error: {}", err_msg);
    }
    validate_module(module);
    engine
}

#[allow(dead_code)]
pub fn execute_test_ir_function(module: *mut LLVMModule, target_function: *mut LLVMValue) -> u64 {
    let engine = create_llvm_engine(module);
    let mut params = [];
    let func_result = run_function(
        engine,
        target_function,
        params.len() as u32,
        params.as_mut_ptr(),
    );

    unsafe {
        LLVMDisposeExecutionEngine(engine);
        LLVMGenericValueToInt(func_result, 0)
    }
}

#[allow(dead_code)]
pub fn setup_main(lc: &mut LLVMCreator) -> *mut LLVMValue {
    let fn_type = function_type(int32_type(), &mut []);
    let main_function = add_function(lc.module, fn_type, "main");
    let block = append_basic_block_in_context(lc.context, main_function, "entry");
    build_position_at_end(lc.builder, block);
    main_function
}
