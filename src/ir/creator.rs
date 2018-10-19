use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

pub struct LLVMCreator {
    pub builder: *mut LLVMBuilder,
    pub context: *mut LLVMContext,
    pub module: *mut LLVMModule,
}

impl LLVMCreator {
    pub fn new(module_name: &str) -> LLVMCreator {
        unsafe {
            let context = LLVMContextCreate();
            let mod_name = c_string!(module_name);

            LLVMCreator {
                builder: LLVMCreateBuilderInContext(context),
                module: LLVMModuleCreateWithName(mod_name.as_ptr()),
                context: context,
            }
        }
    }

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
            LLVMContextDispose(self.context);
        }
    }
}
