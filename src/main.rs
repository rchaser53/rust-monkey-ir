#[macro_use]
mod macros;

#[macro_use]
extern crate lazy_static;

mod lexer;
use lexer::lexer::*;

mod parser;
use parser::parser::*;

mod evalute;
use evalute::eval::*;

fn main() {
  let mut lexer = Lexer::new("0 /* 123 */ 2");
  println!("{:?}", lexer.next_token());
  println!("{:?}", lexer.next_token());
  println!("{:?}", lexer.next_token());
  let input = "
  1 + 3;
";
  let mut lexer = Lexer::new(input);
  let mut parser = Parser::new(&mut lexer);
  let program = parser.parse_program();

  let result_value = eval_program(program);

  println!("{}", result_value[0]);
  // println!("{:?}", lexer.next_token());
  // println!("{:?}", lexer.next_token());
  // println!("{:?}", lexer.next_token());



  // println!("{:?}", read_file_to_tokens("input.txt"));
}