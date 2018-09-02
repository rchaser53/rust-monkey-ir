#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TokenType {
  TokenIdentifier,
  TokenDigit,
  TokenSymbol,
  
  TokenEof,
  TokenAssign,
  TokenColon,
  TokenSemicolon,
  
  // preserve word
  TokenFn,              // fn
  TokenTrue,            // true
  TokenFalse,           // false
  TokenIf,              // if
  TokenElse,            // else
  TokenLet,             // let
  TokenReturn,          // return

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

#[derive(Clone, Debug)]
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