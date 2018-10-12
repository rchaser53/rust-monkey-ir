#[macro_use]
mod macros;

#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate libc;
extern crate llvm_sys;
extern crate rustc_llvm_proxy;

use std::fs::File;
use std::io::prelude::*;

use clap::{App, Arg};

mod lexer;
use lexer::lexer::*;

mod parser;
use parser::parser::*;

mod evalute;
use evalute::environment::*;
use evalute::eval::*;

mod ir;
use ir::builder::*;
use ir::llvm_type::*;
use ir::arithmetic::*;
use ir::condition::*;

use llvm_sys::core::*;
use llvm_sys::*;
use llvm_sys::execution_engine::*;

fn read_file(file_name: &str) -> Result<String, String> {
    if let Ok(mut file) = File::open(file_name) {
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents)
            .map_err(|err| format!("{}", err))?;
        Ok(contents)
    } else {
        Err(format!("failed to open: {}", file_name))
    }
}

fn main() {
    let matches = App::new("rust-monkey-ir")
        .version("1.0")
        .author("rchaser53. <tayoshizawa29@gmail.com>")
        .arg(Arg::with_name("input_file").index(1))
        .get_matches();

    let file_name = matches.value_of("input_file").unwrap_or("index.mr");
    match read_file(file_name) {
        Ok(input) => {
            let mut lexer = Lexer::new(&input);

            let mut parser = Parser::new(&mut lexer);
            let program = parser.parse_program();
            if parser.has_error() {
                panic!("{}", parser.emit_error());
            }

            let mut eval = Eval::new();
            let result_value = eval.eval_program(program, &mut Environment::new());
            if eval.has_error() {
                panic!("{}", eval.emit_error());
            }

            println!("{:?}", result_value);
        }
        Err(error) => {
            panic!("{}", error);
        }
    };
}

// use std::ffi::CString;
// use llvm_sys::LLVMIntPredicate;
// use llvm_sys::analysis::{LLVMVerifyModule, LLVMVerifierFailureAction};

// use std::os::raw::{c_char};

//     unsafe {
//       let mut lb = LlvmBuilder::new("test_module");
//       let main = lb.add_function(int32_type(), &mut [], "main");
//       // let hoge_func = lb.add_function(int32_type(), &mut [], "hoge_func");

//       let block = lb.append_basic_block("main", "entry");
//       build_position_at_end(lb.builder, block);

//       let llvm_value = LLVMBuildAlloca(
//           lb.builder,
//           int32_type(),
//           c_string!("").as_ptr(),
//       );

//       let left_block = append_basic_block(main, "");
//       let right_block = append_basic_block(main, "");
//       let mut llvm_bool = LLVMConstInt(LLVMInt1Type(), 0, 0);

//       build_cond_br(lb.builder, llvm_bool, left_block, right_block);
//       build_position_at_end(lb.builder, left_block);
//       LLVMBuildStore(lb.builder, LLVMConstInt(int32_type(), 11, 0), llvm_value);
//       let hoge = LLVMBuildLoad(
//           lb.builder,
//           llvm_value,
//           c_string!("").as_ptr(),
//       );
//       lb.return_variable(hoge);

//       build_br(lb.builder, right_block);
//       build_position_at_end(lb.builder, right_block);
//       LLVMBuildStore(lb.builder, LLVMConstInt(int32_type(), 12, 0), llvm_value);
//       let hoge2 = LLVMBuildLoad(
//           lb.builder,
//           llvm_value,
//           c_string!("").as_ptr(),
//       );

//       lb.return_variable(hoge2);

//       let mut error: *mut c_char = 0 as *mut c_char;
//       let mut engine: LLVMExecutionEngineRef = 0 as LLVMExecutionEngineRef;
//       let ok = unsafe {
//           error = 0 as *mut c_char;
//           let buf: *mut *mut c_char = &mut error;
//           let engine_ref: *mut LLVMExecutionEngineRef = &mut engine;
//           LLVMLinkInInterpreter();
//           LLVMCreateInterpreterForModule(engine_ref, lb.module, buf)
//       };
//       let ok = unsafe {
//           let buf: *mut *mut c_char = &mut error;
//           LLVMVerifyModule(lb.module, LLVMVerifierFailureAction::LLVMReturnStatusAction, buf)
//       };
//       if ok == 1 {
//           let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
//           panic!("cannot verify module '{:?}'.\nError: {}", CString::new("test_module").unwrap(), err_msg);
//       }

//       if ok == 1 {
//         let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
//         println!("Execution error: {}", err_msg);
//       }

//       let mut params = [];
//       let named_function = unsafe { LLVMGetNamedFunction(lb.module, CString::new("main").unwrap().as_ptr()) };
//       let func_result = LLVMRunFunction(
//           engine,
//           main,
//           params.len() as u32,
//           params.as_mut_ptr(),
//       );
//       let result = LLVMGenericValueToInt(func_result, 0);

//       println!("{:?}", result);

//       lb.dump();
//     }
