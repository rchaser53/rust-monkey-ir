use parser::statements::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Identifier(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(i64),
    StringLiteral(String),
    Boolean(bool),
    Prefix(Prefix, Box<Expression>),
    Infix(Infix, Box<Expression>, Box<Expression>),
    If {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    Function {
        parameters: Vec<Identifier>,
        body: BlockStatement,
    },
    Call(Call),
    Error(String),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Call {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl Expression {
    pub fn string(&self) -> String {
        match self {
            Expression::Identifier(ident) => ident.0.to_string(),
            Expression::IntegerLiteral(int) => int.to_string(),
            Expression::StringLiteral(literal) => format!(r#""{}""#, literal.to_string()),
            Expression::Boolean(boolean) => boolean.to_string(),
            Expression::Prefix(prefix, expr) => format!("({}{})", prefix, expr.string()),
            Expression::Infix(infix, left, right) => {
                format!("({} {} {})", left.string(), infix, right.string())
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                let consequence_string = consequence
                    .iter()
                    .map(|s| s.string())
                    .collect::<Vec<_>>()
                    .join("\n");

                if let Some(alt) = alternative {
                    let alternative_string = alt.iter()
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
            Expression::Function { parameters, body } => {
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
            Expression::Error(message) => format!(r#""{}""#, message.to_string()),
        }
    }
}
