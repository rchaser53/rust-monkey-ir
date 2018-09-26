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
  let input = "
  1 + 3 * 2;
";

  let mut lexer = Lexer::new(input);
  let mut parser = Parser::new(&mut lexer);
  let program = parser.parse_program();

  let eval = Eval::new();
  let result_value = eval.eval_program(program);

  println!("{}", result_value[0]);
  // println!("{:?}", lexer.next_token());
  // println!("{:?}", lexer.next_token());
  // println!("{:?}", lexer.next_token());
}