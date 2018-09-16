use std::fmt;

use lexer::token::*;
use parser::node::*;
use parser::statements::*;

pub trait Expressions {
  fn expression_node(&mut self) -> Node;
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
  fn expression_node(&mut self) -> Node {
    self.node.to_owned()
  }

  fn string(&self) -> String {
    self.node.string()
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

  fn expression_node(&mut self) -> Node {
    Node{
      node_type: NodeType::Identifier,
      value: self.value.to_owned(),
    }
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

  fn expression_node(&mut self) -> Node {
    Node{
      node_type: NodeType::IntegerLiteral,
      value: self.value.to_string(),
    }
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

  fn expression_node(&mut self) -> Node {
    Node{
      node_type: NodeType::PrefixExpression,
      value: self.operator.to_owned(),
    }
  }
}
impl PrefixExpression {
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

  fn expression_node(&mut self) -> Node {
    Node{
      node_type: NodeType::InfixExpression,
      value: self.operator.to_owned(),
    }
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

  fn expression_node(&mut self) -> Node {
    Node{
      node_type: NodeType::Boolean,
      value: self.value.to_string(),
    }
  }
}
impl Boolean {
  fn token_literal(&self) -> String {
    self.token.value.to_string()
  }
}

pub struct IfExpression {
  pub token: Token,
  pub condition: Box<Expressions>,
  pub consequence: BlockStatement,
  pub alternative: Option<BlockStatement>,
}
impl Expressions for IfExpression {
  fn string(&self) -> String {
    let ret_string = "if".to_owned() +  &self.condition.string() + " " + &self.consequence.string();

    if let Some(alt) = &self.alternative {
      return ret_string + "else " + &alt.string();
    }
    ret_string
  }

  fn expression_node(&mut self) -> Node {
    Node{
      node_type: NodeType::IfExpression,
      // value: self.value.to_string(),
      value: String::new(),
    }
  }
}
impl IfExpression {
  fn token_literal(&self) -> String {
    self.token.value.to_string()
  }
}

pub struct FunctionLiteral {
 pub token: Token,
 pub parameters: Vec<Identifier>,
 pub body: BlockStatement
}
impl Expressions for FunctionLiteral {
 fn string(&self) -> String {
  let mut params = Vec::new();
  for parameter in &self.parameters {
    params.push(parameter.string());
  }

  self.token_literal() + "(" + &params.join(", ") + ") " + &self.body.string()
 } 

  fn expression_node(&mut self) -> Node {
    Node{
      node_type: NodeType::FunctionLiteral,
      value: String::new(),
    }
  }
}
impl FunctionLiteral {
  fn token_literal(&self) -> String {
    self.token.value.to_string()
  }
}

pub struct CallExpression {
  pub token: Token,
  pub function: Box<Expressions>,
  pub arguments: Vec<Box<Expressions>>,
}
impl Expressions for CallExpression {
  fn string(&self) -> String {
    let mut args = Vec::new();
    for arg in &self.arguments {
      args.push(arg.string());
    }

    self.function.string() + "(" + &args.join(", ") + ")"
  }

  fn expression_node(&mut self) -> Node {
    Node{
      node_type: NodeType::CallExpression,
      value: String::new(),
    }
  }
}
impl CallExpression {
  fn token_literal(&self) -> String {
    self.token.value.to_string()
  }
}