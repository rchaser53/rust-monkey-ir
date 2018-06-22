extern crate clap;
extern crate reqwest;
extern crate regex;
extern crate serde_json;

#[derive(Debug)]
pub enum AstType {
  Start,
  End,
  Normal,
}

#[derive(Debug)]
struct Part {
  start: usize,
  end: usize,
  kind: AstType,
  value: char,
  children: Vec<Part>
}

impl Part {
  fn new(kind: AstType , imput: char, start: usize) -> Part {
    return Part {
      start: start,
      end: 0,
      kind: kind,
      value: imput,
      children: Vec::new()
    }
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
  part: Vec<Part>,
  current_type: WalkingType,
}

impl <'a>Walker<'a> {
  fn new(input: &'a str) -> Walker {
    Walker {
      input: input,
      part: Vec::new(),
      current_type: WalkingType::Function,
    }
  }

  pub fn get_next_target(last_part: &mut Vec<Part>, last_index: usize) -> &mut Vec<Part> {
    return last_part;
  }

  pub fn walk(&mut self) {
    let mut chars = self.input.chars();
    let mut index = 0;
    
    let mut target = &mut self.part;
    while let Some(cha) = chars.next() {
      if index != 0 {
        let mut target = Walker::get_next_target(target, index - 1);
      }
      match cha {
        '{' => {
          target.push(Part::new(AstType::Start, cha, index));
        },
        '}' => {
          target.push(Part::new(AstType::End, cha, index));
        },
        ' ' => {},
        _ => {
          target.push(Part::new(AstType::Normal, cha, index));
        }
      };
      index += 1;
    }
    println!("{:?}", target);
  }
}

fn main() {
  let mut walker = Walker::new("{a b  c}");
  &walker.walk();

  let nyn: String = "abc".to_string();
  println!("{}", nyn);

  // println!("{:?}", walker);
}

// impl fmt::Debug for Part {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.kind)
//     }
// }
