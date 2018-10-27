use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::block::*;
use ir::condition::*;
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

pub fn get_param(target_func: *mut LLVMValue, arg_index: u32) -> *mut LLVMValue {
    unsafe { LLVMGetParam(target_func, arg_index) }
}

pub fn create_function<F>(
    lc: &mut LLVMCreator,
    fn_type: *mut LLVMType,
    implement: F,
) -> *mut LLVMValue
where
    F: Fn(&mut LLVMCreator, *mut LLVMValue),
{
    let function = add_function(lc.module, fn_type, "");
    let block = append_basic_block(function, "entry");
    build_position_at_end(lc.builder, block);
    implement(lc, function);
    function
}

#[test]
fn call_printf() {
    let mut lc = LLVMCreator::new("test_module");
    lc.setup_builtin();

    let main = setup_main(&mut lc);
    let printf = lc.built_ins["printf"];
    let printf_args = vec![codegen_string(&mut lc, "hello world\n\r", "")];

    call_function(lc.builder, printf, printf_args, "");

    build_ret(lc.builder, const_int(int32_type(), 2));

    execute_test_ir_function(lc.module, main);
}

#[test]
fn call_strcmp() {
    let mut lc = LLVMCreator::new("test_module");
    lc.setup_builtin();

    let main = setup_main(&mut lc);
    let strcmp = lc.built_ins["strcmp"];
    let printf = lc.built_ins["printf"];
    let mut strcmp_args = vec![
        codegen_string(&mut lc, "hello world", ""),
        codegen_string(&mut lc, "hello world", ""),
    ];

    let llvm_value = build_alloca(lc.builder, int32_type(), "");
    let called = call_function(lc.builder, strcmp, strcmp_args, "");
    build_store(lc.builder, called, llvm_value);
    let print_int = build_load(lc.builder, llvm_value, "");

    let printf_args = vec![codegen_string(&mut lc, "resulte %d", ""), print_int];

    call_function(lc.builder, printf, printf_args, "");

    build_ret(lc.builder, const_int(int32_type(), 0));
    // LLVM ERROR: Calling external var arg function 'strcmp' is not supported by the Interpreter.
    // let for_assert = execute_test_ir_function(lc.module, main);
}

#[test]
fn call_int_func() {
    let mut lc = LLVMCreator::new("test_module");
    let test_fn_type = function_type(int32_type(), &mut [int32_type()]);
    let test_func = create_function(&mut lc, test_fn_type, |inner_lc, test_func| {
        build_ret(inner_lc.builder, get_param(test_func, 0));
    });

    let main = setup_main(&mut lc);
    let mut test_func_args = vec![const_int(int32_type(), 10)];
    let called = call_function(lc.builder, test_func, test_func_args, "");
    build_ret(lc.builder, called);

    let for_assert = execute_test_ir_function(lc.module, main);
    let expected = 10;
    assert!(
        for_assert == expected,
        "test failed \r\nexpected: {}\r\nactual:{}",
        expected,
        for_assert
    );
}
