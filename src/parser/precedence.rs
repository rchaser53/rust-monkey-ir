use std::collections::HashMap;
use lexer::token::*;

lazy_static! {
  pub static ref PrecedenceTokenMap: HashMap<TokenType, Precedences>  = {
    let mut m = HashMap::new();
    // ==, !=
    m.insert(TokenType::Eq, Precedences::Equals);
    m.insert(TokenType::NotEq, Precedences::Equals);

    // >, >=, <, <=
    m.insert(TokenType::Lt, Precedences::LessGrater);
    m.insert(TokenType::Lte, Precedences::LessGrater);
    m.insert(TokenType::Gt, Precedences::LessGrater);
    m.insert(TokenType::Gte, Precedences::LessGrater);

    // +, -
    m.insert(TokenType::Plus, Precedences::Sum);
    m.insert(TokenType::Minus, Precedences::Sum);

    // *, /
    m.insert(TokenType::Asterisk, Precedences::Product);
    m.insert(TokenType::Slash, Precedences::Product);

    m.insert(TokenType::Identifier, Precedences::Int);

    m.insert(TokenType::Lparen, Precedences::Call);
    m
  };
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Precedences {
  Int = 1,
  Lowest,
  Equals,         // ==
  LessGrater,     // >, >=, <, <=
  Sum,            // +, -
  Product,        // *, /
  Prefix,         // -X, !X
  Call,           // the_function(X)
}