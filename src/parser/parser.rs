use lexer::lexer::*;
use lexer::token::*;

use parser::precedence::*;
use parser::node::*;
use parser::expression::*;
use parser::statements::*;
use parser::program::*;

pub struct Parser<'a> {
  pub l: &'a  mut Lexer<'a>,
  pub cur_token: Option<Token>,
  pub peek_token: Option<Token>,
  pub errors: Vec<String>,
}

impl <'a>Parser<'a> {
  pub fn new(l: &'a mut Lexer<'a>) -> Parser<'a> {
    let current_token = l.next_token();
    let peek_token = l.next_token();

    Parser{
      l: l,
      cur_token: current_token,
      peek_token: peek_token,
      errors: Vec::new(),
    }
  }

  pub fn next_token(&mut self) {
    self.cur_token = self.peek_token.clone();
    self.peek_token = self.l.next_token();
  }

  pub fn parse_program(&mut self) -> Program {
    let mut program = Program{
      statements: Vec::new()
    };
    while self.cur_token != None {
      if let Some(stmt) = self.parse_statement() {
        program.statements.push(stmt);
      }
      self.next_token();
    }

    if self.errors.len() > 0 {
      self.emit_error();
    }

    program
  }

  pub fn parse_statement(&mut self) -> Option<Box<Statement>> {
    if let Some(token) = &self.cur_token.clone() {
      return match token.kind {
        TokenType::TokenLet => {
          self.parse_let_statement()
        },
        TokenType::TokenReturn => {
          self.parse_return_statement()
        },
        _ => {
          self.parse_expression_statement()
        }
      }
    } else {
      return None;
    }
  }

  pub fn parse_let_statement(&mut self) -> Option<Box<Statement>> {
    let mut stmt = {
      match &self.cur_token {
        Some(token) => {
          LetStatement{
            token: Token{ kind: TokenType::TokenLet, value: write_string!("let") },
            value: Box::new(Expression{ node: Node{} }),
            name: Identifier{
              token: token.clone(),
              value: token.clone().value,
            },
          }
        },
        None => {
          return None;
        }
      }
    };

    if self.expect_peek(TokenType::TokenIdentifier) == false {
      return None;
    }
    
    if let Some(token) = &self.cur_token.clone() {
      let token_clone = token.clone();
      stmt.name = Identifier {
        token: token.clone(),
        value: token_clone.value,
      };

      if self.expect_peek(TokenType::TokenAssign) == false {
        return None;
      }

      self.next_token();
      stmt.value = if let Some(value) = self.parse_expression(Precedences::Lowest) {
        value
      } else {
        return None;
      };

      while self.peek_token_is(TokenType::TokenSemicolon) {
        self.next_token();
      }

      return Some(Box::new(stmt));
    }
    None
  }

  pub fn parse_return_statement(&mut self) -> Option<Box<Statement>> {
    let mut stmt = {
      match &self.cur_token {
        Some(token) => {
          ReturnStatement{
            token: token.clone(),
            return_value: Box::new(Expression{ node: Node{} }),
          }
        },
        None => {
          return None;
        }
      }
    };

    self.next_token();
    stmt.return_value = if let Some(value) = self.parse_expression(Precedences::Lowest) {
      value
    } else {
      return None;
    };

    while self.peek_token_is(TokenType::TokenSemicolon) {
      self.next_token();
    }

    return Some(Box::new(stmt));
  }

  pub fn parse_expression_statement(&mut self) -> Option<Box<Statement>> {
    let mut stmt = {
      match &self.cur_token {
        Some(token) => {
          ExpressionStatement{
            token: token.clone(),
            expression: Box::new(Expression{ node: Node{} }),
          }
        },
        None => {
          return None;
        }
      }
    };

    stmt.expression = if let Some(expression) = self.parse_expression(Precedences::Lowest) {
      expression
    } else {
      return None;
    };

    if self.peek_token_is(TokenType::TokenSemicolon) {
      self.next_token();
    }

    return Some(Box::new(stmt));
  }

  pub fn parse_identifier(&self) -> Option<Box<Expressions>> {
    if let Some(token) = &self.cur_token {
      return Some(Box::new(Identifier{
        token: token.clone(),
        value: token.clone().value,
      }));
    }
    None
  }

  pub fn parse_integer_literal(&mut self) -> Option<Box<Expressions>> {
    if let Some(token) = &self.cur_token {
      if let Ok(value) = token.value.parse::<i64>() {
        return Some(Box::new(
          IntegerLiteral{
            token: token.clone(),
            value: value,
        }));
      } else {
        self.errors.push(format!("could not parse {:?} as integer", token.value));
      }
    }
    None
  }

  pub fn parse_expression(&mut self, precedence: Precedences) -> Option<Box<Expressions>> {
    let mut left_exp: Option<Box<Expressions>> = None;
    if let Some(token) = &self.cur_token.clone() {
      left_exp = match token.kind {
        TokenType::TokenIdentifier => {
          self.parse_identifier()
        },
        TokenType::TokenDigit => {
          self.parse_integer_literal()
        },
        TokenType::TokenBang | TokenType::TokenMinus => {
          self.parse_prefix_expression()
        },
        _ => {
          self.no_prefix_parse_fn_error(token.kind);
          return None;
        },
      };
    }

    while self.peek_token_is(TokenType::TokenSemicolon) == false && precedence < self.peek_precedence() {
      if let Some(token) = &self.peek_token.clone() {
        left_exp = match token.kind {
          TokenType::TokenPlus | TokenType::TokenMinus | TokenType::TokenSlash | TokenType::TokenAsterisk |
          TokenType::TokenEq | TokenType::TokenNotEq |
          TokenType::TokenLt | TokenType::TokenLte | TokenType::TokenGt | TokenType::TokenGte => {
            self.next_token();
            self.parse_infix_expression(left_exp)
          },
          _ => {
            self.no_prefix_parse_fn_error(token.kind);
            return left_exp;
          },
        };
      }
    }

    left_exp
  }

  pub fn parse_prefix_expression(&mut self) -> Option<Box<Expressions>> {
    if let Some(token) = &self.cur_token.clone() {
      if let Some(right) = self.parse_expression(Precedences::Prefix) {
        return Some(Box::new(
          PrefixExpression{
            token: token.clone(),
            operator: token.clone().value,
            right: right,
        }));
      }
    }
    None
  }

  pub fn parse_infix_expression(&mut self, left: Option<Box<Expressions>>) -> Option<Box<Expressions>> {
    if left.is_none() {
      return None;
    }

    if let Some(token) = &self.cur_token.clone() {
      let precedence = self.cur_precedence();
      self.next_token();
      if let Some(right) = self.parse_expression(precedence) {
        return Some(Box::new(
          InfixExpression{
            token: token.clone(),
            operator: token.clone().value,
            left: left.unwrap(),
            right: right,
        }));
      }
    }
    None
  }

  pub fn cur_token_is(&self, t: TokenType) -> bool {
    if let Some(token) = &self.cur_token {
      return token.kind == t;
    }
    false
  }

  pub fn peek_token_is(&self, t: TokenType) -> bool {
    if let Some(token) = &self.peek_token {
      return token.kind == t;
    }
    false
  }

  pub fn expect_peek(&mut self, t: TokenType) -> bool {
    if self.peek_token_is(t) {
      self.next_token();
      return true;
    } else {
      self.peek_error(t);
      return false;
    }
  }
  pub fn peek_precedence(&mut self) -> Precedences {
    if let Some(token) = &self.peek_token {
      let token_type = token.kind;
      if PrecedenceTokenMap.contains_key(&token_type) {
        return PrecedenceTokenMap[&token_type].clone();
      }
    }
    Precedences::Lowest
  }

  pub fn cur_precedence(&mut self) -> Precedences {
    if let Some(token) = &self.cur_token {
      let token_type = token.kind;
      if PrecedenceTokenMap.contains_key(&token_type) {
        return PrecedenceTokenMap[&token_type].clone();
      }
    }
    Precedences::Lowest
  }

  pub fn emit_error(&self) {
    for error in self.errors.iter() {
      println!("{}", error);
    }
  }

  pub fn peek_error(&mut self, t: TokenType) {
    self.errors.push(format!("expected next token to be {:?} instead", t));
  }

  pub fn no_prefix_parse_fn_error(&mut self, t: TokenType) {
    self.errors.push(format!("no prefix parse function for {:?} found", t));
  }
}

/* below the test implementation */
#[warn(dead_code)]
fn statement_assert(statement: &Box<Statement>, expect: &str) {
  assert!(statement.string() == expect, statement.emit_debug_info());
}

#[test]
fn test_let_statements() {
  let input = "
    let x = 5;
    let y = 10;
    let foobar = 939393;
  ";
  let mut lexer = Lexer::new(input);
  let mut parser = Parser::new(&mut lexer);
  let program = parser.parse_program();

  assert!(program.statements.len() > 2, "failed parse correctly");

  let statement = program.statements;

  statement_assert(&statement[0], "let x = 5;");
  statement_assert(&statement[1], "let y = 10;");
  statement_assert(&statement[2], "let foobar = 939393;");
}

#[test]
fn test_return_statements() {
  let input = "
    return 5;
    return 10;
    return 939393;
  ";
  let mut lexer = Lexer::new(input);
  let mut parser = Parser::new(&mut lexer);
  let program = parser.parse_program();

  assert!(program.statements.len() > 2, "failed parse correctly");

  let statement = program.statements;

  statement_assert(&statement[0], "return 5;");
  statement_assert(&statement[1], "return 10;");
  statement_assert(&statement[2], "return 939393;");
}

// #[test]
// fn test_parse_expression_statement() {
//   let input = "
//     return 5;
//     return 10;
//     return 939393;
//   ";
//   let mut lexer = Lexer::new(input);
//   let mut parser = Parser::new(&mut lexer);
//   let program = parser.parse_program();

//   assert!(program.statements.len() > 2, "failed parse correctly");

//   let statement = program.statements;

//   statement_assert(&statement[0], "return 5;");
//   statement_assert(&statement[1], "return 10;");
//   statement_assert(&statement[2], "return 939393;");
// }
