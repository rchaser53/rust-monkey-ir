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
    // temp
    Node{
      node_type: NodeType::Expression,
      value: String::new(),
    }
  }

  fn token_literal(&self) -> String {
    write_string!(self.token.value)
  }

  fn emit_debug_info(&self) -> String {
    write_string!(format!("[ {:?}, value: {:?}, {:?} ]", self.token, self.value, self.name))
  }

  fn string(&self) -> String {
    ("let ".to_owned() + &self.name.value + " = " + &self.value.string()).to_string()
  }
}
impl Default for LetStatement {
    fn default() -> LetStatement {
      LetStatement{
        token: Token{ kind: TokenType::TokenLet, value: write_string!("let") },
        value: Box::new(Expression{
          // temp
          node: Node{
            node_type: NodeType::Expression,
            value: String::new(),
          }
        }),
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
    // temp
    Node{
      node_type: NodeType::Identifier,
      value: String::new(),
    }
  }

  fn token_literal(&self) -> String {
    write_string!(self.token.value)
  }

  fn emit_debug_info(&self) -> String {
    write_string!(format!("[ {:?}, return_value: {:?} ]", self.token, self.return_value))
  }

  fn string(&self) -> String {
    ("return ".to_owned() + &self.return_value.string()).to_string()
  }
}

pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Box<Expressions>,
}
impl Statement for ExpressionStatement {
  fn statement_node(&self) -> Node {
    // temp
    Node{
      node_type: NodeType::Identifier,
      value: String::new(),
    }
  }

  fn token_literal(&self) -> String {
    write_string!(self.token.value)
  }

  fn emit_debug_info(&self) -> String {
    write_string!(format!("[{:?}, {:?}]", self.token, self.expression))
  }

  fn string(&self) -> String {
    self.expression.string()
  }
}

pub struct BlockStatement {
  pub token: Token,
  pub statements: Vec<Box<Statement>>
}

impl Statement for BlockStatement {
  fn statement_node(&self) -> Node {
    // temp
    Node{
      node_type: NodeType::Identifier,
      value: String::new(),
    }
  }

  fn token_literal(&self) -> String {
    write_string!(self.token.value)
  }

  fn emit_debug_info(&self) -> String {
    write_string!(format!("[{:?}, {:?}]", self.token, self.string()))
  }

  fn string(&self) -> String {
    let mut string_vec = Vec::new();

    for statement in &self.statements {
      string_vec.push(statement.string());
    }

    ("{".to_owned() + &string_vec.join("") + "}")
  }
}
