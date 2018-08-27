use std;
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, PartialEq)]
pub enum TokenType {
  TokenIdentifier,
  TokenDigit,
  TokenSymbol,
  TokenInt,
  TokenReturn,
  TokenEof,
}

#[derive(Debug)]
pub struct TempToken {
  pub byte_vec: Vec<u8>
}

impl TempToken {
  fn add_temp_str(&mut self, value: u8) {
    self.byte_vec.push(value);
  }

  fn emit_temp_str(&mut self) -> String {
    let ret_string = String::from_utf8(self.byte_vec.clone()).unwrap();
    self.byte_vec.truncate(0);
    
    ret_string
  }
}

#[derive(Debug)]
pub struct Token {
  pub kind: TokenType,
  pub value: String
}

impl Token {
  fn new(kind: TokenType, value: String) -> Token {
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

#[derive(Debug)]
pub struct Lexer<'a> {
  pub bytes: &'a [u8],
  pub temp_stack: TempToken,
  pub num_flag: bool,
  pub position: usize,
  pub next_token: TokenType,
}

impl <'a>Lexer<'a> {
  pub fn new(input: &'a str) -> Lexer {
    let bytes = input.as_bytes();
    Lexer {
      bytes: bytes,
      temp_stack: TempToken{ byte_vec: Vec::new() },
      num_flag: true,
      position: 0,
      next_token: TokenType::TokenIdentifier,
    }
  }

  pub fn create_token(&mut self, token: TokenType) -> Token {
    let emit_string = self.temp_stack.emit_temp_str();
    Token::new(
      self.handle_reserved_word(&emit_string, token),
      emit_string.to_owned()
    )
  }

  pub fn handle_reserved_word(&self, word: &str, token: TokenType) -> TokenType {
    match word {
      "int" => TokenType::TokenInt,
      "return" => TokenType::TokenReturn,
      _ => token,
    }
  }

  // pub fn add_eof_token(&mut self) {
  //   self.tokens.push(Token::new(
  //     TokenType::TokenEof,
  //     String::new()
  //   ));
  // }

  pub fn get_next_char(&mut self) -> Option<u8> {
    if self.position < self.bytes.len() {
      return Some(self.bytes[self.position]);
    }
    None
  }

  pub fn get_token_type(&mut self) -> TokenType {
    if self.num_flag == true {
      TokenType::TokenDigit
    } else {
      TokenType::TokenIdentifier
    }
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

  pub fn next_token(&mut self) -> Token {
    let mut ret_val: Token = self.create_token(TokenType::TokenSymbol);
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
            let mut flag = false;
            if let Some(next) = self.get_next_char() {
              if next == b'*' {
                self.position += 1;
                self.consume_comment();
              } else {
                ret_val = self.create_token_by_value(TokenType::TokenSymbol, vec![byte]);
                flag = true;
              }
            } else {
              ret_val = self.create_token_by_value(TokenType::TokenSymbol, vec![byte]);
              flag = true;
            }
            flag
          },
          b',' | b'.' | b'+' | b'-' | b'{' | b'}' | b'(' | b')' | b'*' => {
            ret_val = self.create_token_by_value(TokenType::TokenSymbol, vec![byte]);
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

      } else {
        let token = self.get_token_type();
        ret_val = self.create_token(token);
        break;
      }
    }
    ret_val
  }
}

#[test]
fn digit() {
  let mut lexer = Lexer::new("123 456");
  let first = lexer.next_token();
  assert!(first == Token::new(TokenType::TokenDigit, "123".to_string()), "{:?} an incorrect value.", first);

  let second = lexer.next_token();
  assert!(second == Token::new(TokenType::TokenDigit, "456".to_string()), "{:?} an incorrect value.", second);
}

#[test]
fn identifier() {
  let mut lexer = Lexer::new("123 abc 45d6");
  let first = lexer.next_token();
  assert!(first == Token::new(TokenType::TokenDigit, "123".to_string()), "{:?} an incorrect value.", first);

  let second = lexer.next_token();
  assert!(second == Token::new(TokenType::TokenIdentifier, "abc".to_string()), "{:?} an incorrect value.", second);

  let third = lexer.next_token();
  assert!(third == Token::new(TokenType::TokenIdentifier, "45d6".to_string()), "{:?} an incorrect value.", third);
}

#[test]
fn comment() {
  let mut lexer = Lexer::new("0 /* 123 */ 2");
  let first = lexer.next_token();
  assert!(first == Token::new(TokenType::TokenDigit, "0".to_string()), "{:?} an incorrect value.", first);

  let second = lexer.next_token();
  assert!(second == Token::new(TokenType::TokenDigit, "2".to_string()), "{:?} an incorrect value.", second);
}

#[test]
fn division_multiple() {
  let mut lexer = Lexer::new("1 / 323 * 3 / 2");
  let first = lexer.next_token();
  assert!(first == Token::new(TokenType::TokenDigit, "1".to_string()), "{:?} an incorrect value.", first);

  let second = lexer.next_token();
  assert!(second == Token::new(TokenType::TokenSymbol, "/".to_string()), "{:?} an incorrect value.", second);

  let third = lexer.next_token();
  assert!(third == Token::new(TokenType::TokenDigit, "323".to_string()), "{:?} an incorrect value.", third);

  let forth = lexer.next_token();
  assert!(forth == Token::new(TokenType::TokenSymbol, "*".to_string()), "{:?} an incorrect value.", forth);

  let fifth = lexer.next_token();
  assert!(fifth == Token::new(TokenType::TokenDigit, "3".to_string()), "{:?} an incorrect value.", fifth);
}