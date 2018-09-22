use lexer::lexer::*;
use lexer::token::*;

use parser::precedence::*;
use parser::statements_new::*;

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
    let mut program = Vec::new();

    while self.cur_token != None {
      if let Some(stmt) = self.parse_statement() {
        program.push(stmt);
      }
      self.next_token();
    }

    if self.errors.len() > 0 {
      self.emit_error();
    }

    program
  }

  pub fn parse_statement(&mut self) -> Option<Statement> {
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

  pub fn parse_let_statement(&mut self) -> Option<Statement> {
    if self.expect_peek(TokenType::Identifier) == false {
      return None;
    }
    
    if let Some(token) = self.cur_token.to_owned() {
      let name = Identifier(token.value.to_owned());

      if self.expect_peek(TokenType::Assign) == false {
        return None;
      }

      self.next_token();
      let value = if let Some(value) = self.parse_expression(Precedences::Lowest) {
        value
      } else {
        return None;
      };

      while self.peek_token_is(TokenType::Semicolon) {
        self.next_token();
      }

      return Some(Statement::Let(name, value));
    }
    None
  }

  pub fn parse_return_statement(&mut self) -> Option<Statement> {
    self.next_token();
    let return_value = if let Some(value) = self.parse_expression(Precedences::Lowest) {
      value
    } else {
      return None;
    };

    while self.peek_token_is(TokenType::Semicolon) {
      self.next_token();
    }

    return Some(Statement::Return(return_value));
  }

  pub fn parse_expression_statement(&mut self) -> Option<Statement> {
    let expression = if let Some(expression) = self.parse_expression(Precedences::Lowest) {
      expression
    } else {
      return None;
    };

    if self.peek_token_is(TokenType::Semicolon) {
      self.next_token();
    }

    return Some(Statement::Expression(expression));
  }

  pub fn parse_identifier(&self) -> Option<Expression> {
    if let Some(token) = &self.cur_token {
      return Some(Expression::Identifier(
        Identifier(token.value.to_owned())
      ));
    }
    None
  }

  pub fn parse_integer_literal(&mut self) -> Option<Expression> {
    if let Some(token) = &self.cur_token {
      if let Ok(value) = token.value.parse::<i64>() {
        return Some(Expression::IntegerLiteral(value));
      } else {
        self.errors.push(format!("could not parse {:?} as integer", token.value));
      }
    }
    None
  }

  pub fn parse_boolean(&mut self) -> Option<Expression> {
    return Some(Expression::Boolean(
      self.cur_token_is(TokenType::True))
    );
  }

  pub fn parse_expression(&mut self, precedence: Precedences) -> Option<Expression> {
    let mut left_exp: Option<Expression> = None;
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
          TokenType::Plus | TokenType::Minus | TokenType::Divide | TokenType::Multiply |
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

  pub fn parse_prefix_expression(&mut self) -> Option<Expression> {
    if let Some(token) = self.cur_token.to_owned() {
      self.next_token();
      if let Some(right) = self.parse_expression(Precedences::Prefix) {
        return Some(
          Expression::Prefix(
            self.convert_token_to_prefix(token.kind),
            Box::new(right),
          )
        );
      }
    }
    None
  }

  pub fn convert_token_to_prefix(&self, token: TokenType) -> Prefix {
    match token {
      TokenType::Plus => Prefix::Plus,
      TokenType::Minus => Prefix::Minus,
      TokenType::Bang => Prefix::Bang,
      _ => { panic!("nya-n"); }
    }
  }

  pub fn convert_token_to_infix(&self, token: TokenType) -> Infix {
    match token {
      TokenType::Plus => Infix::Plus,
      TokenType::Minus => Infix::Minus,
      TokenType::Divide => Infix::Divide,
      TokenType::Multiply => Infix::Multiply,
      TokenType::Eq => Infix::Eq,
      TokenType::NotEq => Infix::NotEq,
      TokenType::Gte => Infix::Gte,
      TokenType::Gt => Infix::Gt,
      TokenType::Lte => Infix::Lte,
      TokenType::Lt => Infix::Lt,
      _ => { panic!("nya-n"); }
    }
  }

  pub fn parse_infix_expression(&mut self, left: Option<Expression>) -> Option<Expression> {
    if left.is_none() {
      return None;
    }

    if let Some(token) = self.cur_token.to_owned() {
      let precedence = self.cur_precedence();
      self.next_token();
      if let Some(right) = self.parse_expression(precedence) {
        return Some(
          Expression::Infix(
            self.convert_token_to_infix(token.kind),
            Box::new(left.unwrap()),
            Box::new(right)
          )
        );
      }
    }
    None
  }

  pub fn parse_if_expression(&mut self) -> Option<Expression> {
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

        return Some(Expression::If{
          condition: Box::new(condition),
          consequence: consequence,
          alternative: alternative,
        });
      }
    }
    None
  }

  pub fn parse_grouped_expression(&mut self) -> Option<Expression> {
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

  pub fn parse_function_literal(&mut self) -> Option<Expression> {
    if self.expect_peek(TokenType::Lparen) == false {
      return None;
    }

    let parameters = self.parse_function_parameters();

    if self.expect_peek(TokenType::Lbrace) == false {
      return None;
    }

    if let Some(body) = self.parse_block_statement() {
      return Some(
        Expression::Function{
          parameters: parameters,
          body: body,
        }
      );
    }
    None
  }

  pub fn parse_call_expression(&mut self, function: Option<Expression>) -> Option<Expression> {
    if let Some(function) = function {
      Some(
        Expression::Call{
          function: Box::new(function),
          arguments: self.parse_call_arguments(),
        }
      )
    } else {
      None
    }
  }

  pub fn parse_call_arguments(&mut self) -> Vec<Expression> {
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
      parameters.push(
        Identifier(token.value.to_owned())
      );
    }
    
    while self.peek_token_is(TokenType::Comma) {
      self.next_token();
      self.next_token();

      if let Some(token) = self.cur_token.to_owned() {;
        parameters.push(
          Identifier(token.value.to_owned())
        );
      }
    }

    if self.expect_peek(TokenType::Rparen) == false {
      return Vec::new();
    }

    parameters
  }

  pub fn parse_block_statement(&mut self) -> Option<BlockStatement> {
    let mut block = Vec::new();
    self.next_token();

    while self.cur_token_is(TokenType::Rbrace) == false && self.cur_token.is_none() == false {
      if let Some(stmt) = self.parse_statement() {
        block.push(stmt);
      }
      self.next_token();
    }
    return Some(block);
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
