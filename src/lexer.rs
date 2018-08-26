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
  pub bytes: std::slice::Iter<'a, u8>,
  pub temp_stack: TempToken,
  pub num_flag: bool,
  pub next_token: TokenType,
}

impl <'a>Lexer<'a> {
  pub fn new(input: &'a str) -> Lexer {
    let bytes = input.as_bytes().into_iter();
    Lexer {
      bytes: bytes,
      temp_stack: TempToken{ byte_vec: Vec::new() },
      num_flag: true,
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

  pub fn get_token_type(&mut self) -> TokenType {
    if self.num_flag == true {
      TokenType::TokenDigit
    } else {
      TokenType::TokenIdentifier
    }
  }

  pub fn consume_comment(&mut self) {
    while let Some(byte) = self.bytes.next() {
      if *byte == b'*' {
        let next = self.bytes.next();
        if next != None && *next.unwrap() == b'/' {
          break;
        }
      }
    }
  }

  pub fn next_token(&mut self) -> Token {
    let mut ret_val: Token = self.create_token(TokenType::TokenSymbol);
    while let Some(byte) = self.bytes.next() {
      match byte {
        b'0' => {
          let stack_length = self.temp_stack.byte_vec.len();
          if stack_length == 0 {
            self.num_flag = false;
          }
          self.temp_stack.add_temp_str(*byte);
        },
        b'1' ... b'9' => {
          self.temp_stack.add_temp_str(*byte);
        },
        b'a' ... b'z' | b'A' ... b'Z' => {
          self.num_flag = false;
          self.temp_stack.add_temp_str(*byte);
        },
        b'/' => {
          let next = self.bytes.next();
          // if next == None {
          //   self.temp_stack.add_temp_str(*byte);
          //   ret_val = self.create_token(TokenType::TokenSymbol);
          //   continue;
          // }
          if *next.unwrap() == b'*' {
            self.consume_comment();
          } else {
            self.temp_stack.add_temp_str(*byte);
            ret_val = self.create_token(TokenType::TokenSymbol);
            break;
          }
        },
        b'+' | b'-' | b'{' | b'}' | b'(' | b')' | b'*' => {
          let stack_length = self.temp_stack.byte_vec.len();
          if 0 < stack_length {
            let token = self.get_token_type();
            ret_val = self.create_token(token);
            break;
          }
          self.temp_stack.add_temp_str(*byte);
          ret_val = self.create_token(TokenType::TokenSymbol);
          break;
        },
        b'.' => {
          let token = self.get_token_type();
          ret_val = self.create_token(token);
          break;
        },
        b',' => {
          ret_val = self.create_token(TokenType::TokenComma);
          break;
        },
        b' ' => {
          let token = self.get_token_type();
          ret_val = self.create_token(token);
        },
        b'\n' | b'\r' => {
          println!("{:?}", byte);
        },
        _ => {
          panic!("{} cannot be handled.", byte);
        }
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
  let mut lexer = Lexer::new("0123 456");
  let first = lexer.next_token();
  assert!(first == Token::new(TokenType::TokenIdentifier, "0123".to_string()), "{:?} an incorrect value.", first);

  let second = lexer.next_token();
  assert!(second == Token::new(TokenType::TokenDigit, "456".to_string()), "{:?} an incorrect value.", second);
}

// #[test]
// fn comment() {
//   let mut lexer = Lexer::new();
//   lexer.read("0 /* 123 */ 2");

//   let temp_str = &lexer.tokens[1].value;
//   assert!(*temp_str == "2", "should ignore comment '123'");
// }
