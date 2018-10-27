use llvm_sys::*;

use parser::expressions::*;
use parser::parser::*;
use parser::statements::*;

use evaluate_ir::environment::*;
use evaluate_ir::object::*;

use ir::block::*;
use ir::const_value::*;
use ir::creator::*;
use ir::function::*;
use ir::llvm_type::*;
use ir::operate::*;
use ir::validate::*;

pub struct Eval {
    pub stack_arg: Vec<Vec<Expression>>,
    pub error_stack: Vec<Object>,
    pub lc: LLVMCreator,
}

#[allow(dead_code)]
impl Eval {
    pub fn new() -> Self {
        Eval {
            stack_arg: Vec::new(),
            error_stack: Vec::new(),
            lc: LLVMCreator::new("main_module"),
        }
    }

    pub fn setup_llvm(&mut self) -> *mut LLVMValue {
        self.lc.setup_builtin();
        self.setup_main()
    }

    pub fn setup_main(&mut self) -> *mut LLVMValue {
        let fn_type = function_type(int32_type(), &mut []);
        let main_function = add_function(self.lc.module, fn_type, "main");
        let block = append_basic_block_in_context(self.lc.context, main_function, "entry");
        build_position_at_end(self.lc.builder, block);
        main_function
    }

    pub fn dump_llvm(&mut self) {
        validate_module(self.lc.module);
        self.lc.dump();
    }

    pub fn entry_eval_program(&mut self, program: Program, env: &mut Environment) -> Object {
        let main = self.setup_main();
        for statement in program.into_iter() {
            if let Some(obj) = self.eval_statement(statement, env) {
                match obj {
                  Object::Integer(_, llvm_value) => {
                    build_ret(self.lc.builder, llvm_value);
                  },
                  _ => {
                    build_ret(self.lc.builder, const_int(int32_type(), 0));
                  }
                };
                return obj;
            }
        }
        build_ret(self.lc.builder, const_int(int32_type(), 0));
        Object::Null
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
            }
            Statement::Return(expr) => {
                let obj = self.eval_return_statement(expr, env);
                self.accumultae_error(obj)
            }
            Statement::Expression(expr) => match expr {
                Expression::If {
                    condition,
                    consequence,
                    alternative,
                    location,
                } => {
                    let obj = self.eval_if(condition, consequence, alternative, env, location);
                    if let Some(obj) = obj {
                        self.accumultae_error(obj)
                    } else {
                        None
                    }
                }
                _ => {
                    let obj = self.eval_expression(expr, env);
                    let _ = self.accumultae_error(obj);
                    None
                }
            },
            Statement::While(expr, block) => None,
        }
    }

    pub fn accumultae_error(&mut self, obj: Object) -> Option<Object> {
        match obj {
            Object::Error(_) => {
                self.error_stack.push(obj);
                None
            }
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
            Expression::IntegerLiteral(int, _location) => Object::Integer(int, const_int(int32_type(), int)),
            Expression::StringLiteral(string, _location) => Object::String(string),
            Expression::Boolean(boolean, _location) => Object::Boolean(boolean),
            Expression::Prefix(prefix, expr, location) => {
                self.eval_prefix(prefix, expr, env, location)
            }
            Expression::Infix(infix, left, right, location) => {
                self.eval_infix(infix, left, right, env, location)
            }
            Expression::Identifier(ident, location) => self.eval_identifier(ident, env, location),
            // Expression::Function {
            //     parameters,
            //     body,
            //     location: _,
            // } => Object::Function(Function {
            //     parameters: parameters,
            //     body: body,
            //     env: env.clone(),
            // }),
            // Expression::Call(Call {
            //     function,
            //     arguments,
            //     location,
            // }) => self.eval_call(function, arguments, env, location),
            _ => Object::Null,
        }
    }

    pub fn eval_identifier(
        &self,
        ident: Identifier,
        env: &mut Environment,
        location: Location,
    ) -> Object {
        env.get(&ident.0, location)
    }

    pub fn eval_prefix(
        &mut self,
        prefix: Prefix,
        expr: Box<Expression>,
        env: &mut Environment,
        location: Location,
    ) -> Object {
        let expr_value = self.eval_expression(*expr, env);
        match expr_value {
            Object::Integer(expr, _) => self.calculate_prefix_integer(prefix, expr),
            Object::Boolean(expr) => self.calculate_prefix_boolean(prefix, expr, location),
            _ => Object::Error(format!(
                "expr value should be integer, but actually {}. row: {}",
                expr_value, location.row,
            )),
        }
    }

    pub fn eval_infix(
        &mut self,
        infix: Infix,
        left: Box<Expression>,
        right: Box<Expression>,
        env: &mut Environment,
        location: Location,
    ) -> Object {
        let left_value = self.eval_expression(*left, env);
        let right_value = self.eval_expression(*right, env);

        match left_value {
            Object::Integer(left, _) => match right_value {
                Object::Integer(right, _) => {
                    self.calculate_infix_integer(infix, left, right, location)
                }
                _ => Object::Error(format!(
                    "right value should be integer, but actually {}. row: {}",
                    right_value, location.row,
                )),
            },
            Object::String(left) => match right_value {
                Object::String(right) => Object::String(left + &right),
                _ => Object::Error(format!(
                    "right value should be string, but actually {}. row: {}",
                    right_value, location.row,
                )),
            },
            Object::Boolean(left) => match right_value {
                Object::Boolean(right) => Object::Boolean(left == right),
                _ => Object::Error(format!(
                    "right value should be boolean, but actually {}. row: {}",
                    right_value, location.row,
                )),
            },
            _ => {
                let right_type_str = match right_value {
                    Object::Integer(_right, _) => "integer",
                    Object::String(_right) => "string",
                    Object::Boolean(_right) => "boolean",
                    _ => {
                        return Object::Error(format!(
                            "{} {} {} cannot be culculated. row: {}",
                            left_value, infix, right_value, location.row,
                        ));
                    }
                };
                Object::Error(format!(
                    "left value should be {}, but actually {}. row: {}",
                    right_type_str, left_value, location.row
                ))
            }
        }
    }

    pub fn eval_if(
        &mut self,
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
        env: &mut Environment,
        location: Location,
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
                return_obj = Object::Error(format!(
                    "condition should be boolean. actually {}. row: {}",
                    condition_obj, location.row,
                ));
            }
        };

        match return_obj {
            Object::Null => None,
            _ => Some(return_obj),
        }
    }

    pub fn calculate_prefix_boolean(
        &self,
        prefix: Prefix,
        value: bool,
        location: Location,
    ) -> Object {
        match prefix {
            Prefix::Bang => Object::Boolean(!value),
            _ => Object::Error(format!(
                "{} cannot be use for prefix. row: {}",
                prefix, location.row
            )),
        }
    }

    pub fn calculate_prefix_integer(&self, prefix: Prefix, value: u64) -> Object {
        match prefix {
            // Prefix::Minus => Object::Integer(-1 * value, const_int(int32_type(), -1 * value)),
            Prefix::Minus => Object::Integer(value, const_int(int32_type(), value)),
            Prefix::Plus => Object::Integer(value, const_int(int32_type(), value)),
            Prefix::Bang => {
                if value < 0 {
                    Object::Boolean(true)
                } else {
                    Object::Boolean(false)
                }
            }
        }
    }

    pub fn calculate_infix_integer(
        &self,
        infix: Infix,
        left: u64,
        right: u64,
        location: Location,
    ) -> Object {
        match infix {
            Infix::Plus => Object::Integer(left + right, const_int(int32_type(), left + right)),
            Infix::Minus => Object::Integer(left - right, const_int(int32_type(), left - right)),
            Infix::Multiply => Object::Integer(left * right, const_int(int32_type(), left * right)),
            Infix::Divide => Object::Integer(left / right, const_int(int32_type(), left / right)),
            Infix::Lt => Object::Boolean(left < right),
            Infix::Lte => Object::Boolean(left <= right),
            Infix::Gt => Object::Boolean(left > right),
            Infix::Gte => Object::Boolean(left >= right),
            Infix::Eq => Object::Boolean(left == right),
            _ => Object::Error(format!(
                "{} cannot be calculate for integer. row: {}",
                infix, location.row
            )),
        }
    }

    pub fn has_error(&self) -> bool {
        self.error_stack.len() > 0
    }

    pub fn emit_error(&mut self) -> String {
        let mut error_message = String::new();
        for (index, err_obj) in self.error_stack.iter().enumerate() {
            if index == 0 {
                error_message = format!("{}", err_obj);
            } else {
                error_message = format!("{}\n{}", error_message, err_obj);
            }
        }
        error_message.to_string()
    }
}

// pub fn eval_call(
//     &mut self,
//     outer_function: Box<Expression>,
//     outer_arguments: Vec<Expression>,
//     outer_env: &mut Environment,
//     location: Location,
// ) -> Object {
//     match *outer_function {
//         Expression::Identifier(Identifier(ref string), ref _location) => {
//             let mut call_function = outer_env.get(string, location);
//             self.exec_func(call_function, outer_arguments, outer_env)
//         }
//         Expression::Call(call) => {
//             self.stack_arg.push(outer_arguments);
//             self.call_func(call.clone(), call.arguments, outer_env, location)
//         }
//         _ => Object::Error(format!(
//             "cannot call {}. row: {}",
//             outer_function.string(),
//             location.row
//         )),
//     }
// }

// pub fn call_func(
//     &mut self,
//     call: Call,
//     outer_arguments: Vec<Expression>,
//     outer_env: &mut Environment,
//     location: Location,
// ) -> Object {
//     match *call.function {
//         Expression::Identifier(Identifier(ref string), ref _location) => {
//             let mut call_function = outer_env.get(string, location);
//             self.stack_arg.push(call.arguments);

//             while let Some(arg) = self.stack_arg.pop() {
//                 call_function = self.exec_func(call_function, arg, outer_env);

//                 match call_function {
//                     Object::Function(_) => {
//                         continue;
//                     }
//                     _ => {
//                         return call_function;
//                     }
//                 }
//             }
//             call_function
//         }
//         Expression::Call(inner_call) => {
//             self.stack_arg.push(outer_arguments);
//             self.call_func(
//                 inner_call.clone(),
//                 inner_call.arguments,
//                 outer_env,
//                 location,
//             )
//         }
//         _ => Object::Error(format!(
//             "cannot call {}. row: {}",
//             call.function.string(),
//             location.row
//         )),
//     }
// }

// pub fn exec_func(
//     &mut self,
//     maybe_func_obj: Object,
//     outer_arguments: Vec<Expression>,
//     outer_env: &mut Environment,
// ) -> Object {
//     match maybe_func_obj {
//         Object::Function(func) => {
//             let mut func_env = func.env.clone();
//             for (index, Identifier(string)) in func.parameters.into_iter().enumerate() {
//                 let actual_param =
//                     self.eval_expression(outer_arguments[index].clone(), outer_env);
//                 func_env.set(string, actual_param);
//             }
//             self.eval_program(func.body, &mut func_env)
//         }
//         Object::BuildIn(build_in) => match build_in {
//             BuildIn::Print => {
//                 let print_struct = BuildInPrint::new();
//                 print_struct.print(&outer_arguments[0].clone().string());
//                 Object::Null
//             }
//         },
//         _ => maybe_func_obj,
//     }
// }
