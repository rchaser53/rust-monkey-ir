use parser::expressions::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Let(Identifier, LLVMExpressionType, Expression),
    Return(Expression),
    Expression(Expression),
    While(Expression, BlockStatement),
    Assignment(Identifier, Expression),
    AssignmentAggregate(Identifier, Expression, u64),
}

pub type BlockStatement = Vec<Statement>;
pub type Program = BlockStatement;

impl Statement {
    #[allow(dead_code)]
    pub fn emit_debug_info(&self) -> String {
        match self {
            Statement::Let(Identifier(ref string), _, expr) => write_string!(format!(
                "[ identifiy: {}, expression: {} ]",
                string,
                expr.string()
            )),
            Statement::Return(expr) => write_string!(format!("[ expression: {} ]", expr.string())),
            Statement::Expression(expr) => {
                write_string!(format!("[ expression: {} ]", expr.string()))
            }
            Statement::While(expr, body) => {
                let mut ret_string = String::new();
                for (index, statement) in body.iter().enumerate() {
                    if index == 0 {
                        ret_string.push_str(&format!("{}", statement.string()));
                    } else {
                        ret_string.push_str(&format!(" {}", statement.string()));
                    }
                }
                write_string!(format!(
                    "[ expression: {}, block: {} ]",
                    expr.string(),
                    ret_string
                ))
            }
            Statement::Assignment(Identifier(ref string), expr) => write_string!(format!(
                "[ identifiy: {}, expression: {} ]",
                string,
                expr.string()
            )),
            Statement::AssignmentAggregate(Identifier(ref string), expr, idnex) => write_string!(format!(
                "[ identifiy: {}, expression: {}, idnex: {} ]",
                string,
                expr.string(),
                idnex
            )),
        }
    }

    pub fn string(&self) -> String {
        match self {
            Statement::Let(Identifier(ref string), _, expr) => {
                format!("let {} = {}", string, &expr.string())
            }
            Statement::Return(expr) => ("return ".to_owned() + &expr.string()).to_string(),
            Statement::Expression(expr) => expr.string(),
            Statement::While(expr, body) => {
                let mut ret_string = String::new();
                for (index, statement) in body.iter().enumerate() {
                    if index == 0 {
                        ret_string.push_str(&format!("{}", statement.string()));
                    } else {
                        ret_string.push_str(&format!(" {}", statement.string()));
                    }
                }
                format!("while ({}) {{ {} }}", expr.string(), ret_string)
            }
            Statement::Assignment(Identifier(ref string), expr) => {
                format!("{} = {}", string, &expr.string())
            }
            Statement::AssignmentAggregate(Identifier(ref string), expr, idnex) => {
                format!("{}[{}] = {}", string, idnex, &expr.string())
            }
        }
    }
}
