use parser::statements::*;
use parser::expressions::*;

use evalute::object::*;

pub fn eval_statement(statement: Statement) -> Object {
  match statement {
    Statement::Let(ident, expr) => {
      eval_let_staement()
    },
    Statement::Return(expr) => {
      eval_return_statement(expr)
    },
    Statement::Expression(expr) => {
      eval_expression(expr)
    }
  }
}

pub fn eval_let_staement() -> Object {
  Object::Integer(1)
}

pub fn eval_return_statement(expr: Expression) -> Object {
  Object::Integer(1)
}

pub fn eval_expression(expr: Expression) -> Object {
  match expr {
    Expression::IntegerLiteral(int) => {
      Object::Integer(int)
    },
    Expression::Boolean(boolean) => {
      Object::Boolean(boolean)
    },
    Expression::Infix(infix, left, right) => {
      eval_infix(infix, left, right)
    },
    _ => {
      Object::Null
    }
  }
}

pub fn eval_infix(infix: Infix, left: Box<Expression>, right: Box<Expression>) -> Object {
  let left_value = eval_expression(*left);
  let right_value = eval_expression(*right);

  match left_value {
    Object::Integer(left) => {
      match right_value {
        Object::Integer(right) => {
          return Object::Integer(left + right);
        },
        _ => {
          panic!("right value should be integer, but actually {:?}", right_value);
        }
      }
    },
    _ => {
      panic!("left value should be integer, but actually {:?}", left_value);
    }
  }
}

pub fn eval_program(program: Program) -> Vec<Object> {
  let mut objects = Vec::new();
  for statement in program.into_iter() {
    objects.push(eval_statement(statement));
  }
  objects
}