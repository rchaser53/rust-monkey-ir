use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

// use std::fmt;

// mod part;
// mod walker;

// use part::*;
// use walker::*;

#[derive(Debug)]
enum TokenType {
  TokenIdentifier,
  TokenDigit,
  // TokenSymbol,
  // TokenInt,
  // TokenReturn,
  // TokenEof
}

#[derive(Debug)]
pub struct TempToken {
  byte_vec: Vec<u8>
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
struct AstToken {
  kind: TokenType,
  value: String
}

impl AstToken {
  fn new(kind: TokenType, value: String) -> AstToken {
    AstToken {
      kind: kind,
      value: value
    }
  }
}

#[derive(Debug)]
struct AstTokens {
  tokens: Vec<AstToken>,
  temp_stack: TempToken,
  num_flag: bool
}

impl AstTokens {
  pub fn new() -> AstTokens {
    AstTokens {
      tokens: Vec::new(),
      temp_stack: TempToken{ byte_vec: Vec::new() },
      num_flag: true
    }
  }

  pub fn add_token(&mut self) {
    let stack_length = self.temp_stack.byte_vec.len();

    if 0 < stack_length {
      let emit_string = self.temp_stack.emit_temp_str();

      let token = if self.num_flag == true {
        TokenType::TokenDigit
      } else {
        TokenType::TokenIdentifier
      };

      self.tokens.push(AstToken::new(
        token,
        emit_string
      ));
    }
    self.refresh();
  }

  pub fn refresh(&mut self) {
    self.num_flag = true;
  }

  pub fn read(&mut self, input: &str) {
    for byte in input.as_bytes() {
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
        b'+' | b'-' | b'*' | b'/' => {
          let stack_length = self.temp_stack.byte_vec.len();
          if 0 < stack_length {
            self.add_token();
          }
          self.temp_stack.add_temp_str(*byte);
          self.add_token();
        },
        b' ' => {
          self.add_token();
        },
        _ => {
          println!("koya-n");
        }
      }
    }
    self.add_token();
  }
}

fn read_file_to_ast_tokens(file_path: &str) -> io::Result<AstTokens> {
  let mut f = File::open(file_path)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  let mut ast_tokens = AstTokens::new();
  ast_tokens.read(&contents);

  Ok(ast_tokens)
}

fn main() {
  println!("{:?}", read_file_to_ast_tokens("input.txt"));
}

#[test]
fn it_works() {
  let mut ast_tokens = AstTokens::new();
  ast_tokens.read("0123 456");

  let temp_str = &ast_tokens.tokens[0].value;
  assert!(*temp_str == "0123", "should be type Identifier when start character is 0");
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