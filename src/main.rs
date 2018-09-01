mod lexer;
use lexer::lexer::*;

mod parser;
use parser::parser::*;

fn main() {
  let mut lexer = Lexer::new("0 /* 123 */ 2");
  println!("{:?}", lexer.next_token());
  println!("{:?}", lexer.next_token());
  println!("{:?}", lexer.next_token());
  // println!("{:?}", lexer.next_token());
  // println!("{:?}", lexer.next_token());
  // println!("{:?}", lexer.next_token());



  // println!("{:?}", read_file_to_tokens("input.txt"));
}