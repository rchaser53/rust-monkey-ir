extern crate encoding_rs;

use encoding_rs::*;

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
  temp_str: String
}

impl TempToken {
  fn add_temp_str(&mut self, value: char) {
    self.temp_str += &value.to_string();
  }

  fn emit_temp_str(&mut self) -> String {
    let ret_string = self.temp_str.clone();
    self.temp_str.clear();
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
  identifier_stack: TempToken,
  num_stack: TempToken,
  num_flag: bool
}

impl AstTokens {
  pub fn new() -> AstTokens {
    AstTokens {
      tokens: Vec::new(),
      identifier_stack: TempToken{ temp_str: String::new() },
      num_stack: TempToken{ temp_str: String::new() },
      num_flag: true
    }
  }

  pub fn add_token(&mut self) {
    let num_stack_length = self.num_stack.temp_str.len();
    let identifier_stack_length = self.identifier_stack.temp_str.len();

    if 0 < num_stack_length {
      let emit_string = self.num_stack.emit_temp_str();
      self.tokens.push(AstToken::new(
        TokenType::TokenDigit,
        emit_string
      ));
    }

    if 0 < identifier_stack_length {
      let emit_string = self.identifier_stack.emit_temp_str();
      self.tokens.push(AstToken::new(
        TokenType::TokenIdentifier,
        emit_string
      ));
    }
    self.num_flag = true;
  }

  pub fn read(&mut self, input: String) {
    for temp_char in input.chars() {
      match temp_char {
        '0' => {
          let stack_length = self.num_stack.temp_str.len();
          if stack_length == 0 {
            self.identifier_stack.add_temp_str(temp_char);
            self.num_flag = false;
          } else {
            self.num_stack.add_temp_str(temp_char);
          }
        },
        '1' ... '9' => {
          if self.num_flag == true {
            self.num_stack.add_temp_str(temp_char);
          } else {
            self.identifier_stack.add_temp_str(temp_char);
          }
        },
        ' ' => {
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

fn main() {
  let mut ast_tokens = AstTokens::new();

  ast_tokens.read("0123 456".to_string());


  println!("{}", ast_tokens.tokens[0].value);

  
}

#[test]
fn it_works() {
  let mut ast_tokens = AstTokens::new();
  ast_tokens.read("0123 456".to_string());

  let temp_str = &ast_tokens.tokens[0].value;
  assert!(*temp_str == "0123".to_string());
}

// temp_char as u32 - '0' as u32


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


  // let hoge = "konkonko";
  // for temp_char in hoge.as_bytes() {
  //   match temp_char {
  //     b'k' => println!("{}", temp_char),
  //     _ => {}
  //   }
  // }

  // let hhhh = "abcdefg";
  // println!("{:?}", &hhhh[0..2]);  // let hoge = b'a' ;
  // let mut buffer_bytes = [0u8; 8];
  // let mut buffer: &mut str = unsafe {
  //     std::mem::transmute(&mut buffer_bytes[..])
  // };

  // let mut decoder = UTF_8.new_decoder();
  // decoder.decode_to_str(&[hoge], &mut buffer, true);
  // println!("{}", buffer);