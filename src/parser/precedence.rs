use std::collections::HashMap;
use lexer::token::*;

lazy_static! {
  pub static ref PrecedenceTokenMap: HashMap<TokenType, Precedences>  = {
    let mut m = HashMap::new();
    // ==, !=
    m.insert(TokenType::TokenEq, Precedences::Equals);
    m.insert(TokenType::TokenNotEq, Precedences::Equals);

    // >, >=, <, <=
    m.insert(TokenType::TokenLt, Precedences::LessGrater);
    m.insert(TokenType::TokenLte, Precedences::LessGrater);
    m.insert(TokenType::TokenGt, Precedences::LessGrater);
    m.insert(TokenType::TokenGte, Precedences::LessGrater);

    // +, -
    m.insert(TokenType::TokenPlus, Precedences::Sum);
    m.insert(TokenType::TokenMinus, Precedences::Sum);

    // *, /
    m.insert(TokenType::TokenAsterisk, Precedences::Product);
    m.insert(TokenType::TokenSlash, Precedences::Product);
    

    m.insert(TokenType::TokenIdentifier, Precedences::Int);
    m
  };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Precedences {
  Int,
  Lowest,
  Equals,         // ==
  LessGrater,     // >, >=, <, <=
  Sum,            // +, -
  Product,        // *, /
  Prefix,         // -X, !X
  Call,           // the_function(X)
}


// token.EQ:
// token.NOT_EQ:
// token.LT:
// token.GT:
// token.PLUS:
// token.MINUS:
// token.SLASH:
// token.ASTERISK:
// [...]
// EQUALS, EQUALS, LESSGREATER, LESSGREATER, SUM,
// SUM, PRODUCT, PRODUCT,
// }