use lexer::token::*;

#[derive(Debug, Clone)]
pub struct Identifier {
  pub token: Token,
  pub value: String,
}

impl Identifier {
  pub fn expression_node() {}
  pub fn token_literal(&mut self) -> String {
    write_string!(self.token.value)
  }

  pub fn string(&self) -> String {
    self.value.to_string()
  }
}
impl Default for Identifier {
    fn default() -> Identifier {
      Identifier{
        token: Token{ kind: TokenType::TokenIdentifier, value: String::new() },
        value: String::new()
      }
    }
}