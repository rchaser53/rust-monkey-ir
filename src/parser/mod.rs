#[macro_export]
macro_rules! write_string {
  ($w:expr) => ( $w.to_string() );
}

pub mod node;
pub mod expression;
pub mod identifier;
pub mod statements;
pub mod program;
pub mod precedence;
pub mod parser;