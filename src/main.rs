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
  walking_index: usize
}

impl <'a>Walker<'a> {
  fn new(input: &'a str) -> Walker {
    Walker {
      input: input,
      part: Vec::new(),
      current_type: WalkingType::Function,
      walking_index: 0
    }
  }

  fn walk(&mut self) {
    for (index, cha) in self.input.chars().enumerate() {
      match cha {
        '{' => {
          self.part.push(Part::new(AstType::Start, cha, index));
        },
        '}' => {
          self.part.push(Part::new(AstType::End, cha, index));
        },
        ' ' => {},
        _ => {
          self.part.push(Part::new(AstType::Normal, cha, index));
        }
      };
    }
  }
}

fn main() {
  let mut walker = Walker::new("{a b  c}");
  &walker.walk();

  println!("{:?}", walker);
}

// impl fmt::Debug for Part {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.kind)
//     }
// }
