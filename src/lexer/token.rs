#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  TokenIdentifier,
  TokenDigit,
  TokenSymbol,
  TokenInt,
  TokenReturn,
  TokenEof,
  TokenLet,
  TokenAssign,
  TokenColon,
  TokenSemicolon,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenType,
  pub value: String
}

impl Token {
  pub fn new(kind: TokenType, value: String) -> Token {
    Token {
      kind: kind,
      value: value
    }
  }
}

impl PartialEq for Token {
  fn eq(&self, other: &Token) -> bool {
      self.kind == other.kind && self.value == other.value
  }
}