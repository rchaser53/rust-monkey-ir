use llvm_sys::*;

use parser::expressions::*;
use parser::infix::*;

use evaluate_ir::object::*;

use ir::arithmetic::*;
use ir::condition::*;
use ir::converter::*;
use ir::function::*;

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

pub fn resolve_left_integer(
    builder: *mut LLVMBuilder,
    infix: Infix,
    left: *mut LLVMValue,
    right_object: Object,
    location: Location,
) -> Object {
    match right_object {
        Object::Integer(right) => calculate_infix_integer(builder, infix, left, right, location),
        Object::Argument(_, func, index) => {
            let right = get_param(func, index);
            calculate_infix_integer(builder, infix, left, right, location)
        }
        _ => Object::Error(format!(
            "right value should be integer, but actually {}. row: {}",
            right_object, location.row,
        )),
    }
}

pub fn resolve_left_boolean(
    builder: *mut LLVMBuilder,
    infix: Infix,
    left: *mut LLVMValue,
    right_object: Object,
    location: Location,
) -> Object {
    match right_object {
        Object::Boolean(right) => calculate_infix_boolean(builder, infix, left, right, location),
        Object::Argument(_, func, index) => {
            let right = get_param(func, index);
            calculate_infix_boolean(builder, infix, left, right, location)
        }
        _ => Object::Error(format!(
            "right value should be boolean, but actually {}. row: {}",
            right_object, location.row,
        )),
    }
}

pub fn resolve_left_argument(
    builder: *mut LLVMBuilder,
    infix: Infix,
    expression_type_left: LLVMExpressionType,
    left: *mut LLVMValue,
    right_object: Object,
    location: Location,
) -> Object {
    match right_object {
        Object::Integer(right) => calculate_infix_integer(builder, infix, left, right, location),
        Object::Boolean(right) => calculate_infix_boolean(builder, infix, left, right, location),
        Object::Argument(_, func, index) => {
            let right = get_param(func, index);
            match wrap_llvm_value(expression_type_left.clone(), right) {
                Object::Integer(_) => {
                    calculate_infix_integer(builder, infix, left, right, location)
                }
                Object::Boolean(_) => {
                    calculate_infix_boolean(builder, infix, left, right, location)
                }
                _ => Object::Error(format!(
                    "right cannot be analyzed, but actually {:?}. row: {}", // TODO
                    expression_type_left, location.row,
                )),
            }
        }
        _ => Object::Error(format!(
            "right value should be boolean, but actually {}. row: {}",
            right_object, location.row,
        )),
    }
}

// TODO
pub fn resolve_left_string(
    _infix: Infix,
    left: *mut LLVMValue,
    right_object: Object,
    location: Location,
) -> Object {
    match right_object {
        Object::String(_, length) => Object::String(left, length), // TODO
        _ => Object::Error(format!(
            "right value should be string, but actually {}. row: {}",
            right_object, location.row,
        )),
    }
}

pub fn resolve_left_failed(
    infix: Infix,
    left_object: Object,
    right_object: Object,
    location: Location,
) -> Object {
    let right_type_str = match right_object {
        Object::Integer(_) => "integer",
        Object::String(_, _) => "string",
        Object::Boolean(_) => "boolean",
        _ => {
            return Object::Error(format!(
                "{} {} {} cannot be culculated. row: {}",
                left_object, infix, right_object, location.row,
            ));
        }
    };
    Object::Error(format!(
        "left value should be {}, but actually {}. row: {}",
        right_type_str, left_object, location.row
    ))
}
