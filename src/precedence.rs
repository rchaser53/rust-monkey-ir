mod lexer;
use lexer::*;

fn get_precedence(val: &str) -> i32 {
  match val {
    "+" | "-" => {
      10
    },
    "*" | "/" => {
      20
    },
    _ => {
      0
    }
  }
}


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


fn parse_expression(tokens: &mut std::slice::Iter<Token>, lhs: i32, min_precedence: i32) -> i32 {
  if let Some(token) = tokens.next() {
    let val = token.value.as_str();
    let mut precedence = get_precedence(val);

    while precedence <= min_precedence {
      if let Some(next_token) = tokens.next() {
        let next_val = next_token.value.as_str();
        precedence = get_precedence(val);
      } else {
        return lhs;
      }
    }
  } else {
    return lhs;
  }

  return 12;
}


fn main() {
  let mut tokens_struct = Tokens::new();
  tokens_struct.read("234 + 111");
  let mut tokens = tokens_struct.tokens;

  let mut tokens_iter: std::slice::Iter<Token> = tokens.iter();
  let token = tokens_iter.next().unwrap();

  let mut precedence = {
    get_precedence(token.value.as_str())
  };
  let mut int_value = token.value.parse::<i32>().unwrap();

  let aaa = parse_expression(&mut tokens_iter, int_value, precedence);

  println!("{}", aaa);
}





// fn nyan() -> Result<String, String> {
//   Ok("abc".to_string())
//   // Err("aa".to_string())
// }
// if let Ok(ho) = nyan() {
//   println!("{}", ho);
// }
