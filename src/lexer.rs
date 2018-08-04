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
  TokenEof
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

#[derive(Debug)]
pub struct Tokens {
  pub tokens: Vec<Token>,
  pub temp_stack: TempToken,
  pub num_flag: bool,
  pub next_token: TokenType
}

impl Tokens {
  pub fn new() -> Tokens {
    Tokens {
      tokens: Vec::new(),
      temp_stack: TempToken{ byte_vec: Vec::new() },
      num_flag: true,
      next_token: TokenType::TokenIdentifier
    }
  }

  pub fn add_token(&mut self, token: TokenType) {
    let stack_length = self.temp_stack.byte_vec.len();
    let emit_string = self.temp_stack.emit_temp_str();

    if 0 < stack_length {
      let token = self.handle_reserved_word(&emit_string, token);

      self.tokens.push(Token::new(
        token,
        emit_string.to_owned()
      ));
    }
    self.refresh();
  }

  pub fn handle_reserved_word(&self, word: &str, token: TokenType) -> TokenType {
    match word {
      "int" => TokenType::TokenInt,
      "return" => TokenType::TokenReturn,
      _ => token,
    }
  }

  pub fn add_eof_token(&mut self) {
    self.tokens.push(Token::new(
      TokenType::TokenEof,
      String::new()
    ));
  }

  pub fn refresh(&mut self) {
    self.num_flag = true;
  }

  pub fn get_token_type(&mut self) -> TokenType {
    if self.num_flag == true {
      TokenType::TokenDigit
    } else {
      TokenType::TokenIdentifier
    }
  }

  pub fn consume_comment(&mut self, bytes: &mut std::slice::Iter<u8>) {
    while let Some(byte) = bytes.next() {
      if *byte == b'*' {
        let next = bytes.next();
        if next != None && *next.unwrap() == b'/' {
          break;
        }
      }
    }
  }

  pub fn read(&mut self, input: &str) {
    let mut bytes = input.as_bytes().into_iter();
    while let Some(byte) = bytes.next() {
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
          let next = bytes.next();
          if next == None {
            self.temp_stack.add_temp_str(*byte);
            self.add_token(TokenType::TokenSymbol);
            continue;
          }
          if *next.unwrap() == b'*' {
            self.consume_comment(&mut bytes);
          } else {
            self.temp_stack.add_temp_str(*byte);
            self.add_token(TokenType::TokenSymbol);
          }
        },
        b'+' | b'-' | b'{' | b'}' | b'(' | b')' | b'*' => {
          let stack_length = self.temp_stack.byte_vec.len();
          if 0 < stack_length {
            let token = self.get_token_type();
            self.add_token(token);
          }
          self.temp_stack.add_temp_str(*byte);
          self.add_token(TokenType::TokenSymbol);
        },
        b'.' => {
          let token = self.get_token_type();
          self.add_token(token);
        },
        b' ' | b',' => {
          let token = self.get_token_type();
          self.add_token(token);
        },
        b'\n' | b'\r' => {},
        _ => {
          panic!("{} cannot be handled.", byte);
        }
      }
    }
    let token = self.get_token_type();
    self.add_token(token);
    self.add_eof_token();
  }
}

pub fn read_file_to_tokens(file_path: &str) -> io::Result<Tokens> {
  let mut f = File::open(file_path)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  let mut tokens = Tokens::new();
  tokens.read(&contents);

  Ok(tokens)
}

// fn main() {
//   println!("{:?}", read_file_to_tokens("input.txt"));
// }

#[test]
fn normal() {
  let mut tokens = Tokens::new();
  tokens.read("0123 456");

  let temp_str = &tokens.tokens[0].value;
  assert!(*temp_str == "0123", "should be type Identifier when start character is 0");
}

#[test]
fn comment() {
  let mut tokens = Tokens::new();
  tokens.read("0 /* 123 */ 2");

  let temp_str = &tokens.tokens[1].value;
  assert!(*temp_str == "2", "should ignore comment '123'");
}


  // let mut walker = Walker::new("{afda {b  c} } ");
  // walker.walk();

  // let mut chars: Vec<char> = Vec::new();
  // let mut strs: Vec<String> = Vec::new();
  // for part in walker.part_arena.parts.iter() {
  //   if add_str(&mut chars, part) {
  //     strs.push(chars.iter().collect::<String>());
  //     chars.truncate(0);
  //   }
  // }
  // println!("{:?}", strs);

  // println!("{:?}, {:?}", num_stack.temp_str, identifier_stack.temp_str );

  // let hhhh = "abcdefg";
  // println!("{:?}", &hhhh[0..2]);