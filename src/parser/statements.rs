use parser::expressions::*;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Prefix {
    Plus,
    Minus,
    Bang,
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Prefix::Plus => write!(f, "+"),
            Prefix::Minus => write!(f, "-"),
            Prefix::Bang => write!(f, "!"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Rem,
    Eq,
    NotEq,
    Gte,
    Gt,
    Lte,
    Lt,
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Divide => write!(f, "/"),
            Infix::Multiply => write!(f, "*"),
            Infix::Rem => write!(f, "%"),
            Infix::Eq => write!(f, "=="),
            Infix::NotEq => write!(f, "!="),
            Infix::Gte => write!(f, ">="),
            Infix::Gt => write!(f, ">"),
            Infix::Lte => write!(f, "<="),
            Infix::Lt => write!(f, "<"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Let(Identifier, LLVMExpressionType, Expression),
    Return(Expression),
    Expression(Expression),
    While(Expression, BlockStatement),
    Assignment(Identifier, Expression),
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
        }
    }
}
