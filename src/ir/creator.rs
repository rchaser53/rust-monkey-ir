use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::llvm_type::*;
use ir::function::*;

pub struct LLVMCreator {
    pub builder: *mut LLVMBuilder,
    pub context: *mut LLVMContext,
    pub module: *mut LLVMModule,
}

impl LLVMCreator {
    pub fn new(module_name: &str) -> LLVMCreator {
        unsafe {
            let context = LLVMGetGlobalContext();
            let mod_name = c_string!(module_name);

            LLVMCreator {
                builder: LLVMCreateBuilderInContext(context),
                module: LLVMModuleCreateWithName(mod_name.as_ptr()),
                context: context,
            }
        }
    }

    pub fn get_named_function(&mut self, name: &str) -> *mut LLVMValue {
        let func_name = c_string!(name);
        unsafe { LLVMGetNamedFunction(self.module, func_name.as_ptr()) }
    }

    pub fn append_basic_block(&mut self, function_name: &str, name: &str) -> *mut LLVMBasicBlock {
        let label_name = c_string!(name);
        let function = self.get_named_function(function_name);

        unsafe { LLVMAppendBasicBlock(function, label_name.as_ptr()) }
    }

    pub fn end_basic_block(&mut self, block: *mut LLVMBasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, block);
        }
    }

    /* need refactoring below */

    pub fn create_int_variable(
        &mut self,
        name: &str,
        value: u64,
        llvm_type: *mut LLVMType,
    ) -> *mut LLVMValue {
        let val_name = c_string!(name);
        let llvm_value = unsafe { LLVMBuildAlloca(self.builder, llvm_type, val_name.as_ptr()) };
        unsafe {
            LLVMBuildStore(self.builder, LLVMConstInt(llvm_type, value, 0), llvm_value);
        }
        unsafe { LLVMBuildLoad(self.builder, llvm_value, val_name.as_ptr()) }
    }

    pub fn setup_main(&mut self) -> *mut LLVMValue {
        let fn_type = create_function_type(int32_type(), &mut []);
        let mut main_function = add_function(self.module, fn_type, "main");
        let block = self.append_basic_block("main", "entry");
        self.end_basic_block(block);
        main_function
    }

    /* need refactoring above */

    #[allow(dead_code)]
    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.module) }
    }

    #[allow(dead_code)]
    pub fn emit_file(&self, filename: &str) {
        unsafe {
            LLVMPrintModuleToFile(
                self.module,
                c_string!(filename).as_ptr(),
                c_string!("").as_ptr() as *mut _,
            );
        }
    }
}

impl Drop for LLVMCreator {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
        }
    }
}

pub fn codegen_string(
    builder: *mut LLVMBuilder,
    context: *mut LLVMContext,
    input_str: &str,
) -> *mut LLVMValue {
    let length = input_str.len() as u32;
    unsafe {
        let str_val =
            LLVMConstStringInContext(context, c_string!(input_str).as_ptr(), length - 1, 0);
        let llvm_value = LLVMBuildAlloca(
            builder,
            LLVMArrayType(LLVMInt8Type(), length),
            c_string!("").as_ptr(),
        );
        LLVMBuildStore(builder, str_val, llvm_value);

        let mut args = [
            LLVMConstInt(LLVMInt32Type(), 0, 0),
            LLVMConstInt(LLVMInt32Type(), 0, 0),
        ];

        return LLVMBuildGEP(
            builder,
            llvm_value,
            args.as_mut_ptr(),
            2,
            c_string!("").as_ptr(),
        );
    }
}
