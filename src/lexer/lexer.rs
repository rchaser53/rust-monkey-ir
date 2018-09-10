use lexer::token::*;

#[derive(Debug)]
pub struct Lexer<'a> {
  pub bytes: &'a [u8],
  pub position: usize,
}

impl <'a>Lexer<'a> {
  pub fn new(input: &'a str) -> Lexer {
    let bytes = input.as_bytes();
    Lexer {
      bytes: bytes,
      position: 0,
    }
  }

  pub fn create_eof_token(&mut self) -> Token {
    Token::new(
      TokenType::TokenEof,
      String::new()
    )
  }

  pub fn handle_reserved_word(&self, word: &str, token: TokenType) -> TokenType {
    match word {
      "let" => TokenType::TokenLet,
      "fn" => TokenType::TokenFn,
      "true" => TokenType::TokenTrue,
      "false" => TokenType::TokenFalse,
      "if" => TokenType::TokenIf,
      "else" => TokenType::TokenElse,
      "return" => TokenType::TokenReturn,
      _ => token,
    }
  }

  pub fn get_next_char(&mut self) -> Option<u8> {
    if self.position < self.bytes.len() {
      return Some(self.bytes[self.position]);
    }
    None
  }

  pub fn consume_comment(&mut self) {
    loop {
      if let Some(byte) = self.get_next_char() {
        self.position += 1;
        if byte == b'*' {
          if let Some(next) = self.get_next_char() {
            if next == b'/' {
              self.position += 1;
              break;
            }
          } else {
            break;
          }
        }
      } else {
        break;
      }
    }
  }

  pub fn consumue_character(&mut self, first_byte: u8, mut num_flag: bool) -> Token {
    let mut temp_vec: Vec<u8> = Vec::new();
    temp_vec.push(first_byte);
    loop {
      if let Some(byte) = self.get_next_char() {

        let break_flg = match byte {
          b'0' ... b'9' => {
            self.position += 1;
            temp_vec.push(byte);
            false
          },
          b'a' ... b'z' | b'A' ... b'Z' => {
            self.position += 1;
            temp_vec.push(byte);
            num_flag = false;
            false
          }
          _ => {
            true
          }
        };

        if break_flg == true {
          break;
        }

      } else {
        break;
      }
    }

    let token_type = if num_flag == true {
      TokenType::TokenDigit
    } else {
      TokenType::TokenIdentifier
    };

    self.create_token_by_value(token_type, temp_vec)
  }

  pub fn create_token_by_value(&mut self, token: TokenType, value_vec: Vec<u8>) -> Token {
    let ret_string = String::from_utf8(value_vec).unwrap();
    Token::new(
      self.handle_reserved_word(&ret_string, token),
      ret_string.to_owned()
    )
  }

  pub fn consume_slash(&mut self, target_token: Token) -> (Token, bool) {
    if let Some(next) = self.get_next_char() {
      if next == b'*' {
        self.position += 1;
        self.consume_comment();
        return (target_token, false);
      }
    }
    (self.create_token_by_value(TokenType::TokenSlash, vec![b'/']), true)
  }

  pub fn consume_equal(&mut self) -> Token {
    if let Some(next) = self.get_next_char() {
      if next == b'=' {
        self.position += 1;
        return self.create_token_by_value(TokenType::TokenEq, vec![b'=', b'=']);
      }
    }
    self.create_token_by_value(TokenType::TokenAssign, vec![b'='])
  }

  pub fn consume_ban(&mut self) -> Token {
    if let Some(next) = self.get_next_char() {
      if next == b'=' {
        self.position += 1;
        return self.create_token_by_value(TokenType::TokenNotEq, vec![b'!', b'=']);
      }
    }
    self.create_token_by_value(TokenType::TokenBang, vec![b'!'])
  }

  pub fn consume_lt(&mut self) -> Token {
    if let Some(next) = self.get_next_char() {
      if next == b'=' {
        self.position += 1;
        return self.create_token_by_value(TokenType::TokenLte, vec![b'<', b'=']);
      }
    }
    self.create_token_by_value(TokenType::TokenLt, vec![b'<'])
  }

  pub fn consume_gt(&mut self) -> Token {
    if let Some(next) = self.get_next_char() {
      if next == b'=' {
        self.position += 1;
        return self.create_token_by_value(TokenType::TokenGte, vec![b'>', b'=']);
      }
    }
    self.create_token_by_value(TokenType::TokenGt, vec![b'>'])
  }

  pub fn next_token(&mut self) -> Option<Token> {
    let mut ret_val: Token = self.create_eof_token();
    loop {
      if let Some(byte) = self.get_next_char() {
        self.position += 1;
        let flag = match byte {
          b'0' ... b'9' => {
            ret_val = self.consumue_character(byte, true);
            true
          },
          b'a' ... b'z' | b'A' ... b'Z' => {
            ret_val = self.consumue_character(byte, false);
            true
          },
          b'/' => {
            let (temp_ret, flag) = self.consume_slash(ret_val);
            ret_val = temp_ret;
            flag
          },
          b'=' => {
            ret_val = self.consume_equal();
            true
          },
          b',' | b'.' => {
            ret_val = self.create_token_by_value(TokenType::TokenSymbol, vec![byte]);
            true
          },
          b'{' => {
            ret_val = self.create_token_by_value(TokenType::TokenLbrace, vec![byte]);
            true
          },
          b'}' => {
            ret_val = self.create_token_by_value(TokenType::TokenRbrace, vec![byte]);
            true
          },
          b'(' => {
            ret_val = self.create_token_by_value(TokenType::TokenLparen, vec![byte]);
            true
          },
          b')' => {
            ret_val = self.create_token_by_value(TokenType::TokenRparen, vec![byte]);
            true
          },
          b'!' => {
            ret_val = self.consume_ban();
            true
          },
          b'*' => {
            ret_val = self.create_token_by_value(TokenType::TokenAsterisk, vec![byte]);
            true
          },
          b'+' => {
            ret_val = self.create_token_by_value(TokenType::TokenPlus, vec![byte]);
            true
          },
          b'-' => {
            ret_val = self.create_token_by_value(TokenType::TokenMinus, vec![byte]);
            true
          },
          b'<' => {
            ret_val = self.consume_lt();
            true
          },
          b'>' => {
            ret_val = self.consume_gt();
            true
          },
          b':' => {
            ret_val = self.create_token_by_value(TokenType::TokenColon, vec![byte]);
            true
          },
          b';' => {
            ret_val = self.create_token_by_value(TokenType::TokenSemicolon, vec![byte]);
            true
          },
          b'\n' | b'\r' | b' ' => {
            false
          },
          _ => {
            panic!("{} cannot be handled.", byte);
          }
        };

        if flag == true {
          break;
        }

      }
       else {
        return None;
      }
    }
    Some(ret_val)
  }
}

// below is test implementation

#[warn(dead_code)]
fn lexer_assert(token: Token, token_type: TokenType, value: &str) {
  let expected = Token::new(token_type, value.to_string());
  assert!(token == expected, "\r\nexpected: {:?} \r\nactual: {:?}", expected, token);
}

#[test]
fn digit() {
  let mut lexer = Lexer::new("123 456");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "123");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "456");
}

#[test]
fn identifier() {
  let mut lexer = Lexer::new("123 abc 45d6");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "123");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenIdentifier, "abc");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenIdentifier, "45d6");
}

#[test]
fn comment() {
  let mut lexer = Lexer::new("0 /* 123 */ 2");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "0");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "2");
}

#[test]
fn ban() {
  let mut lexer = Lexer::new("let abc = !abc");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenLet, "let");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenIdentifier, "abc");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenAssign, "=");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenBang, "!");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenIdentifier, "abc");
}

#[test]
fn division_multiple() {
  let mut lexer = Lexer::new("1 / 323 * 3 / 2");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "1");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenSlash, "/");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "323");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenAsterisk, "*");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "3");
}

#[test]
fn gt() {
  let mut lexer = Lexer::new("123 <= 456");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "123");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenLte, "<=");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "456");
}

#[test]
fn if_test() {
  let mut lexer = Lexer::new("if 123 == 456");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenIf, "if");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "123");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenEq, "==");
  lexer_assert(lexer.next_token().unwrap(), TokenType::TokenDigit, "456");
}
