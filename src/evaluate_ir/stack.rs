use llvm_sys::*;

pub struct FunctionStack {
    pub stack: Vec<*mut LLVMValue>,
    pub main_func: *mut LLVMValue,
}

impl FunctionStack {
    pub fn new(main_func: *mut LLVMValue) -> Self {
        FunctionStack {
            stack: Vec::new(),
            main_func: main_func,
        }
    }

    pub fn push(&mut self, function: *mut LLVMValue) {
        self.stack.push(function);
    }

    pub fn pop(&mut self) -> *mut LLVMValue {
        if let Some(function) = self.stack.pop() {
            function
        } else {
            self.main_func
        }
    }

    pub fn last(&mut self) -> *mut LLVMValue {
        if let Some(function) = self.stack.last_mut() {
            *function
        } else {
            self.main_func
        }
    }
}
