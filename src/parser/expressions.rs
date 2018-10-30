use parser::statements::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Identifier(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Identifier(Identifier, Location),
    IntegerLiteral(u64, Location),
    StringLiteral(String, Location),
    Boolean(bool, Location),
    Prefix(Prefix, Box<Expression>, Location),
    Infix(Infix, Box<Expression>, Box<Expression>, Location),
    If {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
        location: Location,
    },
    Function {
        parameters: Vec<Identifier>,
        parameter_types: Vec<LLVMExpressionType>,
        body: BlockStatement,
        return_type: LLVMExpressionType,
        location: Location,
    },
    While {
        condition: Box<Expression>,
        body: BlockStatement,
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
    Int,
    String,
    Boolean,
    Null,
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
            Expression::Prefix(prefix, expr, _location) => format!("({}{})", prefix, expr.string()),
            Expression::Infix(infix, left, right, _location) => {
                format!("({} {} {})", left.string(), infix, right.string())
            }
            Expression::If {
                condition,
                consequence,
                alternative,
                location: _,
            } => {
                let consequence_string = consequence
                    .iter()
                    .map(|s| s.string())
                    .collect::<Vec<_>>()
                    .join("\n");

                if let Some(alt) = alternative {
                    let alternative_string = alt
                        .iter()
                        .map(|s| s.string())
                        .collect::<Vec<_>>()
                        .join("\n");

                    return format!(
                        "if{} {{ {} }} else {{ {} }}",
                        &condition.string(),
                        consequence_string,
                        alternative_string
                    );
                }
                format!("if{} {{ {} }}", &condition.string(), consequence_string)
            }
            Expression::Function {
                parameters,
                body,
                parameter_types: _,
                return_type: _,
                location: _,
            } => {
                let mut param_string = String::new();
                for (index, Identifier(ref string)) in parameters.iter().enumerate() {
                    if index == 0 {
                        param_string.push_str(&format!("{}", string));
                    } else {
                        param_string.push_str(&format!(", {}", string));
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

                format!("fn({}) {{ {} }}", param_string, ret_string)
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
            Expression::While {
                condition,
                body,
                location: _,
            } => {
                let mut ret_string = String::new();
                for (index, statement) in body.iter().enumerate() {
                    if index == 0 {
                        ret_string.push_str(&format!("{}", statement.string()));
                    } else {
                        ret_string.push_str(&format!(" {}", statement.string()));
                    }
                }
                format!("while({}) {{ {} }}", &condition.string(), ret_string)
            }
        }
    }
}
