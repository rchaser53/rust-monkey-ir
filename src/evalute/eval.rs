use lexer::lexer::*;

use parser::expressions::*;
use parser::parser::*;
use parser::statements::*;

use evalute::object::*;

pub struct Eval {}

impl Eval {
    pub fn new() -> Self {
        Eval {}
    }

    pub fn eval_program(&self, program: Program) -> Vec<Object> {
        let mut objects = Vec::new();
        for statement in program.into_iter() {
            objects.push(self.eval_statement(statement));
        }
        objects
    }

    pub fn eval_statement(&self, statement: Statement) -> Object {
        match statement {
            Statement::Let(ident, expr) => self.eval_let_staement(),
            Statement::Return(expr) => self.eval_return_statement(expr),
            Statement::Expression(expr) => self.eval_expression(expr),
        }
    }

    pub fn eval_let_staement(&self) -> Object {
        Object::Integer(1)
    }

    pub fn eval_return_statement(&self, expr: Expression) -> Object {
        Object::Integer(1)
    }

    pub fn eval_expression(&self, expr: Expression) -> Object {
        match expr {
            Expression::IntegerLiteral(int) => Object::Integer(int),
            Expression::Boolean(boolean) => Object::Boolean(boolean),
            Expression::Prefix(prefix, expr) => self.eval_prefix(prefix, expr),
            Expression::Infix(infix, left, right) => self.eval_infix(infix, left, right),
            _ => Object::Null,
        }
    }

    pub fn eval_prefix(&self, prefix: Prefix, expr: Box<Expression>) -> Object {
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
        &self,
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
fn integer() {
    let input = "
  1;
  1 + 2;
  3 - 1;
";
    let statements = compile_input(input);

    let mut eval = Eval::new();
    let objects = eval.eval_program(statements);

    assert!("1" == format!("{}", objects[0]));
    assert!("3" == format!("{}", objects[1]));
    assert!("2" == format!("{}", objects[2]));
}

#[test]
fn boolean() {
    let input = "
  true;
  false;
  !true;
  !false;
";
    let statements = compile_input(input);

    let mut eval = Eval::new();
    let objects = eval.eval_program(statements);

    assert!("true" == format!("{}", objects[0]));
    assert!("false" == format!("{}", objects[1]));
    assert!("false" == format!("{}", objects[2]));
    assert!("true" == format!("{}", objects[3]));
}
