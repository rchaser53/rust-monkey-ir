use part::*;

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
  pub fn new(input: &str) -> Walker {
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