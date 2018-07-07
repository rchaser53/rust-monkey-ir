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
  tokens: Vec<AstToken>
}

impl AstTokens {
  pub fn add_token(&mut self, token: AstToken) {
    self.tokens.push(token);
  }
}

fn main() {
  let mut ast_tokens = AstTokens { tokens: Vec::new() };

  let mut num_stack = TempToken{ temp_str: String::new() };
  let mut identifier_stack = TempToken{ temp_str: String::new() };
  let mut num_flag = true;
  let temp_str = "0123 456 ";

  for temp_char in temp_str.chars() {
    match temp_char {
      '0' => {
        let stack_length = num_stack.temp_str.len();
        if stack_length == 0 {
          identifier_stack.add_temp_str(temp_char);
          num_flag = false;
        } else {
          num_stack.add_temp_str(temp_char);
        }
      },
      '1' ... '9' => {
        if num_flag == true {
          num_stack.add_temp_str(temp_char);
        } else {
          identifier_stack.add_temp_str(temp_char);
        }
      },
      ' ' => {
        let num_stack_length = num_stack.temp_str.len();
        let identifier_stack_length = identifier_stack.temp_str.len();

        if 0 < num_stack_length {
          ast_tokens.add_token(AstToken::new(
            TokenType::TokenDigit,
            num_stack.emit_temp_str()
          ));
        }

        if 0 < identifier_stack_length {
          ast_tokens.add_token(AstToken::new(
            TokenType::TokenIdentifier,
            identifier_stack.emit_temp_str()
          ));
        }

        num_flag = true;
      },
      _ => {
        println!("koya-n");
      }
    }
  }
  println!("{:?}", ast_tokens);
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