use std::fmt;

use parser::infix::*;
use parser::prefix::*;
use parser::statements::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Identifier(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Identifier(Identifier, Location),
    IntegerLiteral(u64, Location),
    StringLiteral(String, Location),
    Boolean(bool, Location),
    Array(LLVMExpressionType, Vec<Expression>),
    ArrayElement(Identifier, Box<Expression>, Location),
    Prefix(Prefix, Box<Expression>, Location),
    Infix(Infix, Box<Expression>, Box<Expression>, Location),
    If {
        conditions: Vec<Expression>,
        bodies: Vec<BlockStatement>,
        location: Location,
    },
    Function {
        parameters: Vec<Identifier>,
        parameter_types: Vec<LLVMExpressionType>,
        body: BlockStatement,
        return_type: LLVMExpressionType,
        location: Location,
    },
    Call(Call),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Call {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub location: Location,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Location {
    pub row: usize,
}

impl Location {
    pub fn new(row: usize) -> Self {
        Location { row: row }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LLVMExpressionType {
    Integer,
    String(u32),
    Boolean,
    Null,
    Array(Box<LLVMExpressionType>, u32),
    Function,
    Call,
}

impl fmt::Display for LLVMExpressionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LLVMExpressionType::Integer => write!(f, "{}", "int"),
            LLVMExpressionType::String(_) => write!(f, "{}", "string"),
            LLVMExpressionType::Boolean => write!(f, "{}", "boolean"),
            LLVMExpressionType::Null => write!(f, "{}", "null"),
            LLVMExpressionType::Array(_, _) => write!(f, "{}", "array"),
            LLVMExpressionType::Function => write!(f, "{}", "function"),
            LLVMExpressionType::Call => write!(f, "{}", "call"),
        }
    }
}

impl Expression {
    pub fn string(&self) -> String {
        match self {
            Expression::Identifier(ident, _location) => ident.0.to_string(),
            Expression::IntegerLiteral(int, _location) => int.to_string(),
            Expression::StringLiteral(literal, _location) => {
                format!(r#""{}""#, literal.to_string())
            }
            Expression::Boolean(boolean, _location) => boolean.to_string(),
            Expression::Array(_, elements) => {
                let elements_string = elements
                    .iter()
                    .fold(Vec::new(), |mut stack, element| {
                        stack.push(element.string());
                        stack
                    })
                    .join(", ");

                format!("[{}]", elements_string)
            }
            Expression::ArrayElement(ident, index_expression, _) => {
                format!("{}[{}]", ident.0.to_string(), index_expression.string())
            }
            Expression::Prefix(prefix, expr, _location) => format!("({}{})", prefix, expr.string()),
            Expression::Infix(infix, left, right, _location) => {
                format!("({} {} {})", left.string(), infix, right.string())
            }
            Expression::If {
                conditions,
                bodies,
                location: _,
            } => {
                let mut condition_strings =
                    conditions.iter().map(|s| s.string()).collect::<Vec<_>>();

                let body_strings = bodies.iter().fold(Vec::new(), |mut stack, body| {
                    let body_string = body
                        .iter()
                        .map(|s| s.string())
                        .collect::<Vec<_>>()
                        .join("\n");
                    stack.push(body_string);
                    stack
                });

                let mut ret_string = String::new();
                for (index, condition_string) in condition_strings.iter().enumerate() {
                    if index == 0 {
                        ret_string.push_str(&format!(
                            "if({}) {{ {} }} ",
                            condition_string, body_strings[index]
                        ));
                    } else {
                        ret_string.push_str(&format!(
                            "elseif({}) {{ {} }}",
                            condition_string, body_strings[index]
                        ));
                    }
                }

                format!("{}", ret_string)
            }
            Expression::Function {
                parameters,
                body,
                parameter_types,
                return_type,
                location: _,
            } => {
                let mut param_string = String::new();
                for (index, Identifier(ref string)) in parameters.iter().enumerate() {
                    if index == 0 {
                        param_string.push_str(&format!("{}: {}", string, parameter_types[index]));
                    } else {
                        param_string.push_str(&format!(", {}: {}", string, parameter_types[index]));
                    }
                }
                let mut ret_string = String::new();
                for (index, statement) in body.iter().enumerate() {
                    if index == 0 {
                        ret_string.push_str(&format!("{}", statement.string()));
                    } else {
                        ret_string.push_str(&format!(" {}", statement.string()));
                    }
                }

                format!("fn({}): {} {{ {} }}", param_string, return_type, ret_string)
            }
            Expression::Call(call) => {
                let mut ret_string = String::new();
                for (index, parameter) in call.arguments.iter().enumerate() {
                    if index == 0 {
                        ret_string.push_str(&format!("{}", &parameter.string()));
                    } else {
                        ret_string.push_str(&format!(", {}", &parameter.string()));
                    }
                }

                format!("{}({})", call.function.string(), ret_string)
            }
        }
    }
}
