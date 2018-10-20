use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::const_value::*;
use ir::creator::*;
use ir::llvm_type::*;
use ir::operate::*;
use ir::string::*;
use ir::test_util::*;

pub fn add_function(
    target_module: *mut LLVMModule,
    function_type: *mut LLVMType,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMAddFunction(target_module, c_string!(name).as_ptr(), function_type) }
}

pub fn call_function(
    builder: *mut LLVMBuilder,
    function: *mut LLVMValue,
    mut args: Vec<*mut LLVMValue>,
    name: &str,
) -> *mut LLVMValue {
    unsafe {
        LLVMBuildCall(
            builder,
            function,
            args.as_mut_ptr(),
            args.len() as u32,
            c_string!(name).as_ptr(),
        )
    }
}

#[test]
fn call_function_test() {
    let mut lc = LLVMCreator::new("test_module");
    lc.setup_builtin();

    let main = setup_main(&mut lc);
    let printf = lc.built_ins["printf"];
    let printf_args = vec![codegen_string(&mut lc, "hello world\n\r", "")];

    call_function(lc.builder, printf, printf_args, "");

    build_ret(lc.builder, const_int(int32_type(), 2));

    execute_test_ir_function(lc.module, main);
}
}
