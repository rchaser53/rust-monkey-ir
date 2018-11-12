use llvm_sys::*;

use parser::expressions::*;
use parser::statements::*;

use evaluate_ir::environment::*;
use evaluate_ir::object::*;
use evaluate_ir::stack::*;

use ir::arithmetic::*;
use ir::block::*;
use ir::condition::*;
use ir::const_value::*;
use ir::converter::*;
use ir::creator::*;
use ir::function::*;
use ir::llvm_type::*;
use ir::operate::*;
use ir::string::*;
use ir::validate::*;

pub struct Eval {
    pub stack_arg: Vec<Vec<Expression>>,
    pub error_stack: Vec<Object>,
    pub lc: LLVMCreator,
    pub main_block: *mut LLVMBasicBlock,
    pub function_stack: FunctionStack,
}

#[allow(dead_code)]
impl Eval {
    pub fn new() -> Self {
        let mut lc = LLVMCreator::new("main_module");
        let (main_block, main_function) = Eval::setup_main(&mut lc);

        Eval {
            stack_arg: Vec::new(),
            error_stack: Vec::new(),
            lc: lc,
            main_block: main_block,
            function_stack: FunctionStack::new(main_function),
        }
    }

    pub fn setup_main(lc: &mut LLVMCreator) -> (*mut LLVMBasicBlock, *mut LLVMValue) {
        let fn_type = function_type(int32_type(), &mut []);
        let main_function = add_function(lc.module, fn_type, "main");
        let block = append_basic_block_in_context(lc.context, main_function, "entry");
        build_position_at_end(lc.builder, block);
        (block, main_function)
    }

    pub fn dump_llvm(&mut self) {
        self.lc.dump();
        validate_module(self.lc.module);
    }

    pub fn emit_llvm(&mut self, file_name: &str) {
        self.lc.emit_file(file_name);
    }

    pub fn entry_eval_program(&mut self, program: Program, env: &mut Environment) -> Object {
        for statement in program.into_iter() {
            if let Some(mut obj) = self.eval_statement(statement, env) {
                self.build_llvm_return(&mut obj);
                return obj;
            }
        }
        build_ret(self.lc.builder, llvm_integer!(0));
        Object::Null
    }

    pub fn build_llvm_return(&mut self, object: &mut Object) {
        let llvm_value = unwrap_object(object);
        build_ret(self.lc.builder, llvm_value);
    }

    pub fn eval_program(&mut self, program: Program, env: &mut Environment) -> Object {
        for statement in program.into_iter() {
            if let Some(mut obj) = self.eval_statement(statement, env) {
                self.build_llvm_return(&mut obj);
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
            Statement::Let(ident, expr_type, expr) => {
                let obj = self.eval_let_staement(ident, expr_type, expr, env);
                let _ = self.accumultae_error(obj);
                None
            }
            Statement::Return(expr) => {
                let obj = self.eval_return_statement(expr, env);
                self.accumultae_error(obj)
            }
            Statement::Expression(expr) => match expr {
                Expression::If {
                    conditions,
                    bodies,
                    location,
                } => {
                    let obj = self.eval_if(conditions, bodies, env, location);
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
            Statement::While(expr, block) => {
                self.eval_while_statement(expr, block, env);
                None
            }
            Statement::Assignment(ident, expr) => {
                let obj = self.eval_assign_statement(ident, expr, env);
                let _ = self.accumultae_error(obj);
                None
            }
        }
    }

    pub fn eval_while_statement(
        &mut self,
        condition: Expression,
        block: BlockStatement,
        env: &mut Environment,
    ) -> Object {
        let mut object = self.eval_expression(condition.clone(), &mut env.clone());
        let mut llvm_value = unwrap_object(&mut object);

        let current_function = self.function_stack.last();
        let loop_block = append_basic_block_in_context(self.lc.context, current_function, "");
        let end_block = append_basic_block_in_context(self.lc.context, current_function, "");

        build_cond_br(self.lc.builder, llvm_value, loop_block, end_block);
        build_position_at_end(self.lc.builder, loop_block);
        let return_obj = self.eval_program(block, env);

        object = self.eval_expression(condition, &mut env.clone());
        llvm_value = unwrap_object(&mut object);
        build_cond_br(self.lc.builder, llvm_value, loop_block, end_block);
        build_position_at_end(self.lc.builder, end_block);

        return_obj
    }

    pub fn eval_let_staement(
        &mut self,
        ident: Identifier,
        expr_type: LLVMExpressionType,
        expr: Expression,
        env: &mut Environment,
    ) -> Object {
        let mut object = self.eval_expression(expr, env);
        let llvm_type = convert_llvm_type(expr_type);
        let llvm_value = unwrap_object(&mut object);

        let llvm_value_ref = build_alloca(self.lc.builder, llvm_type, &ident.0);
        build_store(self.lc.builder, llvm_value, llvm_value_ref);

        let rewraped_object = rewrap_llvm_value_ref(object, llvm_value_ref);

        env.set(ident.0, rewraped_object)
    }

    pub fn eval_return_statement(&mut self, expr: Expression, env: &mut Environment) -> Object {
        self.eval_expression(expr, env)
    }

    pub fn eval_expression(&mut self, expr: Expression, env: &mut Environment) -> Object {
        match expr {
            Expression::IntegerLiteral(int, _location) => Object::Integer(llvm_integer!(int)),
            Expression::StringLiteral(string, _location) => Object::String(
                LLVMExpressionType::Array(Box::new(LLVMExpressionType::Int), string.len() as u32),
                codegen_string(&mut self.lc, &string, ""),
            ),
            Expression::Boolean(boolean, _location) => Object::Boolean(llvm_bool!(boolean)),
            Expression::Array(expression_type, elements) => {
                self.eval_array(expression_type, elements, env)
            }
            Expression::ArrayChild(ident, index_expression, location) => {
                self.eval_array_child(ident, *index_expression, env, location)
            }
            Expression::Prefix(prefix, expr, location) => {
                self.eval_prefix(prefix, expr, env, location)
            }
            Expression::Infix(infix, left, right, location) => {
                self.eval_infix(infix, left, right, env, location)
            }
            Expression::Identifier(ident, location) => self.eval_identifier(ident, env, location),
            Expression::Function {
                parameters,
                parameter_types,
                body,
                return_type,
                location,
            } => self.eval_function(
                parameters,
                parameter_types,
                body,
                return_type,
                env,
                location,
            ),
            Expression::Call(Call {
                function,
                arguments,
                location,
            }) => self.eval_call(function, arguments, env, location),
            _ => Object::Null,
        }
    }

    pub fn eval_array(
        &mut self,
        expression_type: LLVMExpressionType,
        elements: Vec<Expression>,
        env: &mut Environment,
    ) -> Object {
        let elements_len = elements.len() as u32;
        let object_vec = elements
            .into_iter()
            .map(
                |element| match self.eval_expression(element, &mut env.clone()) {
                    Object::Integer(reference) => reference,
                    Object::Boolean(reference) => reference,
                    _ => 0 as *mut LLVMValue,
                },
            ).collect();
        let llvm_type = convert_llvm_type(expression_type.clone());
        let llvm_array_value = const_array(llvm_type, object_vec);

        Object::Array(expression_type, llvm_array_value)
    }

    pub fn eval_array_child(
        &mut self,
        ident: Identifier,
        expr: Expression,
        env: &mut Environment,
        location: Location,
    ) -> Object {
        // need to get llvm_value_reference.
        // so access directly
        let mut obj = env.get(&ident.0, location);
        let child_expression_type = match obj.clone() {
            Object::Array(child_expression_type, _) => child_expression_type,
            _ => LLVMExpressionType::Null,
        };
        let array_llvm_value = unwrap_object(&mut obj);

        let mut index_object = self.eval_expression(expr, env);
        let index_llvm_value = unwrap_object(&mut index_object);

        let llvm_child_value = build_gep(
            self.lc.builder,
            array_llvm_value,
            vec![const_int(int32_type(), 0), index_llvm_value],
            "",
        );

        wrap_llvm_value(
            child_expression_type,
            build_load(self.lc.builder, llvm_child_value, ""),
        )
    }

    pub fn eval_assign_statement(
        &mut self,
        ident: Identifier,
        expr: Expression,
        env: &mut Environment,
    ) -> Object {
        let identify_object = env.get(&ident.0, Location::new(0)); // TODO
        let llvm_value_ref = match identify_object {
            Object::Integer(reference) => reference,
            Object::Boolean(reference) => reference,
            _ => 0 as *mut LLVMValue,
        };

        let mut object = self.eval_expression(expr, &mut env.clone());
        let llvm_value = unwrap_object(&mut object);
        build_store(self.lc.builder, llvm_value, llvm_value_ref);
        Object::Null
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

    pub fn eval_function(
        &mut self,
        parameters: Vec<Identifier>,
        parameter_types: Vec<LLVMExpressionType>,
        block: BlockStatement,
        return_type: LLVMExpressionType,
        env: &mut Environment,
        _location: Location,
    ) -> Object {
        let mut converted: Vec<*mut LLVMType> = parameter_types
            .clone()
            .into_iter()
            .map(|elem| convert_llvm_type(elem))
            .collect();

        let fn_type = function_type(convert_llvm_type(return_type.clone()), &mut converted);
        let (target_func, func_block) = create_function(&mut self.lc, fn_type);
        self.function_stack.push(target_func);

        let mut func_env = env.clone();
        for (index, Identifier(string)) in parameters.clone().into_iter().enumerate() {
            func_env.set(
                string,
                Object::Argument(parameter_types[index].clone(), target_func, index as u32),
            );
        }

        self.eval_program(block, &mut func_env);

        build_position_at_end(self.lc.builder, self.main_block);
        let _ = self.function_stack.pop();

        Object::Function(Function {
            return_type: return_type,
            llvm_value: target_func,
            llvm_block: func_block,
        })
    }

    pub fn eval_identifier(
        &self,
        ident: Identifier,
        env: &mut Environment,
        location: Location,
    ) -> Object {
        let obj = env.get(&ident.0, location);

        match obj {
            Object::Integer(llvm_val_ref) => {
                Object::Integer(build_load(self.lc.builder, llvm_val_ref, ""))
            }
            Object::Boolean(llvm_val_ref) => {
                Object::Boolean(build_load(self.lc.builder, llvm_val_ref, ""))
            }
            Object::Array(llvm_child_type, llvm_val_ref) => Object::Array(
                llvm_child_type,
                build_load(self.lc.builder, llvm_val_ref, ""),
            ),
            Object::Argument(expression_type, func, index) => {
                let llvm_value = get_param(func, index);
                wrap_llvm_value(expression_type, llvm_value)
            }
            _ => obj,
        }
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
            Object::Integer(value) => self.calculate_prefix_integer(prefix, value),
            Object::Boolean(value) => self.calculate_prefix_boolean(prefix, value, location),
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
        let left_object = self.eval_expression(*left, env);
        let right_object = self.eval_expression(*right, env);

        match left_object {
            Object::Integer(left) => self.resolve_left_integer(infix, left, right_object, location),
            Object::Boolean(left) => self.resolve_left_boolean(infix, left, right_object, location),
            Object::Argument(expression_type_left, func, index) => {
                let left = get_param(func, index);
                self.resolve_left_argument(
                    infix,
                    expression_type_left,
                    left,
                    right_object,
                    location,
                )
            }
            Object::String(_, left) => {
                self.resolve_left_string(infix, left, right_object, location)
            }
            _ => self.resolve_left_failed(infix, left_object, right_object, location),
        }
    }

    pub fn resolve_left_integer(
        &mut self,
        infix: Infix,
        left: *mut LLVMValue,
        right_object: Object,
        location: Location,
    ) -> Object {
        match right_object {
            Object::Integer(right) => self.calculate_infix_integer(infix, left, right, location),
            Object::Argument(_, func, index) => {
                let right = get_param(func, index);
                self.calculate_infix_integer(infix, left, right, location)
            }
            _ => Object::Error(format!(
                "right value should be integer, but actually {}. row: {}",
                right_object, location.row,
            )),
        }
    }

    pub fn resolve_left_boolean(
        &mut self,
        infix: Infix,
        left: *mut LLVMValue,
        right_object: Object,
        location: Location,
    ) -> Object {
        match right_object {
            Object::Boolean(right) => self.calculate_infix_boolean(infix, left, right, location),
            Object::Argument(_, func, index) => {
                let right = get_param(func, index);
                self.calculate_infix_boolean(infix, left, right, location)
            }
            _ => Object::Error(format!(
                "right value should be boolean, but actually {}. row: {}",
                right_object, location.row,
            )),
        }
    }

    pub fn resolve_left_argument(
        &mut self,
        infix: Infix,
        expression_type_left: LLVMExpressionType,
        left: *mut LLVMValue,
        right_object: Object,
        location: Location,
    ) -> Object {
        match right_object {
            Object::Integer(right) => self.calculate_infix_integer(infix, left, right, location),
            Object::Boolean(right) => self.calculate_infix_boolean(infix, left, right, location),
            Object::Argument(_, func, index) => {
                let right = get_param(func, index);
                match wrap_llvm_value(expression_type_left.clone(), right) {
                    Object::Integer(_) => {
                        self.calculate_infix_integer(infix, left, right, location)
                    }
                    Object::Boolean(_) => {
                        self.calculate_infix_boolean(infix, left, right, location)
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
        &mut self,
        _infix: Infix,
        left: *mut LLVMValue,
        right_object: Object,
        location: Location,
    ) -> Object {
        match right_object {
            Object::String(llvm_expression_type, _) => Object::String(llvm_expression_type, left), // TODO
            _ => Object::Error(format!(
                "right value should be string, but actually {}. row: {}",
                right_object, location.row,
            )),
        }
    }

    pub fn resolve_left_failed(
        &mut self,
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

    pub fn eval_if(
        &mut self,
        conditions: Vec<Expression>,
        bodies: Vec<BlockStatement>,
        env: &mut Environment,
        _location: Location,
    ) -> Option<Object> {
        let current_function = self.function_stack.last();
        let last_index = conditions.len();
        let mut blocks = Vec::new();
        let mut condition_blocks = Vec::new();
        let mut return_obj = Object::Null;

        let booleans: Vec<*mut LLVMValue> = conditions
            .into_iter()
            .map(|condition| {
                let mut object = self.eval_expression(condition, &mut env.clone());
                unwrap_object(&mut object)
            }).collect();

        for _ in 0..last_index {
            condition_blocks.push(append_basic_block_in_context(
                self.lc.context,
                current_function,
                "",
            ));
            blocks.push(append_basic_block_in_context(
                self.lc.context,
                current_function,
                "",
            ));
        }
        let end_block = append_basic_block_in_context(self.lc.context, current_function, "");

        for (index, condition_block) in condition_blocks.into_iter().enumerate() {
            let block = blocks[index];

            build_cond_br(self.lc.builder, booleans[index], block, condition_block);
            build_position_at_end(self.lc.builder, block);
            return_obj = self.eval_program(bodies[index].clone(), env);
            build_br(self.lc.builder, end_block);

            build_position_at_end(self.lc.builder, condition_block);
        }

        build_br(self.lc.builder, end_block);
        build_position_at_end(self.lc.builder, end_block);

        match return_obj {
            Object::Null => None,
            _ => Some(return_obj),
        }
    }

    pub fn calculate_prefix_boolean(
        &self,
        prefix: Prefix,
        value: *mut LLVMValue,
        location: Location,
    ) -> Object {
        match prefix {
            Prefix::Bang => Object::Boolean(value), // need to fix
            _ => Object::Error(format!(
                "{} cannot be use for prefix. row: {}",
                prefix, location.row
            )),
        }
    }

    pub fn calculate_prefix_integer(&self, prefix: Prefix, value: *mut LLVMValue) -> Object {
        match prefix {
            Prefix::Minus => Object::Integer(const_neg(value)),
            Prefix::Plus => Object::Integer(value),
            Prefix::Bang => Object::Boolean(build_int_ult(
                self.lc.builder,
                const_int(int32_type(), 0),
                value,
                "",
            )),
        }
    }

    pub fn calculate_infix_integer(
        &self,
        infix: Infix,
        left: *mut LLVMValue,
        right: *mut LLVMValue,
        _location: Location,
    ) -> Object {
        match infix {
            Infix::Plus => Object::Integer(add_variable(self.lc.builder, left, right, "")),
            Infix::Minus => Object::Integer(sub_variable(self.lc.builder, left, right, "")),
            Infix::Multiply => Object::Integer(multiple_variable(self.lc.builder, left, right, "")),
            Infix::Rem => Object::Integer(rem_variable(self.lc.builder, left, right, "")),
            Infix::Divide => Object::Integer(divide_variable(self.lc.builder, left, right, "")),
            Infix::Lt => Object::Boolean(build_int_ult(self.lc.builder, left, right, "")),
            Infix::Lte => Object::Boolean(build_int_ule(self.lc.builder, left, right, "")),
            Infix::Gt => Object::Boolean(build_int_ugt(self.lc.builder, left, right, "")),
            Infix::Gte => Object::Boolean(build_int_uge(self.lc.builder, left, right, "")),
            Infix::Eq => Object::Boolean(build_int_eq(self.lc.builder, left, right, "")),
            Infix::NotEq => Object::Boolean(build_int_ne(self.lc.builder, left, right, "")),
        }
    }

    pub fn calculate_infix_boolean(
        &self,
        infix: Infix,
        left: *mut LLVMValue,
        right: *mut LLVMValue,
        location: Location,
    ) -> Object {
        match infix {
            Infix::Eq => Object::Boolean(build_int_eq(self.lc.builder, left, right, "")),
            Infix::NotEq => Object::Boolean(build_int_ne(self.lc.builder, left, right, "")),
            _ => Object::Error(format!(
                "{} cannot be calculate for boolean. row: {}",
                infix, location.row
            )),
        }
    }

    pub fn eval_call(
        &mut self,
        outer_function: Box<Expression>,
        outer_arguments: Vec<Expression>,
        outer_env: &mut Environment,
        location: Location,
    ) -> Object {
        match *outer_function {
            Expression::Identifier(Identifier(ref string), ref _location) => {
                let mut call_function = outer_env.get(string, location);
                self.exec_func(call_function, outer_arguments, outer_env)
            }
            Expression::Call(call) => {
                self.stack_arg.push(outer_arguments);
                self.call_func(call.clone(), call.arguments, outer_env, location)
            }
            _ => Object::Error(format!(
                "cannot call {}. row: {}",
                outer_function.string(),
                location.row
            )),
        }
    }

    pub fn call_func(
        &mut self,
        call: Call,
        outer_arguments: Vec<Expression>,
        outer_env: &mut Environment,
        location: Location,
    ) -> Object {
        match *call.function {
            Expression::Identifier(Identifier(ref string), ref _location) => {
                let mut call_function = outer_env.get(string, location);
                self.stack_arg.push(call.arguments);

                while let Some(arg) = self.stack_arg.pop() {
                    call_function = self.exec_func(call_function, arg, outer_env);

                    match call_function {
                        Object::Function(_) => {
                            continue;
                        }
                        _ => {
                            return call_function;
                        }
                    }
                }
                call_function
            }
            Expression::Call(inner_call) => {
                self.stack_arg.push(outer_arguments);
                self.call_func(
                    inner_call.clone(),
                    inner_call.arguments,
                    outer_env,
                    location,
                )
            }
            _ => Object::Error(format!(
                "cannot call {}. row: {}",
                call.function.string(),
                location.row
            )),
        }
    }

    pub fn exec_func(
        &mut self,
        maybe_func_obj: Object,
        outer_arguments: Vec<Expression>,
        outer_env: &mut Environment,
    ) -> Object {
        match maybe_func_obj {
            Object::Function(func) => {
                let function_argments: Vec<*mut LLVMValue> = outer_arguments
                    .into_iter()
                    .map(|elem| {
                        let mut object = self.eval_expression(elem, &mut outer_env.clone());
                        unwrap_object(&mut object)
                    }).collect();
                let llvm_value =
                    call_function(self.lc.builder, func.llvm_value, function_argments, "");
                wrap_llvm_value(func.return_type, llvm_value)
            }
            Object::BuildIn(build_in) => match build_in {
                BuildIn::Printf => {
                    let printf = self.lc.built_ins["printf"];
                    let function_argments: Vec<*mut LLVMValue> = outer_arguments
                        .into_iter()
                        .map(|elem| {
                            let mut object = self.eval_expression(elem, &mut outer_env.clone());
                            unwrap_object(&mut object)
                        }).collect();

                    call_function(self.lc.builder, printf, function_argments, "");
                    Object::Null
                }
            },
            _ => maybe_func_obj,
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
