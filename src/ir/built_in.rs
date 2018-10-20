use llvm_sys::*;

use ir::function::*;
use ir::llvm_type::*;

pub fn create_printf(module: *mut LLVMModule) -> *mut LLVMValue {
    let mut printf_args_type_list = vec![pointer_type()];
    let printf_type = function_type(int32_type(), &mut printf_args_type_list);
    return add_function(module, printf_type, "printf");
}
