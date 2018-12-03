use llvm_sys::*;

use parser::expressions::*;
use parser::prefix::*;

use evaluate_ir::object::*;

use ir::condition::*;
use ir::const_value::*;
use ir::llvm_type::*;
use ir::operate::*;

pub fn calculate_prefix_boolean(
    prefix: Prefix,
    value: *mut LLVMValue,
    location: Location,
) -> Object {
    match prefix {
        Prefix::Bang => {
            if get_u64_from_llvm_value(value) == 1 {
                Object::Boolean(const_int(int1_type(), 0))
            } else {
                Object::Boolean(const_int(int1_type(), 1))
            }
        }
        _ => Object::Error(format!(
            "{} cannot be use for prefix. row: {}",
            prefix, location.row
        )),
    }
}

pub fn calculate_prefix_integer(
    builder: *mut LLVMBuilder,
    prefix: Prefix,
    value: *mut LLVMValue,
) -> Object {
    match prefix {
        Prefix::Minus => Object::Integer(const_neg(value)),
        Prefix::Plus => Object::Integer(value),
        Prefix::Bang => Object::Boolean(build_int_ult(
            builder,
            const_int(int32_type(), 0),
            value,
            "",
        )),
    }
}
