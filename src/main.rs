extern crate clap;
extern crate reqwest;
extern crate regex;
extern crate serde_json;

use std::rc::{Rc, Weak};

#[derive(Debug)]
pub enum AstType {
  Start,
  End,
  Normal,
}

#[derive(Debug)]
pub struct PartArena {
  pub parts: Vec<Part>,
}

// impl PartArena {
//   pub fn insert(mut self, part: Part) {
//     self.parts.push(part);
//   }
// }

#[derive(Debug)]
pub struct Part {
  id: usize,
  start: usize,
  end: usize,
  kind: AstType,
  value: char,
  children: Vec<Option<usize>>,
  parent: Option<usize>,
}

impl Part {
  fn new(id: usize, kind: AstType, imput: char, start: usize) {
    let p = Part {
      id: id,
      start: start,
      end: 0,
      kind: kind,
      value: imput,
      children: Vec::new(),
      parent: None,
    };
  }

  fn add_child(&mut self, id: usize, kind: AstType, imput: char, start: usize) {
    let c = Part {
      id: id,
      start: start,
      end: 0,
      kind: kind,
      value: imput,
      children: Vec::new(),
      parent: Some(self.id),
    };
    self.children.push(Some(c.id));
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
  part: usize,
  part_arena: PartArena,
  current_type: WalkingType,
}

impl <'a>Walker<'a> {
  fn new(input: &str) -> Walker {
    Walker {
      input: input,
      part: 0,
      part_arena: PartArena{ parts: Vec::new() },
      current_type: WalkingType::Function,
    }
  }

  // pub fn get_next_target(mut self, last_part: &mut Part, index: usize) -> &mut Part {
  //   if index == 0 {
  //     return last_part;
  //   }

  //   let last_index = index - 1;
  //   let last_char = last_part.value;

  //   if last_char == '{' {
  //     // last_part.add_child(AstType::Start, '{', 0, last_part);
  //     return &mut self.part_arena.parts[last_part.children[index].unwrap()]
  //   } else if last_char == '}' {
  //     // let def = last_part.parent.upgrade().unwrap();
  //     // return &mut Rc::get_mut(&mut last_part.parent.upgrade().unwrap()).unwrap();

  //   }

  //   return last_part;
  // }

  pub fn walk(&mut self) {
    let length = self.part_arena.parts.len();
    if let Some(part) = self.part_arena.parts.get_mut(self.part) {
      part.add_child(length, AstType::Start, '{', 0);
    }
    
    // self.part_arena.parts.push(part);


    // let mut chars = self.input.chars();
    // let mut index = 0;
    
    // let target = &mut self.part;
    // while let Some(cha) = chars.next() {
    //   let mut target = Walker::get_next_target(&mut target, index);
    //   match cha {
    //     '{' => {
    //       target.children.push(Rc::new(Part::new(AstType::Start, cha, index)));
    //       continue;
    //     },
    //     '}' => {
    //       target.children.push(Rc::new(Part::new(AstType::End, cha, index)));
    //     },
    //     ' ' => {
    //       target.children.push(Rc::new(Part::new(AstType::End, cha, index)));
    //     },
    //     _ => {
    //       target.children.push(Rc::new(Part::new(AstType::Normal, cha, index)));
    //     }
    //   };
    //   index += 1;
    // }
  }
}

fn main() {
  let mut walker = Walker::new("{a b  c}");
  walker.walk();

  let nyn: String = "abc".to_string();
  println!("{}", nyn);
}

// impl fmt::Debug for Part {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.kind)
//     }
// }
