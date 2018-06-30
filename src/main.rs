extern crate clap;
extern crate reqwest;
extern crate regex;
extern crate serde_json;

use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub enum AstType {
  Start,
  End,
  Normal,
}

#[derive(Debug, Clone)]
struct Part<'a> {
  start: usize,
  end: usize,
  kind: AstType,
  value: char,
  children: Vec<Rc<Part<'a>>>,
  parent: Weak<&'a Part<'a>>,
}

impl <'a>Part<'a> {
  fn new(kind: AstType , imput: char, start: usize) -> Part<'a> {
    return Part {
      start: start,
      end: 0,
      kind: kind,
      value: imput,
      children: Vec::new(),
      parent: Weak::new(),
    }
  }

  // fn add_child(mut self, kind: AstType , imput: char, start: usize, parent: &'a mut Part) -> Part<'a> {
  fn add_child(mut self, kind: AstType , imput: char, start: usize, parent: &'a mut Part<'a>) {
    let child = Rc::new(Part {
      start: start,
      end: 0,
      kind: kind,
      value: imput,
      children: Vec::new(),
      parent: Rc::downgrade(&Rc::new(parent)),
    });
    self.children.push(child);
  }
}

#[derive(Debug, Clone)]
pub enum WalkingType {
  Function,
  Normal,
}

#[derive(Debug, Clone)]
struct Walker<'a> {
  input: &'a str,
  part: Part<'a>,
  current_type: WalkingType,
}

impl <'a>Walker<'a> {
  fn new(input: &'a str) -> Walker {
    Walker {
      input: input,
      part: Part::new(AstType::Start, ' ', 0),
      current_type: WalkingType::Function,
    }
  }

  pub fn get_next_target(last_part: &'a mut Part<'a>, index: usize) -> &'a mut Part<'a> {
    if index == 0 {
      return last_part;
    }

    let last_index = index - 1;
    let last_char = last_part.value;

    if last_char == '{' {
      // last_part.add_child(AstType::Start, '{', 0, last_part);
      return Rc::get_mut(&mut last_part.children[index]).unwrap();
    } else if last_char == '}' {
      // let def = last_part.parent.upgrade().unwrap();
      return &mut Rc::get_mut(&mut last_part.parent.upgrade().unwrap()).unwrap();

    }

    return last_part;
  }

  pub fn walk(&'_ mut self) {
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

  // println!("{:?}", walker);
}

// impl fmt::Debug for Part {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.kind)
//     }
// }
