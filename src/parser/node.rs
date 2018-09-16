#[derive(Debug, Clone)]
pub struct Node {
  pub node_type: NodeType,
  pub value: String,
}
impl Node {
  pub fn token_literal(&mut self) -> String {
    String::new()
  }

  pub fn string(&self) -> String {
    String::new()
  }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum NodeType {
  Expression,
  Identifier,
  IntegerLiteral,
  PrefixExpression,
  InfixExpression,
  Boolean,
  IfExpression,
  FunctionLiteral,
  CallExpression
}