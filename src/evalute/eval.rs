use std::collections::HashMap;

use lexer::lexer::*;

use parser::expressions::*;
use parser::parser::*;
use parser::statements::*;

use evalute::object::*;

pub struct Eval {
  pub store: HashMap<String, Object>
}

impl Eval {
    pub fn new() -> Self {
        Eval {
          store: HashMap::new()
        }
    }

    pub fn eval_program(&mut self, program: Program) -> Object {
        for statement in program.into_iter() {
          if let Some(obj) = self.eval_statement(statement) {
            return obj;
          }
        }
        Object::Null
    }

    pub fn eval_statement(&mut self, statement: Statement) -> Option<Object> {
        match statement {
            Statement::Let(ident, expr) => {
              self.eval_let_staement(ident, expr);
              None
            },
            Statement::Return(expr) => {
              Some(self.eval_return_statement(expr))
            },
            Statement::Expression(expr) => {
              self.eval_expression(expr);
              None
            },
        }
    }

    pub fn eval_let_staement(&mut self, ident: Identifier, expr: Expression) -> Object {
      let value = self.eval_expression(expr);
      self.store.insert(
        ident.0,
        value.clone()
      );

      value
    }

    pub fn eval_return_statement(&mut self, expr: Expression) -> Object {
        self.eval_expression(expr)
    }

    pub fn eval_expression(&mut self, expr: Expression) -> Object {
        match expr {
            Expression::IntegerLiteral(int) => Object::Integer(int),
            Expression::Boolean(boolean) => Object::Boolean(boolean),
            Expression::Prefix(prefix, expr) => self.eval_prefix(prefix, expr),
            Expression::Infix(infix, left, right) => self.eval_infix(infix, left, right),
            Expression::If{condition, consequence, alternative } => self.eval_if(condition, consequence, alternative),
            Expression::Identifier(ident) => self.eval_identifier(ident),
            _ => Object::Null,
        }
    }

    pub fn eval_identifier(&self, ident: Identifier) -> Object {
      self.store[&ident.0].clone()
    }

    pub fn eval_prefix(&mut self, prefix: Prefix, expr: Box<Expression>) -> Object {
        let expr_value = self.eval_expression(*expr);

        match expr_value {
            Object::Integer(expr) => self.calculate_prefix_integer(prefix, expr),
            Object::Boolean(expr) => self.calculate_prefix_boolean(prefix, expr),
            _ => {
                panic!(
                    "expr value should be integer, but actually {:?}",
                    expr_value
                );
            }
        }
    }

    pub fn eval_infix(
        &mut self,
        infix: Infix,
        left: Box<Expression>,
        right: Box<Expression>,
    ) -> Object {
        let left_value = self.eval_expression(*left);
        let right_value = self.eval_expression(*right);

        match left_value {
            Object::Integer(left) => match right_value {
                Object::Integer(right) => self.calculate_infix_integer(infix, left, right),
                _ => {
                    panic!(
                        "right value should be integer, but actually {:?}",
                        right_value
                    );
                }
            },
            _ => {
                panic!(
                    "left value should be integer, but actually {:?}",
                    left_value
                );
            }
        }
    }

    pub fn eval_if(
      &mut self,
      condition: Box<Expression>,
      consequence: BlockStatement,
      alternative: Option<BlockStatement>,
    ) -> Object {
      let condition_obj = self.eval_expression(*condition);

      match condition_obj {
        Object::Boolean(boolean) => {
          if boolean {
            self.eval_program(consequence);
          }
          if let Some(alt) = alternative {
            self.eval_program(alt);
          }
        },
        _ => {
            panic!("condition should be boolean. actually {:?}", condition_obj);
        }
      };
      Object::Null
    }

    pub fn calculate_prefix_boolean(&self, prefix: Prefix, value: bool) -> Object {
        match prefix {
            Prefix::Bang => Object::Boolean(!value),
            _ => {
                panic!("{:?} cannot be use for prefix", prefix);
            }
        }
    }

    pub fn calculate_prefix_integer(&self, prefix: Prefix, value: i64) -> Object {
        match prefix {
            Prefix::Minus => Object::Integer(-1 * value),
            Prefix::Plus => Object::Integer(value),
            Prefix::Bang => {
                if value < 0 {
                    Object::Boolean(true)
                } else {
                    Object::Boolean(false)
                }
            }
        }
    }

    pub fn calculate_infix_integer(&self, infix: Infix, left: i64, right: i64) -> Object {
        match infix {
            Infix::Plus => Object::Integer(left + right),
            Infix::Minus => Object::Integer(left - right),
            Infix::Multiply => Object::Integer(left * right),
            Infix::Divide => Object::Integer(left / right),
            _ => {
                panic!("{:?} cannot be calculate for integer", infix);
            }
        }
    }
}

#[warn(dead_code)]
fn compile_input(input: &str) -> Vec<Statement> {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    parser.parse_program()
}

#[test]
fn eval_integer() {
    let input = "
  return 1;
";
    let statements = compile_input(input);

    let mut eval = Eval::new();
    let objects = eval.eval_program(statements);

    assert!("1" == format!("{}", objects));
}

#[test]
fn eval_boolean() {
    let input = "
  return true;
";
    let statements = compile_input(input);

    let mut eval = Eval::new();
    let objects = eval.eval_program(statements);

    assert!("true" == format!("{}", objects));
}

#[test]
fn eval_null() {
    let input = "
  let a = 1;
";
    let statements = compile_input(input);

    let mut eval = Eval::new();
    let objects = eval.eval_program(statements);

    assert!("Null" == format!("{}", objects));
}

#[test]
fn eval_let() {
    let input = "
  let a = 1;
  let b = 2;
  return a + b;
";
    let statements = compile_input(input);

    let mut eval = Eval::new();
    let objects = eval.eval_program(statements);

    assert!("3" == format!("{}", objects));
}