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
use evalute::object::*;

fn main() {
  let input = "
  1 + 3 * 2;
";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    let mut eval = Eval::new(Environment::new());
    let result_value = eval.eval_program(program);
