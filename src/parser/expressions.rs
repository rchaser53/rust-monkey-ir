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
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

impl Expression {
    pub fn string(&self) -> String {
        match self {
            Expression::Identifier(ident) => {
                ident.0.to_string()
            }
            Expression::IntegerLiteral(int) => int.to_string(),
            Expression::StringLiteral(literal) => literal.to_string(),
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
                let mut ret_string = String::new();
                for (index, parameter) in parameters.iter().enumerate() {
                    if index != 0 {
                        ret_string = ret_string + ", "
                    }
                    ret_string = ret_string + &parameter.0;
                }
                for statement in body {
                    ret_string = ret_string + " " + &statement.string();
                }

                format!("fn({}) {{}}", ret_string)
            }
            Expression::Call {
                function,
                arguments,
            } => {
                let mut ret_string = String::new();
                for (index, parameter) in arguments.iter().enumerate() {
                    if index != 0 {
                        ret_string = ret_string + ", "
                    }
                    ret_string = ret_string + &parameter.string();
                }

                format!("{}({})", function.string(), ret_string)
            }
        }
    }
}
