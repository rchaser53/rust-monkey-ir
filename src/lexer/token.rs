#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  TokenIdentifier,
  TokenDigit,
  TokenSymbol,
  TokenReturn,
  TokenEof,
  TokenLet,
  TokenAssign,
  TokenColon,
  TokenSemicolon,
  
  // for Arithmetic
  TokenEq,              // =
  TokenNotEq,           // !=
  TokenLt,              // <
  TokenLte,             // <=
  TokenGt,              // >
  TokenGte,             // =>
  TokenPlus,            // +
  TokenMinus,           // -
  TokenSlash,           // /
  TokenAsterisk,        // *
  TokenBan,
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