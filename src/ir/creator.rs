use std::collections::HashMap;
use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use ir::built_in::*;

pub struct LLVMCreator {
    pub builder: *mut LLVMBuilder,
    pub context: *mut LLVMContext,
    pub module: *mut LLVMModule,
    pub built_ins: HashMap<&'static str, *mut LLVMValue>,
}

impl LLVMCreator {
    pub fn new(module_name: &str) -> LLVMCreator {
        unsafe {
            let context = LLVMContextCreate();
            let mod_name = c_string!(module_name);

            let mut lc = LLVMCreator {
                builder: LLVMCreateBuilderInContext(context),
                module: LLVMModuleCreateWithName(mod_name.as_ptr()),
                context: context,
                built_ins: HashMap::new(),
            };
            lc.setup_builtin();
            lc
        }
    }

    #[allow(dead_code)]
    pub fn setup_builtin(&mut self) {
        self.built_ins.insert("printf", create_printf(self.module));
        self.built_ins.insert("strcmp", create_strcmp(self.module));
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
