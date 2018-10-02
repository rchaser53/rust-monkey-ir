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

    pub fn eval_program(&mut self, program: Program, env: &mut Environment) -> Object {
        for statement in program.into_iter() {
            if let Some(obj) = self.eval_statement(statement, env) {
                return obj;
            }
        }
        Object::Null
    }

    pub fn eval_statement(
        &mut self,
        statement: Statement,
        env: &mut Environment,
    ) -> Option<Object> {
        match statement {
            Statement::Let(ident, expr) => {
                self.eval_let_staement(ident, expr, env);
                None
            }
            Statement::Return(expr) => Some(self.eval_return_statement(expr, env)),
            Statement::Expression(expr) => match expr {
                Expression::If {
                    condition,
                    consequence,
                    alternative,
                } => self.eval_if(condition, consequence, alternative, env),
                _ => {
                    self.eval_expression(expr, env);
                    None
                }
            },
        }
    }

    pub fn eval_let_staement(
        &mut self,
        ident: Identifier,
        expr: Expression,
        env: &mut Environment,
    ) -> Object {
        let value = self.eval_expression(expr, env);
        env.set(ident.0, value)
    }

    pub fn eval_return_statement(&mut self, expr: Expression, env: &mut Environment) -> Object {
        self.eval_expression(expr, env)
    }

    pub fn eval_expression(&mut self, expr: Expression, env: &mut Environment) -> Object {
        match expr {
            Expression::IntegerLiteral(int) => Object::Integer(int),
            Expression::StringLiteral(string) => Object::String(string),
            Expression::Boolean(boolean) => Object::Boolean(boolean),
            Expression::Prefix(prefix, expr) => self.eval_prefix(prefix, expr, env),
            Expression::Infix(infix, left, right) => self.eval_infix(infix, left, right, env),
            Expression::Identifier(ident) => self.eval_identifier(ident, env),
            Expression::Function { parameters, body } => Object::Function(Function{
                parameters: parameters,
                body: body,
                env: env.clone(),
            }),
            Expression::Call(Call{
                function,
                arguments,
            }) => self.eval_call(function, arguments, env),
            _ => Object::Null,
        }
    }

    pub fn eval_call(
        &mut self,
        outer_function: Box<Expression>,
        outer_arguments: Vec<Expression>,
        outer_env: &mut Environment,
    ) -> Object {
        match *outer_function {
            Expression::Identifier(Identifier(ref string)) => {
                let mut call_function = outer_env.get(string);
                self.exec_func(call_function, outer_arguments, outer_env)
            },
            Expression::Call(call) => {
              match *call.function {
                Expression::Identifier(Identifier(ref string)) => {
                  let mut call_function = outer_env.get(string);
                  let maybe_func = self.exec_func(call_function, call.arguments, outer_env);
                  self.exec_func(maybe_func, outer_arguments, outer_env)
                },
                Expression::Call(inner_call) => {
                  let maybe_func = self.eval_call(inner_call.function, inner_call.arguments.clone(), outer_env);
                  self.exec_func(maybe_func, outer_arguments, outer_env)
                },
                _ => {
                  panic!("[in] cannot call {:?}", call.function);
                }
              }
            },
            _ => {
                panic!("[out] cannot call {:?}", outer_function);
            }
        }
    }

    pub fn exec_func(
      &mut self,
      call_function: Object,
      outer_arguments: Vec<Expression>,
      outer_env: &mut Environment,
    ) -> Object {
        match call_function {
            Object::Function(func) => {
                let mut func_env = func.env.clone();
                for (index, parameter) in func.parameters.into_iter().enumerate() {
                    let actual_param =
                        self.eval_expression(outer_arguments[index].clone(), outer_env);
                    func_env.set(parameter.0.to_string(), actual_param);
                }
                self.eval_program(func.body, &mut func_env)
            }
            _ => {
                call_function
            }
        }
    }

    pub fn eval_identifier(&self, ident: Identifier, env: &mut Environment) -> Object {
        env.get(&ident.0)
    }

    pub fn eval_prefix(
        &mut self,
        prefix: Prefix,
        expr: Box<Expression>,
        env: &mut Environment,
    ) -> Object {
        let expr_value = self.eval_expression(*expr, env);

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
        env: &mut Environment,
    ) -> Object {
        let left_value = self.eval_expression(*left, env);
        let right_value = self.eval_expression(*right, env);

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
            Object::String(left) => match right_value {
                Object::String(right) => Object::String(left + &right),
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
        env: &mut Environment,
    ) -> Option<Object> {
        let condition_obj = self.eval_expression(*condition, env);
        let mut return_obj = Object::Null;

        match condition_obj {
            Object::Boolean(boolean) => {
                if boolean {
                    return_obj = self.eval_program(consequence, env);
                }
                if let Some(alt) = alternative {
                    return_obj = self.eval_program(alt, env);
                }
            }
            _ => {
                panic!("condition should be boolean. actually {:?}", condition_obj);
            }
        };

        match return_obj {
            Object::Null => None,
            _ => Some(return_obj),
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
            Infix::Lt => Object::Boolean(left < right),
            Infix::Lte => Object::Boolean(left <= right),
            Infix::Gt => Object::Boolean(left > right),
            Infix::Gte => Object::Boolean(left >= right),
            _ => {
                panic!("{:?} cannot be calculate for integer", infix);
            }
        }
    }
}

#[warn(dead_code)]
fn compile_input(input: &str) -> Object {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let statements = parser.parse_program();
    let mut eval = Eval::new();
    eval.eval_program(statements, &mut Environment::new())
}

#[test]
fn eval_integer() {
    let input = r#"
  return 1;
"#;
    assert!("1" == format!("{}", compile_input(input)));
}

#[test]
fn eval_string() {
    let input = "
  return \"abc\";
";
    assert!("abc" == format!("{}", compile_input(input)));
}

#[test]
fn eval_integer_with_paren() {
    let input = r#"
    return (1 + 3 * 2) * (2 - 1);
  "#;

    assert!("7" == format!("{}", compile_input(input)));
}

#[test]
fn eval_infix_gte() {
    let input = r#"
    return 3 >= (5 - 1);
  "#;

    assert!("false" == format!("{}", compile_input(input)));
}

#[test]
fn eval_infix_string() {
    let input = r#"
    return "hello " + "world";
  "#;

    assert!("hello world" == format!("{}", compile_input(input)));
}

#[test]
fn eval_boolean() {
    let input = r#"
  return true;
"#;
    assert!("true" == format!("{}", compile_input(input)));
}

#[test]
fn eval_null() {
    let input = r#"
  let a = 1;
"#;
    assert!("Null" == format!("{}", compile_input(input)));
}

#[test]
fn eval_let() {
    let input = r#"
  let a = 1;
  let b = 2;
  return a + b;
"#;
    assert!("3" == format!("{}", compile_input(input)));
}

#[test]
fn eval_if() {
    let input = r#"
  if (true) {
    return 3;
  }
"#;
    assert!("3" == format!("{}", compile_input(input)));
}

#[test]
fn eval_else() {
    let input = r#"
  if (false) {
    return 1;
  } else {
    return 3;
  }
"#;
    assert!("3" == format!("{}", compile_input(input)));
}

#[test]
fn eval_no_return_if() {
    let input = r#"
  if (1 < 3) {
    let a = 1;
  }
  return 3;
"#;
    assert!("3" == format!("{}", compile_input(input)));
}

#[test]
fn eval_function() {
    let input = r#"
  let hoge = fn(a) {
    return a + 3;
  };

  return hoge(1) + 3;
"#;
    assert!("7" == format!("{}", compile_input(input)));
}

#[test]
fn eval_closure() {
    let input = r#"
  let a = 5;
  let b = 2;
  let hoge = fn(a) {
    return a + b;
  };

  return hoge(1) + 3;
"#;
    assert!("6" == format!("{}", compile_input(input)));
}

#[test]
fn eval_return_function() {
    let input = r#"
  let ho = fn(a) {
    return fn(b) {
      return a + b;
    };
  };
  let hoi = ho(1);

  return hoi(2);
"#;
    assert!("3" == format!("{}", compile_input(input)));
}

#[test]
fn eval_returned_function() {
    let input = r#"
    let x = fn(a) {
      return fn(b) {
        return a + b;
      };
    };
    return x(1)(2);
  "#;
    assert!("3" == format!("{}", compile_input(input)));
}