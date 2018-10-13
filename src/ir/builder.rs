use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::llvm_type::*;

pub struct LlvmBuilder {
    pub builder: *mut LLVMBuilder,
    pub context: *mut LLVMContext,
    pub module: *mut LLVMModule,
}

impl LlvmBuilder {
    pub fn new(module_name: &str) -> LlvmBuilder {
        unsafe {
            let context = LLVMGetGlobalContext();
            let mod_name = c_string!(module_name);

            LlvmBuilder {
                builder: LLVMCreateBuilderInContext(context),
                module: LLVMModuleCreateWithName(mod_name.as_ptr()),
                context: context,
            }
        }
    }

    pub fn add_function(
        &mut self,
        ret_type: *mut LLVMType,
        args: &mut [*mut LLVMType],
        fn_name: &str,
    ) -> *mut LLVMValue {
        unsafe {
            let fn_type = LLVMFunctionType(ret_type, args.as_mut_ptr(), args.len() as u32, 0);
            let cstring = c_string!(fn_name);
            let ptr = cstring.as_ptr() as *mut _;
            LLVMAddFunction(self.module, ptr, fn_type)
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

    pub fn multiple_variable(
        &mut self,
        var_a: *mut LLVMValue,
        var_b: *mut LLVMValue,
        c_str: CString,
    ) -> *mut LLVMValue {
        unsafe { LLVMBuildMul(self.builder, var_a, var_b, c_str.as_ptr()) }
    }

    pub fn return_variable(&mut self, res: *mut LLVMValue) -> *mut LLVMValue {
        unsafe { LLVMBuildRet(self.builder, res) }
    }

    pub fn setup_main(&mut self) -> *mut LLVMValue {
        let mut main_function = self.add_function(int32_type(), &mut [], "main");
        let block = self.append_basic_block("main", "entry");
        self.end_basic_block(block);
        main_function
    }

    /* need refactoring above */

    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.module) }
    }

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

impl Drop for LlvmBuilder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
        }
    }
}

pub struct BuilderFunctions {}

impl BuilderFunctions {
    pub fn hello_world(
        &mut self,
        builder: *mut LLVMBuilder,
        context: *mut LLVMContext,
        module: *mut LLVMModule,
    ) {
        unsafe {
            let print = self.create_printf(module);
            let mut printf_args = [codegen_string(builder, context, "hello world\n\r")];

            LLVMBuildCall(
                builder,
                print,
                printf_args.as_mut_ptr(),
                1,
                c_string!("").as_ptr(),
            );
        }
    }

    pub fn create_printf(&mut self, module: *mut LLVMModule) -> *mut LLVMValue {
        unsafe {
            let mut printf_args_type_list = [LLVMPointerType(LLVMInt8Type(), 0)];
            let printf_type =
                LLVMFunctionType(LLVMInt32Type(), printf_args_type_list.as_mut_ptr(), 1, 0);
            return LLVMAddFunction(module, c_string!("printf").as_ptr() as *mut _, printf_type);
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
