use std::fmt;

// use lexer::token::*;

// use parser::node::*;
// use parser::expression::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Identifier(pub String);

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
    body: BlockStatement
  },
  Call {
    function: Box<Expression>,
    arguments: Vec<Expression>
  }
}

// impl Statement {
//   fn statement_node(&mut self) -> Node {
//     match *self {
//       Statement::LetStatement(token, expressions, identifier) => {
//         Node{
//           node_type: NodeType::Expression,
//           value: String::new(),
//         }
//       },
//       Statement::ReturnStatement(token, expressions) => {
//         // temp
//         Node{
//           node_type: NodeType::Identifier,
//           value: String::new(),
//         }
//       },
//       Statement::ExpressionStatement(token, expressions) => {
//         // temp
//         Node{
//           node_type: NodeType::Identifier,
//           value: String::new(),
//         }
//       },
//       Statement::BlockStatement(token, statements) => {
//         // temp
//         Node{
//           node_type: NodeType::Identifier,
//           value: String::new(),
//         }
//       },
//     }
//   }

//   fn token_literal(&self) -> String {
//     match *self {
//       Statement::LetStatement(token, expressions, identifier) => {
//         write_string!(token.value)
//       },
//       Statement::ReturnStatement(token, expressions) => {
//         write_string!(token.value)
//       },
//       Statement::ExpressionStatement(token, expressions) => {
//         write_string!(token.value)
//       },
//       Statement::BlockStatement(token, statements) => {
//         write_string!(token.value)
//       }
//     }
    
//   }

//   fn emit_debug_info(&self) -> String {
//     match *self {
//       Statement::LetStatement(token, expressions, identifier) => {
//         write_string!(format!("[ {:?}, value: {:?}, {:?} ]", token, expressions, identifier))
//       },
//       Statement::ReturnStatement(token, expressions) => {
//         write_string!(format!("[ {:?}, return_value: {:?} ]", token, expressions))
//       },
//       Statement::ExpressionStatement(token, expressions) => {
//         write_string!(format!("[{:?}, {:?}]", token, expression))
//       },
//       Statement::BlockStatement(token, statements) => {
//         write_string!(format!("[{:?}, {:?}]", token, self.string()))
//       }
//     }
//   }

//   fn string(&self) -> String {
//     match *self {
//       Statement::LetStatement(token, expressions, identifier) => {
//         ("let ".to_owned() + &self.name.value + " = " + &self.value.string()).to_string()
//       },
//       Statement::ReturnStatement(token, expressions) => {
//         ("return ".to_owned() + &self.return_value.string()).to_string()
//       },
//       Statement::ExpressionStatement(token, expressions) => {
//         expression.string()
//       },
//       Statement::BlockStatement(token, statements) => {
//         let mut string_vec = Vec::new();

//         for statement in &statements {
//           string_vec.push(statement.string());
//         }

//         ("{".to_owned() + &string_vec.join("") + "}")
//       }
//     }
//   }
// }