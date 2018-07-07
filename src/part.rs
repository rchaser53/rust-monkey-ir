#[derive(Debug)]
pub struct PartArena {
  pub parts: Vec<Part>,
}

#[derive(Debug)]
pub enum AstType {
  Root,
  Start,
  End,
  Normal,
  Delimiter
}

#[derive(Debug)]
pub struct Part {
  pub id: usize,
  pub start: usize,
  pub end: usize,
  pub kind: AstType,
  pub value: char,
  pub children: Vec<usize>,
  pub parent: Option<usize>,
}

impl Part {
  pub fn new(kind: AstType, imput: char, start: usize) -> Part {
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

  pub fn add_child(&mut self, id: usize, kind: AstType, imput: char, start: usize) -> Part {
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