use std::fmt;

use lexer::token::*;
use parser::node::*;

pub trait Expressions {
  // fn expression_node(&mut self) -> Node;
  fn string(&self) -> String;
}
impl fmt::Debug for Expressions {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.string())
  }
}

#[derive(Debug, Clone)]
pub struct Expression {
  pub node: Node
}
impl Expressions for Expression {
  fn string(&self) -> String {
    self.node.string()
  }
}
impl Expression {
  pub fn expression_node(&mut self) -> Node {
    self.node.clone()
  }
}

#[derive(Debug, Clone)]
pub struct Identifier {
  pub token: Token,
  pub value: String,
}
impl Expressions for Identifier {
  fn string(&self) -> String {
    self.token.value.to_string()
  }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
  pub token: Token,
  pub value: i64,
}
impl Expressions for IntegerLiteral {
  fn string(&self) -> String {
    self.token.value.to_string()
  }
}

// Box avoids to add derive Clone
pub struct PrefixExpression {
  pub token: Token,
  pub operator: String,
  pub right: Box<Expressions>,
}
impl Expressions for PrefixExpression {
  fn string(&self) -> String {
    ("(".to_owned() + &self.operator.to_string() + &self.right.string() + ")")
  }
}
impl PrefixExpression {
  fn expression_node() -> Node {
    Node{}
  }

  fn token_literal(&self) -> String {
    self.token.value.to_string()
  }
}

// Box avoids to add derive Clone
pub struct InfixExpression {
  pub token: Token,
  pub operator: String,
  pub left: Box<Expressions>,
  pub right: Box<Expressions>,
}
impl Expressions for InfixExpression {
  fn string(&self) -> String {
    ("(".to_owned() + &self.left.string() + " " + &self.operator + " " + &self.right.string() + ")")
  }
}

#[derive(Debug, Clone)]
pub struct Boolean {
  pub token: Token,
  pub value: bool,
}
impl Expressions for Boolean {
  fn string(&self) -> String {
    self.value.to_string()
  }
}
impl Boolean {
  fn token_literal(&self) -> String {
    self.token.value.to_string()
  }
}