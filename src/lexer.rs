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
  TokenComma,
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

  

  pub fn consume_number(&mut self, first_byte: u8) -> Token {
    let mut temp_vec: Vec<u8> = Vec::new();
    temp_vec.push(first_byte);

    let mut num_flag = true;
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
            ret_val = self.consume_number(byte);
            true
          },
          b'a' ... b'z' | b'A' ... b'Z' => {
            self.num_flag = false;
            self.temp_stack.add_temp_str(byte);
            false
          },
          b'/' => {
            // let next = ;
            // if next == None {
            //   self.temp_stack.add_temp_str(*byte);
            //   ret_val = self.create_token(TokenType::TokenSymbol);
            //   continue;
            // }
            if let Some(next) = self.get_next_char() {
              if next == b'*' {
                self.position += 1;
                self.consume_comment();
              } else {
                self.temp_stack.add_temp_str(byte);
                ret_val = self.create_token(TokenType::TokenSymbol);
              }
            } else {
              self.temp_stack.add_temp_str(byte);
              ret_val = self.create_token(TokenType::TokenSymbol);
            }
            true
          },
          b'+' | b'-' | b'{' | b'}' | b'(' | b')' | b'*' => {
            let stack_length = self.temp_stack.byte_vec.len();
            if 0 < stack_length {
              let token = self.get_token_type();
              ret_val = self.create_token(token);
            } else {
              self.temp_stack.add_temp_str(byte);
              ret_val = self.create_token(TokenType::TokenSymbol);
            }
            true
          },
          b'.' => {
            let token = self.get_token_type();
            ret_val = self.create_token(token);
            true
          },
          b',' => {
            ret_val = self.create_token(TokenType::TokenComma);
            true
          },
          b' ' => {
            // let token = self.get_token_type();
            // ret_val = self.create_token(token);
            false
          },
          b'\n' | b'\r' => {
            println!("{:?}", byte);
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

// pub fn read_file_to_tokens(file_path: &str) -> io::Result<Vec<Token>> {
//   let mut f = File::open(file_path)?;
//   let mut contents = String::new();
//   f.read_to_string(&mut contents)?;

//   let mut lexer = Lexer::new();
//   lexer.read(&contents);

//   Ok(lexer.tokens)
// }

#[test]
fn normal() {
  let mut lexer = Lexer::new("123 456");
  let first = lexer.next_token();
  assert!(first == Token::new(TokenType::TokenDigit, "123".to_string()), "{:?} an incorrect value.", first);

  let second = lexer.next_token();
  assert!(second == Token::new(TokenType::TokenDigit, "456".to_string()), "{:?} an incorrect value.", second);
}

// #[test]
// fn comment() {
//   let mut lexer = Lexer::new("0 /* 123 */ 2");
//   let first = lexer.next_token();
//   assert!(first == Token::new(TokenType::TokenDigit, "0".to_string()), "{:?} an incorrect value.", first);

//   let second = lexer.next_token();
//   assert!(second == Token::new(TokenType::TokenDigit, "2".to_string()), "{:?} an incorrect value.", second);
// }
