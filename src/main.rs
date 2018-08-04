// parse_expression ()
//     return parse_expression_1 (parse_primary (), 0)
// parse_expression_1 (lhs, min_precedence)
//     lookahead := peek next token
//     while lookahead is a binary operator whose precedence is >= min_precedence
//         op := lookahead
//         advance to next token
//         rhs := parse_primary ()
//         lookahead := peek next token
//         while lookahead is a binary operator whose precedence is greater
//                  than op's, or a right-associative operator
//                  whose precedence is equal to op's
//             rhs := parse_expression_1 (rhs, lookahead's precedence)
//             lookahead := peek next token
//         lhs := the result of applying op with operands lhs and rhs
//     return lhs

mod lexer;
use lexer::*;

fn main() {
  let mut tokens_struct = Tokens::new();
  tokens_struct.read("abc + 111");
  let tokens = tokens_struct.tokens;

  for token in tokens.iter() {
    let val = token.value.as_str();

    match val {
      "+" | "-" => {
        println!("{}, 10", val);
      },
      "*" | "/" => {
        println!("{}, 20", val);
      },
      _ => {}
    };
  }
}