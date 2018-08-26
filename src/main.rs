mod lexer;
use lexer::*;

fn main() {
  println!("{:?}", read_file_to_tokens("input.txt"));
}