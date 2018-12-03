use lexer::token::*;

use parser::expressions::*;
use parser::infix::*;

pub fn get_expression_llvm_type(expression: &Expression) -> LLVMExpressionType {
    match expression.clone() {
        Expression::IntegerLiteral(_, _) => LLVMExpressionType::Integer,
        Expression::StringLiteral(string, _) => LLVMExpressionType::String(string.len() as u32),
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

pub fn convert_token_to_expression_type(token: Token) -> LLVMExpressionType {
    match token.kind {
        TokenType::LLVMTokenType(llvm_type) => match llvm_type {
            LLVMTokenType::Boolean => LLVMExpressionType::Boolean,
            LLVMTokenType::Integer => LLVMExpressionType::Integer,
            LLVMTokenType::String => {
                // need to include null character(+1)
                let string_length = (token.value.len() + 1) as u32;
                LLVMExpressionType::String(string_length)
            }
            LLVMTokenType::Null => LLVMExpressionType::Null,
        },
        _ => LLVMExpressionType::Null,
    }
}
