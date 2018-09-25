use std::fmt;
use parser::expressions::*;

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

  pub fn emit_debug_info(&self) -> String {
    match self {
      Statement::Let(ident, expr) => {
        write_string!(format!("[ identifiy: {:?}, expression: {:?} ]", ident, expr))
      },
      Statement::Return(expr) => {
        write_string!(format!("[ expression: {:?} ]", expr))
      },
      Statement::Expression(expr) => {
        write_string!(format!("[ expression: {:?} ]", expr))
      },
    }
  }

  pub fn string(&self) -> String {
    match self {
      Statement::Let(ident, expr) => {
        ("let ".to_owned() + &ident.0 + " = " + &expr.string()).to_string()
      },
      Statement::Return(expr) => {
        ("return ".to_owned() + &expr.string()).to_string()
      },
      Statement::Expression(expr) => {
        expr.string()
      },
    }
  }
}