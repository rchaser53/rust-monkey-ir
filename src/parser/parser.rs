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
    self.cur_token = self.peek_token.to_owned();
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
    if let Some(token) = self.cur_token.to_owned() {
      return match token.kind {
        TokenType::Let => {
          self.parse_let_statement()
        },
        TokenType::Return => {
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
            token: Token{ kind: TokenType::Let, value: write_string!("let") },
            value: Box::new(Expression{
              node: Node{
                node_type: NodeType::Expression,
                value: token.value.to_owned()
              }
            }),
            name: Identifier{
              token: token.to_owned(),
              value: token.value.to_owned(),
            },
          }
        },
        None => {
          return None;
        }
      }
    };

    if self.expect_peek(TokenType::Identifier) == false {
      return None;
    }
    
    if let Some(token) = self.cur_token.to_owned() {
      stmt.name = Identifier {
        token: token.to_owned(),
        value: token.value.to_owned(),
      };

      if self.expect_peek(TokenType::Assign) == false {
        return None;
      }

      self.next_token();
      stmt.value = if let Some(value) = self.parse_expression(Precedences::Lowest) {
        value
      } else {
        return None;
      };

      while self.peek_token_is(TokenType::Semicolon) {
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
            token: token.to_owned(),
            return_value: Box::new(Expression{
              node: Node{
                node_type: NodeType::Expression,
                value: token.value.to_owned()
              }
            }),
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

    while self.peek_token_is(TokenType::Semicolon) {
      self.next_token();
    }

    return Some(Box::new(stmt));
  }

  pub fn parse_expression_statement(&mut self) -> Option<Box<Statement>> {
    let mut stmt = {
      match &self.cur_token {
        Some(token) => {
          ExpressionStatement{
            token: token.to_owned(),
            expression: Box::new(Expression{
              node: Node{
                node_type: NodeType::Expression,
                value: token.value.to_owned()
              } 
            }),
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

    if self.peek_token_is(TokenType::Semicolon) {
      self.next_token();
    }

    return Some(Box::new(stmt));
  }

  pub fn parse_identifier(&self) -> Option<Box<Expressions>> {
    if let Some(token) = &self.cur_token {
      return Some(Box::new(Identifier{
        token: token.to_owned(),
        value: token.value.to_owned(),
      }));
    }
    None
  }

  pub fn parse_integer_literal(&mut self) -> Option<Box<Expressions>> {
    if let Some(token) = &self.cur_token {
      if let Ok(value) = token.value.parse::<i64>() {
        return Some(Box::new(
          IntegerLiteral{
            token: token.to_owned(),
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
    if let Some(token) = self.cur_token.to_owned() {
      left_exp = match token.kind {
        TokenType::Identifier => {
          self.parse_identifier()
        },
        TokenType::Digit => {
          self.parse_integer_literal()
        },
        TokenType::Bang | TokenType::Minus => {
          self.parse_prefix_expression()
        },
        TokenType::Lparen => {
          self.parse_grouped_expression()
        },
        TokenType::True | TokenType::False => {
          self.parse_boolean()
        },
        TokenType::If => {
          self.parse_if_expression()
        },
        TokenType::Fn => {
          self.parse_function_literal()
        },
        _ => {
          self.no_prefix_parse_fn_error(token.kind);
          return None;
        },
      };
    }

    while self.peek_token_is(TokenType::Semicolon) == false && precedence < self.peek_precedence() {
      if let Some(token) = self.peek_token.to_owned() {
        left_exp = match token.kind {
          TokenType::Plus | TokenType::Minus | TokenType::Slash | TokenType::Asterisk |
          TokenType::Eq | TokenType::NotEq |
          TokenType::Lt | TokenType::Lte | TokenType::Gt | TokenType::Gte => {
            self.next_token();
            self.parse_infix_expression(left_exp)
          },
          TokenType::Lparen => {
            self.next_token();
            self.parse_call_expression(left_exp)
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

  pub fn parse_boolean(&mut self) -> Option<Box<Expressions>> {
    if let Some(token) = self.cur_token.to_owned() {
      return Some(Box::new(
        Boolean{
          token: token,
          value: self.cur_token_is(TokenType::True)
      }))
    }
    None
  }

  pub fn parse_prefix_expression(&mut self) -> Option<Box<Expressions>> {
    if let Some(token) = self.cur_token.to_owned() {
      self.next_token();
      if let Some(right) = self.parse_expression(Precedences::Prefix) {
        return Some(Box::new(
          PrefixExpression{
            token: token.to_owned(),
            operator: token.value.to_owned(),
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

    if let Some(token) = self.cur_token.to_owned() {
      let precedence = self.cur_precedence();
      self.next_token();
      if let Some(right) = self.parse_expression(precedence) {
        return Some(Box::new(
          InfixExpression{
            token: token.to_owned(),
            operator: token.value.to_owned(),
            left: left.unwrap(),
            right: right,
        }));
      }
    }
    None
  }

  pub fn parse_if_expression(&mut self) -> Option<Box<Expressions>> {
    if let Some(token) = self.cur_token.to_owned() {
      if self.expect_peek(TokenType::Lparen) == false {
        return None;
      }
      self.next_token();

      if let Some(condition) = self.parse_expression(Precedences::Lowest) {
        if self.expect_peek(TokenType::Rparen) == false {
          return None;
        }

        if self.expect_peek(TokenType::Lbrace) == false {
          return None;
        }

        if let Some(consequence) = self.parse_block_statement() {
          let alternative = if self.peek_token_is(TokenType::Else) {
            self.next_token();
            if self.expect_peek(TokenType::Lbrace) == false {
              return None;
            }
            self.parse_block_statement()
          } else {
            None
          };

          return Some(Box::new(IfExpression{
            token: token,
            condition: condition,
            consequence: consequence,
            alternative: alternative
          }));
        }
      }
    }
    None
  }

  pub fn parse_grouped_expression(&mut self) -> Option<Box<Expressions>> {
    self.next_token();
    let exp = if let Some(ret) = self.parse_expression(Precedences::Lowest) {
      ret
    } else {
      return None;
    };

    if self.expect_peek(TokenType::Rparen) == false {
      return None
    }
    Some(exp)
  }

  pub fn parse_function_literal(&mut self) -> Option<Box<Expressions>> {
    if let Some(token) = self.cur_token.to_owned() {
      if self.expect_peek(TokenType::Lparen) == false {
        return None;
      }

      let parameters = self.parse_function_parameters();

      if self.expect_peek(TokenType::Lbrace) == false {
        return None;
      }

      if let Some(body) = self.parse_block_statement() {
        return Some(Box::new(
          FunctionLiteral{
            token: token,
            parameters: parameters,
            body: body,
          }
        ));
      }
    }
    None
  }

  pub fn parse_call_expression(&mut self, function: Option<Box<Expressions>>) -> Option<Box<Expressions>> {
    if function.is_none() {
      return None;
    }

    if let Some(token) = self.cur_token.to_owned() {
      return Some(Box::new(
        CallExpression{
          token: token,
          function: function.unwrap(),
          arguments: self.parse_call_arguments(),
        }
      ));
    }
    None
  }

  pub fn parse_call_arguments(&mut self) -> Vec<Box<Expressions>> {
    let mut args = Vec::new();
    if self.peek_token_is(TokenType::Rparen) {
      self.next_token();
      return args;
    }
    self.next_token();

    if let Some(arg) = self.parse_expression(Precedences::Lowest) {
      args.push(arg);
    }

    while self.peek_token_is(TokenType::Comma) {
      self.next_token();
      self.next_token();
      if let Some(arg) = self.parse_expression(Precedences::Lowest) {
        args.push(arg);
      }
    }

    if self.expect_peek(TokenType::Rparen) == false {
      return args;
    }
    args
  }

  pub fn parse_function_parameters(&mut self) -> Vec<Identifier> {
    let mut parameters = Vec::new();

    if self.peek_token_is(TokenType::Rparen) {
      self.next_token();
      return parameters;
    }
    self.next_token();

    if let Some(token) = self.cur_token.to_owned() {
      parameters.push(Identifier{
        token: token.to_owned(),
        value: token.value.to_owned()
      });
    }
    
    while self.peek_token_is(TokenType::Comma) {
      self.next_token();
      self.next_token();

      if let Some(token) = self.cur_token.to_owned() {
        parameters.push(Identifier{
          token: token.to_owned(),
          value: token.value.to_owned()
        });
      }
    }

    if self.expect_peek(TokenType::Rparen) == false {
      return Vec::new();
    }

    parameters
  }

  pub fn parse_block_statement(&mut self) -> Option<BlockStatement> {
    if let Some(token) = self.cur_token.to_owned() {
      let mut block = BlockStatement{
        token: token,
        statements: Vec::new()
      };

      self.next_token();

      while self.cur_token_is(TokenType::Rbrace) == false && self.cur_token.is_none() == false {
        if let Some(stmt) = self.parse_statement() {
          block.statements.push(stmt);
        }
        self.next_token();
      }
      return Some(block);
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
        return PrecedenceTokenMap[&token_type].to_owned();
      }
    }
    Precedences::Lowest
  }

  pub fn cur_precedence(&self) -> Precedences {
    if let Some(token) = &self.cur_token {
      let token_type = token.kind;
      if PrecedenceTokenMap.contains_key(&token_type) {
        return PrecedenceTokenMap[&token_type].to_owned();
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
    let x = 5
    let y = 10
    let foobar = 939393
  ";
  let mut lexer = Lexer::new(input);
  let mut parser = Parser::new(&mut lexer);
  let program = parser.parse_program();

  assert!(program.statements.len() > 2, "failed parse correctly");

  let statement = program.statements;

  statement_assert(&statement[0], "let x = 5");
  statement_assert(&statement[1], "let y = 10");
  statement_assert(&statement[2], "let foobar = 939393");
}

#[test]
fn test_return_statements() {
  let input = "
    return 5
    return 10
    return 939393
  ";
  let mut lexer = Lexer::new(input);
  let mut parser = Parser::new(&mut lexer);
  let program = parser.parse_program();

  assert!(program.statements.len() > 2, "failed parse correctly");

  let statement = program.statements;

  statement_assert(&statement[0], "return 5");
  statement_assert(&statement[1], "return 10");
  statement_assert(&statement[2], "return 939393");
}

#[test]
fn test_operator_precedence_parsing() {
  let input = "
  -a * b
  !-a
  a + b + c
  a + b - c
  a * b * c
  a * b / c
  a + b / c
  a + b * c + d / e - f
  3 + 4 - 5 * 5
  5 > 4 == 3 < 4
  5 < 4 != 3 > 4
  3 + 4 * 5 == 3 * 1 + 4 * 5
";

  let mut lexer = Lexer::new(input);
  let mut parser = Parser::new(&mut lexer);
  let program = parser.parse_program();

  let statement = program.statements;

  statement_assert(&statement[0], "((-a) * b)");
  statement_assert(&statement[1], "(!(-a))");
  statement_assert(&statement[2], "((a + b) + c)");
  statement_assert(&statement[3], "((a + b) - c)");
  statement_assert(&statement[4], "((a * b) * c)");
  statement_assert(&statement[5], "((a * b) / c)");
  statement_assert(&statement[6], "(a + (b / c))");
  statement_assert(&statement[7], "(((a + (b * c)) + (d / e)) - f)");
  statement_assert(&statement[8], "((3 + 4) - (5 * 5))");
  statement_assert(&statement[9], "((5 > 4) == (3 < 4))");
  statement_assert(&statement[10], "((5 < 4) != (3 > 4))");
  statement_assert(&statement[11], "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))");
}

#[test]
fn test_boolean_parsing() {
  let input = "
  true
  false
  3 > 5 == false
  3 < 5 == true
";

  let mut lexer = Lexer::new(input);
  let mut parser = Parser::new(&mut lexer);
  let program = parser.parse_program();

  let statement = program.statements;

  statement_assert(&statement[0], "true");
  statement_assert(&statement[1], "false");
  statement_assert(&statement[2], "((3 > 5) == false)");
  statement_assert(&statement[3], "((3 < 5) == true)");
}

#[test]
fn test_funciton_parsing() {
  let input = "
  fn() {};
  fn(x) {};
  fn(x, y, z) {};
";

  let mut lexer = Lexer::new(input);
  let mut parser = Parser::new(&mut lexer);
  let program = parser.parse_program();

  let statement = program.statements;

  statement_assert(&statement[0], "fn() {}");
  statement_assert(&statement[1], "fn(x) {}");
  statement_assert(&statement[2], "fn(x, y, z) {}");
}

#[test]
fn test_call_parsing() {
  let input = "
  a + add(b * c) + d;
  add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8));
  add(a + b + c * d / f + g);
";

  let mut lexer = Lexer::new(input);
  let mut parser = Parser::new(&mut lexer);
  let program = parser.parse_program();

  let statement = program.statements;

  statement_assert(&statement[0], "((a + add((b * c))) + d)");
  statement_assert(&statement[1], "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))");
  statement_assert(&statement[2], "add((((a + b) + ((c * d) / f)) + g))");
}
