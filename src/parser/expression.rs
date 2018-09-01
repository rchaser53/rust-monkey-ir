use parser::node::*;

#[derive(Debug, Clone)]
pub struct Expression {
  pub node: Node
}
impl Expression {
  pub fn expression_node(&mut self) -> Node {
    self.node.clone()
  }

  pub fn string(&self) -> String {
    self.node.string()
  }
}
