use std::fmt;
use lexer::*;

#[derive(Debug, Clone)]
struct Node {}
impl Node {
  pub fn token_literal(&mut self) -> String {
    "".to_string()
  }
}

pub trait Statement {
  fn statement_node(&mut self) -> Node {
    Node{}
  }

  fn token_literal(&self) -> String {
    String::new()
  }
}

#[derive(Debug)]
struct LetStatement {
  token: Token,
  value: Expression,
  name: Identifier,
}
impl Statement for LetStatement {
  fn token_literal(&self) -> String {
    self.token.value.to_string()
  }
}

#[derive(Debug)]
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
      "".to_string()
    }
  }
}

impl fmt::Debug for Program {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let output: Vec<_> = self.statements.iter().map(|elem| elem.token_literal() ).collect();
    write!(f, "{:?}", output)
  }
}

#[derive(Debug)]
struct Identifier {
  pub token: Token,
  pub value: String,
}

impl Identifier {
  pub fn expression_node() {}
  pub fn token_literal(&mut self) -> String {
    self.token.value.to_string()
  }
}

pub struct Parser<'a> {
  pub l: &'a  mut Lexer<'a>,
  pub cur_token: Option<Token>,
  pub peek_token: Option<Token>,
}

impl <'a>Parser<'a> {
  pub fn new(l: &'a mut Lexer<'a>) -> Parser<'a> {
    let peek_token = l.next_token();
    let current_token = l.next_token();

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
    // while self.cur_token.type != token.EOF {
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
            token: token.clone(),
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


    // let mut stmt = LetStatement{ token: self.cur_token.unwrap().clone() };
    if self.expect_peek(TokenType::TokenIdentifier) == false {
      return None;
    }
    
    if let Some(token) = &self.cur_token.clone() {
      let token_clone = token.clone();
      stmt.name = Identifier{
        token: token.clone(),
        value: token_clone.value,
      };

      // TODO: セミコロンに遭遇するまで式を読み飛ばしてしまっている for !p.curTokenIs(token.SEMICOLON) {
      if self.expect_peek(TokenType::TokenAssign) == false {
        self.next_token();
      }
      return Some(Box::new(stmt));
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
      return false;
    }
  }
}

#[test]
fn digit() {
  let mut lexer = Lexer::new("let abc = 456");
  let mut parser = Parser::new(&mut lexer);

  assert!(true, "nya-n");
}
