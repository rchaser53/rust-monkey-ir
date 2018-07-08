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
}

fn main() {
  let mut ast_tokens = AstTokens::new();

  let temp_str = "0123 456";

  for temp_char in temp_str.chars() {
    match temp_char {
      '0' => {
        let stack_length = ast_tokens.num_stack.temp_str.len();
        if stack_length == 0 {
          ast_tokens.identifier_stack.add_temp_str(temp_char);
          ast_tokens.num_flag = false;
        } else {
          ast_tokens.num_stack.add_temp_str(temp_char);
        }
      },
      '1' ... '9' => {
        if ast_tokens.num_flag == true {
          ast_tokens.num_stack.add_temp_str(temp_char);
        } else {
          ast_tokens.identifier_stack.add_temp_str(temp_char);
        }
      },
      ' ' => {
        ast_tokens.add_token();
      },
      _ => {
        println!("koya-n");
      }
    }
  }

  ast_tokens.add_token();
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