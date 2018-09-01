use std::fmt;
use lexer::*;

macro_rules! write_string {
  ($w:expr) => ( $w.to_string() );
}

#[derive(Debug, PartialEq, Clone)]
struct Node {}
impl Node {
  pub fn token_literal(&mut self) -> String {
    String::new()
  }
}

pub trait Statement {
  fn statement_node(&self) -> Node;
  fn token_literal(&self) -> String;
  fn emit_debug_info(&self) -> String;
}

#[derive(Clone)]
struct LetStatement {
  token: Token,
  value: Expression,
  name: Identifier,
}
impl Statement for LetStatement {
  fn statement_node(&self) -> Node {
    Node{}
  }

  fn token_literal(&self) -> String {
    write_string!(self.token.value)
  }

  fn emit_debug_info(&self) -> String {
    write_string!(format!("{:?} {:?} {:?}", self.token, self.value, self.name))
  }
}
#[derive(Clone)]
struct ReturnStatement {
  token: Token,
  return_value: Expression,
}
impl Statement for ReturnStatement {
  fn statement_node(&self) -> Node {
    Node{}
  }

  fn token_literal(&self) -> String {
    write_string!(self.token.value)
  }

  fn emit_debug_info(&self) -> String {
    write_string!(format!("{:?} {:?}", self.token, self.return_value))
  }
}

#[derive(Debug, Clone)]
pub struct Expression {
  node: Node
}
impl Expression {
  pub fn expression_node(&mut self) -> Node {
    self.node.clone()
  }
}

pub struct Program {
  pub statements: Vec<Box<Statement>>
}
impl Program {
  pub fn token_literal(&mut self) -> String {
     if self.statements.len() > 0 {
      self.statements[0].token_literal()
    } else {
      write_string!("")
    }
  }
}

impl fmt::Debug for Program {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let output: Vec<_> = self.statements.iter().map(|elem| { elem.emit_debug_info() }).collect();
    write!(f, "{:?}", output)
  }
}

#[derive(Debug, Clone)]
struct Identifier {
  pub token: Token,
  pub value: String,
}

impl Identifier {
  pub fn expression_node() {}
  pub fn token_literal(&mut self) -> String {
    write_string!(self.token.value)
  }
}

pub struct Parser<'a> {
  pub l: &'a  mut Lexer<'a>,
  pub cur_token: Option<Token>,
  pub peek_token: Option<Token>,
}

impl <'a>Parser<'a> {
  pub fn new(l: &'a mut Lexer<'a>) -> Parser<'a> {
    let current_token = l.next_token();
    let peek_token = l.next_token();

    Parser{
      l: l,
      cur_token: current_token,
      peek_token: peek_token,
    }
  }

  pub fn next_token(&mut self) {
    self.cur_token = {
      self.peek_token.clone()
    };
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
          None
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
            value: Expression{ node: Node{} },
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

      // TODO this implementation skip nodes until semicolon
      while self.cur_token_is(TokenType::TokenSemicolon) {
        self.next_token();
      }

      return Some(Box::new(stmt));
    }
    None
  }

  pub fn parse_return_statement(&mut self) -> Option<Box<Statement>> {
    let stmt = {
      match &self.cur_token {
        Some(token) => {
          ReturnStatement{
            token: token.clone(),
            return_value: Expression{ node: Node{} },
          }
        },
        None => {
          return None;
        }
      }
    };

    // TODO this implementation skip nodes until semicolon
    while self.cur_token_is(TokenType::TokenSemicolon) {
      self.next_token();
    }

    return Some(Box::new(stmt));
  }

  pub fn peek_error(&self, t: TokenType) {
    println!("expected next token to be {:?} instead", t);
  }

  pub fn cur_token_is(&self, t: TokenType) -> bool {
    if let Some(token) = &self.cur_token {
      return token.kind == t;
    }
    false
  }

  pub fn peek_token_is(&self, t: &TokenType) -> bool {
    if let Some(token) = &self.peek_token {
      return token.kind == *t;
    }
    false
  }

  pub fn expect_peek(&mut self, t: TokenType) -> bool {
    if self.peek_token_is(&t) {
      self.next_token();
      return true;
    } else {
      self.peek_error(t);
      return false;
    }
  }
}

#[test]
fn digit() {
  let mut lexer = Lexer::new("let abc = 456");
  let mut parser = Parser::new(&mut lexer);

  parser.parse_program();
}
