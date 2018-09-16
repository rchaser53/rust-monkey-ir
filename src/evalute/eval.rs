use parser::node::*;
use evalute::object::*;

pub fn eval(node: Node) -> Box<Object> {
  match node.node_type {
    IntegerLiteral => {
      Box::new(Integer {
        // TBD error handling
        value: node.value.parse::<i64>().unwrap_or(0)
      })
    }
  }
}