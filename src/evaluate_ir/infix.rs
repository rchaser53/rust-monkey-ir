use llvm_sys::*;

use parser::expressions::*;
use parser::infix::*;

use evaluate_ir::object::*;

use ir::arithmetic::*;
use ir::condition::*;

pub fn calculate_infix_integer(
    builder: *mut LLVMBuilder,
    infix: Infix,
    left: *mut LLVMValue,
    right: *mut LLVMValue,
    _location: Location,
) -> Object {
    match infix {
        Infix::Plus => Object::Integer(add_variable(builder, left, right, "")),
        Infix::Minus => Object::Integer(sub_variable(builder, left, right, "")),
        Infix::Multiply => Object::Integer(multiple_variable(builder, left, right, "")),
        Infix::Rem => Object::Integer(rem_variable(builder, left, right, "")),
        Infix::Divide => Object::Integer(divide_variable(builder, left, right, "")),
        Infix::Lt => Object::Boolean(build_int_ult(builder, left, right, "")),
        Infix::Lte => Object::Boolean(build_int_ule(builder, left, right, "")),
        Infix::Gt => Object::Boolean(build_int_ugt(builder, left, right, "")),
        Infix::Gte => Object::Boolean(build_int_uge(builder, left, right, "")),
        Infix::Eq => Object::Boolean(build_int_eq(builder, left, right, "")),
        Infix::NotEq => Object::Boolean(build_int_ne(builder, left, right, "")),
    }
}

pub fn calculate_infix_boolean(
    builder: *mut LLVMBuilder,
    infix: Infix,
    left: *mut LLVMValue,
    right: *mut LLVMValue,
    location: Location,
) -> Object {
    match infix {
        Infix::Eq => Object::Boolean(build_int_eq(builder, left, right, "")),
        Infix::NotEq => Object::Boolean(build_int_ne(builder, left, right, "")),
        _ => Object::Error(format!(
            "{} cannot be calculate for boolean. row: {}",
            infix, location.row
        )),
    }
}
