use lexer::token::*;

use parser::node::*;
use parser::expression::*;

pub trait Statement {
  fn statement_node(&self) -> Node;
  fn token_literal(&self) -> String;
  fn emit_debug_info(&self) -> String;
  fn string(&self) -> String;
}

pub struct LetStatement {
  pub token: Token,
  pub value: Box<Expressions>,
  pub name: Identifier,
}
impl Statement for LetStatement {
  fn statement_node(&self) -> Node {
    Node{}
  }

  fn token_literal(&self) -> String {
    write_string!(self.token.value)
  }

  fn emit_debug_info(&self) -> String {
    write_string!(format!("{:?} {:?} {:?}", self.token, self.value, self.name))
  }

  fn string(&self) -> String {
    ("let".to_owned() + &self.name.value + " = " + &self.value.string()).to_string()
  }
}
impl Default for LetStatement {
    fn default() -> LetStatement {
      LetStatement{
        token: Token{ kind: TokenType::TokenLet, value: write_string!("let") },
        value: Box::new(Expression{ node: Node{} }),
        name: Identifier {
          token: Token{ kind: TokenType::TokenIdentifier, value: write_string!("empty_variable") },
          value: write_string!("empty_variable")
        }
      }
    }
}

pub struct ReturnStatement {
  pub token: Token,
  pub return_value: Box<Expressions>,
}
impl Statement for ReturnStatement {
  fn statement_node(&self) -> Node {
    Node{}
  }

  fn token_literal(&self) -> String {
    write_string!(self.token.value)
  }

  fn emit_debug_info(&self) -> String {
    write_string!(format!("{:?} {:?}", self.token, self.return_value))
  }

  fn string(&self) -> String {
    ("return".to_owned() + &self.return_value.string()).to_string()
  }
}

pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Box<Expressions>,
}
impl Statement for ExpressionStatement {
  fn statement_node(&self) -> Node {
    Node{}
  }

  fn token_literal(&self) -> String {
    write_string!(self.token.value)
  }

  fn emit_debug_info(&self) -> String {
    write_string!(format!("{:?} {:?}", self.token, self.expression))
  }

  fn string(&self) -> String {
    self.expression.string()
  }
}