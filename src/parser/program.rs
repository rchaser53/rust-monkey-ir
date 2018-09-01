use std::fmt;
use parser::statements::*;

pub struct Program {
  pub statements: Vec<Box<Statement>>
}
impl Program {
  pub fn token_literal(&mut self) -> String {
     if self.statements.len() > 0 {
      self.statements[0].token_literal()
    } else {
      write_string!("")
    }
  }

  pub fn string(&mut self) -> String {
    let a: Vec<String> = self.statements.iter().map(|s| s.string()).collect();
    a.join("")
  }
}

impl fmt::Debug for Program {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let output: Vec<_> = self.statements.iter().map(|elem| { elem.emit_debug_info() }).collect();
    write!(f, "{:?}", output)
  }
}