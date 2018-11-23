use parser::expressions::*;
use parser::infix::*;

pub fn get_expression_llvm_type(expression: &Expression) -> LLVMExpressionType {
    match expression.clone() {
        Expression::IntegerLiteral(_, _) => LLVMExpressionType::Integer,
        Expression::StringLiteral(_, _) => LLVMExpressionType::String,
        Expression::Boolean(_, _) => LLVMExpressionType::Boolean,
        Expression::Array(expression_type, elements) => {
            LLVMExpressionType::Array(Box::new(expression_type), elements.len() as u32)
        }
        Expression::ArrayElement(_, boxed_element, _) => get_expression_llvm_type(&boxed_element),
        Expression::Infix(infix, left, _, _) => handle_infix_type(infix, *left),
        Expression::Function {
            parameters: _,
            parameter_types: _,
            body: _,
            return_type: _,
            location: _,
        } => LLVMExpressionType::Function,
        Expression::Call(_) => LLVMExpressionType::Call,
        _ => LLVMExpressionType::Null,
    }
}

pub fn handle_infix_type(infix: Infix, left: Expression) -> LLVMExpressionType {
    match infix {
        Infix::Plus => get_expression_llvm_type(&left),
        Infix::Minus => LLVMExpressionType::Integer,
        Infix::Divide => LLVMExpressionType::Integer,
        Infix::Multiply => LLVMExpressionType::Integer,
        Infix::Rem => LLVMExpressionType::Integer,
        Infix::Eq => LLVMExpressionType::Boolean,
        Infix::NotEq => LLVMExpressionType::Boolean,
        Infix::Gte => LLVMExpressionType::Boolean,
        Infix::Gt => LLVMExpressionType::Boolean,
        Infix::Lte => LLVMExpressionType::Boolean,
        Infix::Lt => LLVMExpressionType::Boolean,
    }
}
