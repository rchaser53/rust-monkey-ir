use lexer::token::*;

use parser::node::*;
use parser::expression::*;

pub enum Statement {
  LetStatement(Token, Box<Expressions>, Identifier),
  ReturnStatement(Token, Expressions),
  ExpressionStatement(Token, Expressions),
  BlockStatement(Token, Vec<Statement>),
}

pub enum Expressions {}

impl Statement {
  fn statement_node(&mut self) -> Node {
    match *self {
      Statement::LetStatement(token, expressions, identifier) => {
        Node{
          node_type: NodeType::Expression,
          value: String::new(),
        }
      },
      Statement::ReturnStatement(token, expressions) => {
        // temp
        Node{
          node_type: NodeType::Identifier,
          value: String::new(),
        }
      },
      Statement::ExpressionStatement(token, expressions) => {
        // temp
        Node{
          node_type: NodeType::Identifier,
          value: String::new(),
        }
      },
      Statement::BlockStatement(token, statements) => {
        // temp
        Node{
          node_type: NodeType::Identifier,
          value: String::new(),
        }
      },
    }
  }

  fn token_literal(&self) -> String {
    match *self {
      Statement::LetStatement(token, expressions, identifier) => {
        write_string!(token.value)
      },
      Statement::ReturnStatement(token, expressions) => {
        write_string!(token.value)
      },
      Statement::ExpressionStatement(token, expressions) => {
        write_string!(token.value)
      },
      Statement::BlockStatement(token, statements) => {
        write_string!(token.value)
      }
    }
    
  }

  fn emit_debug_info(&self) -> String {
    match *self {
      Statement::LetStatement(token, expressions, identifier) => {
        write_string!(format!("[ {:?}, value: {:?}, {:?} ]", token, expressions, identifier))
      },
      Statement::ReturnStatement(token, expressions) => {
        write_string!(format!("[ {:?}, return_value: {:?} ]", token, expressions))
      },
      Statement::ExpressionStatement(token, expressions) => {
        write_string!(format!("[{:?}, {:?}]", token, expression))
      },
      Statement::BlockStatement(token, statements) => {
        write_string!(format!("[{:?}, {:?}]", token, self.string()))
      }
    }
  }

  fn string(&self) -> String {
    match *self {
      Statement::LetStatement(token, expressions, identifier) => {
        ("let ".to_owned() + &self.name.value + " = " + &self.value.string()).to_string()
      },
      Statement::ReturnStatement(token, expressions) => {
        ("return ".to_owned() + &self.return_value.string()).to_string()
      },
      Statement::ExpressionStatement(token, expressions) => {
        expression.string()
      },
      Statement::BlockStatement(token, statements) => {
        let mut string_vec = Vec::new();

        for statement in &statements {
          string_vec.push(statement.string());
        }

        ("{".to_owned() + &string_vec.join("") + "}")
      }
    }
  }
}