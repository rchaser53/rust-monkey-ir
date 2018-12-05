use std::collections::HashMap;
use std::ffi::CString;
use std::path::Path;

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
    pub fn emit_file<P: AsRef<Path>>(&self, path: P) {
        let path = path
            .as_ref()
            .to_str()
            .expect("Did not find a valid Unicode path string");
        let mut error: *mut i8 = 0 as *mut i8;
        let buf: *mut *mut i8 = &mut error;
        let result = unsafe { LLVMPrintModuleToFile(self.module, path.as_ptr() as *const _, buf) };
        if result > 0 {
            println!("{}", string_from_raw!(error));
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
