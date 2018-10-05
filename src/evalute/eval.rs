use lexer::lexer::*;

use parser::expressions::*;
use parser::parser::*;
use parser::statements::*;

use evalute::object::*;

pub struct Eval {
  pub stack_arg: Vec<Vec<Expression>>,
  pub error_stack: Vec<Object>
}

impl Eval {
    pub fn new() -> Self {
        Eval {
          stack_arg: Vec::new(),
          error_stack: Vec::new(),
        }
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
                let obj = self.eval_let_staement(ident, expr, env);
                let _ = self.accumultae_error(obj);
                None
            },
            Statement::Return(expr) => {
              let obj = self.eval_return_statement(expr, env);
              self.accumultae_error(obj)
            },
            Statement::Expression(expr) => match expr {
                Expression::If {
                    condition,
                    consequence,
                    alternative,
                } => {
                  let obj = self.eval_if(condition, consequence, alternative, env);
                  if let Some(obj) = obj {
                    self.accumultae_error(obj)
                  } else {
                    None
                  }
                },
                _ => {
                    let obj = self.eval_expression(expr, env);
                    let _ = self.accumultae_error(obj);
                    None
                }
            },
        }
    }

    pub fn accumultae_error(&mut self, obj: Object) -> Option<Object> {
        match obj {
          Object::Error(_) => {
            self.error_stack.push(obj);
            None
          },
          _ => Some(obj),
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
              self.stack_arg.push(outer_arguments);
              self.call_func(call.clone(), call.arguments, outer_env)
            },
            _ => {
                Object::Error(format!("cannot call {}", outer_function.string()))
            }
        }
    }

    pub fn call_func(
      &mut self,
      call: Call,
      outer_arguments: Vec<Expression>,
      outer_env: &mut Environment,
    ) -> Object {
        match *call.function {
          Expression::Identifier(Identifier(ref string)) => {
            let mut call_function = outer_env.get(string);
            self.stack_arg.push(call.arguments);

            while let Some(arg) = self.stack_arg.pop() {
              call_function = self.exec_func(call_function, arg, outer_env);

              match call_function {
                Object::Function(_) => {
                  continue;
                },
                _ => {
                  return call_function;
                }
              }
            }
            call_function
          },
          Expression::Call(inner_call) => {
            self.stack_arg.push(outer_arguments);
            self.call_func(inner_call.clone(), inner_call.arguments, outer_env)
          },
          _ => {
            Object::Error(format!("cannot call {}", call.function.string()))
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
                for (index, Identifier(string)) in func.parameters.into_iter().enumerate() {
                    let actual_param =
                        self.eval_expression(outer_arguments[index].clone(), outer_env);
                    func_env.set(string, actual_param);
                }
                self.eval_program(func.body, &mut func_env)
            }
            _ => call_function
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
            _ => Object::Error(format!("expr value should be integer, but actually {}", expr_value))
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
                _ => Object::Error(format!("right value should be integer, but actually {}", right_value))
            },
            Object::String(left) => match right_value {
                Object::String(right) => Object::String(left + &right),
                _ => Object::Error(format!("right value should be integer, but actually {}", right_value))
            },
            _ => Object::Error(format!("left value should be integer, but actually {}", left_value))
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
                return_obj = Object::Error(format!("condition should be boolean. actually {}", condition_obj));
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
            _ => Object::Error(format!("{} cannot be use for prefix", prefix)),
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
            _ => Object::Error(format!("{} cannot be calculate for integer", infix)),
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

#[warn(dead_code)]
fn compile_and_emit_error(input: &str, error_messages: Vec<&str>) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let statements = parser.parse_program();
    let mut eval = Eval::new();
    eval.eval_program(statements, &mut Environment::new());
    
    let error_stack = eval.error_stack;

    for (index, error_message) in error_messages.into_iter().enumerate() {
        assert!(
            format!("{}", error_stack[index]) == format!("{}", error_message),
            "\r\nexpected: {:?} \r\nactual: {:?}",
            error_stack[index],
            error_message
        );
    }
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

#[test]
fn eval_returned_returned_function() {
    let input = r#"
    let x = fn(a) {
      return fn(b) {
        return fn(c) {
          return c * b + a;
        };
      };
    };
    return x(1)(2)(3);
  "#;
    assert!("7" == format!("{}", compile_input(input)));
}

#[test]
fn eval_variable_not_found() {
    let input = r#"
    x;
  "#;
    compile_and_emit_error(input, vec!["x is not found"]);
}

#[test]
fn eval_variable_cannot_call_variable() {
    let input = r#"
    3();
  "#;
    compile_and_emit_error(input, vec!["cannot call 3"]);
}

// #[test]
fn eval_variable_conditoin_is_not_boolean() {
    let input = r#"
    if (1) {
      return 3;
    };
  "#;
    compile_and_emit_error(input, vec!["condition should be boolean. actually 1"]);
}
