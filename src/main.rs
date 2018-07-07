#![feature(slice_concat_ext)]
use std::slice::SliceConcatExt;
use std::fmt;

#[derive(Debug)]
pub enum AstType {
  Root,
  Start,
  End,
  Normal,
  Delimiter
}

#[derive(Debug)]
pub struct PartArena {
  pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Part {
  id: usize,
  start: usize,
  end: usize,
  kind: AstType,
  value: char,
  children: Vec<usize>,
  parent: Option<usize>,
}

impl Part {
  fn new(kind: AstType, imput: char, start: usize) -> Part {
    Part {
      id: 0,
      start: start,
      end: 0,
      kind: kind,
      value: imput,
      children: Vec::new(),
      parent: None,
    }
  }

  fn add_child(&mut self, id: usize, kind: AstType, imput: char, start: usize) -> Part {
    let p = Part {
      id: id,
      start: start,
      end: 0,
      kind: kind,
      value: imput,
      children: Vec::new(),
      parent: Some(self.id),
    };
    self.children.push(p.id);
    p
  }
}

#[derive(Debug)]
pub enum WalkingType {
  Function,
  Normal,
}

#[derive(Debug)]
struct Walker<'a> {
  input: &'a str,
  next: usize,
  part: usize,
  part_arena: PartArena,
  current_type: WalkingType,
}

impl <'a>Walker<'a> {
  fn new(input: &str) -> Walker {
    let mut pa = PartArena{ parts: Vec::new() };
    pa.parts.push(Part::new(
      AstType::Root, ' ', 0
    ));
    
    Walker {
      input: input,
      next: 0,
      part: 0,
      part_arena: pa,
      current_type: WalkingType::Function,
    }
  }

  pub fn walk(&mut self) {
    let mut chars = self.input.chars();
    let mut index: usize = 0;
    let mut arena_id: usize = 0;
    let mut new_part: Part;
    
    while let Some(cha) = chars.next() {
      {
        let mut part = &mut self.part_arena.parts.get_mut(arena_id).unwrap();
        new_part = match cha {
          '{' => {
            let child_part = part.add_child(index + 1, AstType::Start, cha, index);
            arena_id = child_part.id;
            child_part
          },
          '}' => {
            arena_id = part.parent.unwrap();
            part.add_child(index + 1, AstType::Start, cha, index)
          },
          ' ' => {
            part.add_child(index + 1, AstType::Delimiter, cha, index)
          },
          _ => {
            part.add_child(index + 1, AstType::Normal, cha, index)
          }
        };
      }
      self.part_arena.parts.push(new_part);
      index += 1;
    }
  }
}


pub fn add_str(chars: &mut Vec<char>, part: &Part) -> bool {
  match part.kind {
    AstType::Normal => {
      chars.push(part.value);
      false
    },
    AstType::Delimiter => {
      if chars.len() == 0 {
        return false;
      }
      true
    }
    _ => {
      false
    }
  }
}

#[derive(Debug)]
enum TokenType {
  TOKEN_IDENTIFIER,
  TOKEN_DIGIT,
  TOKEN_SYMBOL,
  TOKEN_INT,
  TOKEN_RETURN,
  TOKEN_EOF
}

#[derive(Debug)]
struct TempToken {
  temp_str: String
}

// pub trait AddStack<T> {
//   fn add_stack(&mut self, value: T);
// }

// impl AddStack<u32> for TempToken<u32> {
//   fn add_stack(&mut self, value: u32) {
//     self.temp_str + value;
//   }
// }

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
  kind: TokenType
}

impl AstToken {
  fn new(kind: TokenType) -> AstToken {
    AstToken {
      kind: kind
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

  let mut ast_tokens = AstTokens { tokens: Vec::new() };

  let mut num_stack = TempToken{ temp_str: "".to_string() };
  let mut identifier_stack = TempToken{ temp_str: "".to_string() };
  let mut num_flag = true;
  let temp_str = "0123 ";

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

        if (0 < num_stack_length) {
          ast_tokens.add_token(AstToken::new(TokenType::TOKEN_DIGIT));
          // println!("num {}", num_stack.emit_temp_str());
        }

        if (0 < identifier_stack_length) {
          ast_tokens.add_token(AstToken::new(TokenType::TOKEN_IDENTIFIER));
          // println!("char {}", identifier_stack.emit_temp_str());
        }

        num_flag = true;
      },
      _ => {
        println!("koya-n");
      }
    }
  }



  // println!("{:?}, {:?}", num_stack.temp_str, identifier_stack.temp_str );

  println!("{:?}", ast_tokens);
}


// temp_char as u32 - '0' as u32