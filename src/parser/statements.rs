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
    Let(Identifier, Expression),
    Return(Expression),
    Expression(Expression),
}

pub type BlockStatement = Vec<Statement>;
pub type Program = BlockStatement;

impl Statement {
    #[warn(dead_code)]
    pub fn emit_debug_info(&self) -> String {
        match self {
            Statement::Let(Identifier(ref string), expr) => write_string!(format!(
                "[ identifiy: {}, expression: {} ]",
                string,
                expr.string()
            )),
            Statement::Return(expr) => write_string!(format!("[ expression: {} ]", expr.string())),
            Statement::Expression(expr) => {
                write_string!(format!("[ expression: {} ]", expr.string()))
            }
        }
    }

    pub fn string(&self) -> String {
        match self {
            Statement::Let(Identifier(ref string), expr) => {
                format!("let {} = {}", string, &expr.string())
                // ("let ".to_owned() + &ident.0 + " = " + &expr.string()).to_string()
            }
            Statement::Return(expr) => ("return ".to_owned() + &expr.string()).to_string(),
            Statement::Expression(expr) => expr.string(),
        }
    }
}
